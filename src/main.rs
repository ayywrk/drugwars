pub mod config;
pub mod dealer;
pub mod element;
pub mod error;
pub mod location_data;
pub mod render;
pub mod renderer;
pub mod resources;
pub mod utils;

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::SystemTime,
};

use chrono::Duration;
use config::{get_game_data_from_config, get_settings_from_config, Settings};
use dealer::{Dealer, DealerStatus, Dealers};
use element::Element;
use error::{Error, Result};
use ircie::{
    format::{Color, Msg},
    system::IntoResponse,
    system_params::{Arguments, Res, ResMut},
    Irc, IrcPrefix,
};
use location_data::{LocationData, SingleLocationData};
use num_bigint::ToBigInt;
use rand::{rngs::StdRng, seq::IteratorRandom, SeedableRng};
use render::{
    render_admin_help, render_help, render_info, render_market, render_people, render_prices_from,
};
use resources::{Drug, DrugWarsRng, Flights, GameData, Matching, Messages};
use utils::{get_date_and_time, load_config};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let drugwars_config = load_config("drugwars_config.yaml").await?;
    let game_data = get_game_data_from_config(&drugwars_config);

    let settings = get_settings_from_config(&drugwars_config, "drugwars_config.yaml");

    let mut rng = DrugWarsRng(StdRng::from_entropy());

    let mut loc_data = LocationData::default();
    for loc in game_data.locations.iter() {
        loc_data.0.insert(
            loc.clone(),
            Arc::new(RwLock::new(SingleLocationData::default())),
        );
    }
    loc_data.update(&game_data, &mut rng.0);

    let mut irc = Irc::from_config("irc_config.yaml").await?;

    // -- defaults
    irc.add_default_system(default_sys)
        .await
        .add_invalid_system(show_help)
        .await;

    // -- resources
    irc.add_resource(game_data)
        .await
        .add_resource(settings)
        .await
        .add_resource(Dealers::default())
        .await
        .add_resource(rng)
        .await
        .add_resource(loc_data)
        .await
        .add_resource(Flights::default())
        .await;

    // -- intervals
    irc.add_interval_task(std::time::Duration::from_millis(50), new_day)
        .await;

    // -- systems
    irc.add_system("melp?", melp)
        .await
        .add_system("register", register)
        .await
        .add_system("i", dealer_info)
        .await
        .add_system("h", show_help)
        .await
        .add_system("m", show_market)
        .await
        .add_system("p", show_people)
        .await
        .add_system("cf", check_flight_prices)
        .await
        .add_system("f", fly_to)
        .await
        .add_system("t", show_time)
        .await
        .add_system("ha", show_admin_help)
        .await
        .add_system("bd", buy::<Drug>)
        .await;

    irc.run().await?;

    Ok(())
}

fn new_day(
    mut settings: ResMut<Settings>,
    mut loc_data: ResMut<LocationData>,
    game_data: Res<GameData>,
    mut flights: ResMut<Flights>,
    dealers: Res<Dealers>,
    mut rng: ResMut<DrugWarsRng>,
) -> std::result::Result<impl IntoResponse, ()> {
    if settings.timer.elapsed().unwrap().as_secs() < settings.day_duration as u64 {
        return Err(());
    }

    settings.current_day += Duration::days(1);
    settings.timer = SystemTime::now();

    loc_data.update(&game_data, &mut rng.0);

    let mut lines = vec![Msg::new()
        .text("new day: ")
        .color(Color::Green)
        .text(settings.current_day.format("%Y-%m-%d").to_string())];

    for (nick, destination) in flights.iter() {
        let mut dealer = dealers.get_dealer_mut(nick).unwrap();

        let msg = Msg::new()
            .text(format!("{}: ", dealer.nick))
            .color(Color::Green)
            .text("you landed at ")
            .color(Color::Purple)
            .text(&destination.name);

        lines.push(msg);
        dealer.location = destination.clone();
        dealer.status = DealerStatus::Available;

        let data = loc_data.get_mut(destination).unwrap();
        data.write().unwrap().people.insert(dealer.nick.clone());
    }

    flights.clear();

    Ok((false, lines))
}

fn default_sys() -> impl IntoResponse {
    "melp?"
}

fn register(
    prefix: IrcPrefix,
    mut dealers: ResMut<Dealers>,
    game_data: Res<GameData>,
    mut rng: ResMut<DrugWarsRng>,
) -> impl IntoResponse {
    if dealers.0.contains_key(prefix.nick) {
        return Err(Error::AlreadyRegistered);
    }

    let mut owned_drugs = HashMap::default();
    let mut owned_items = HashMap::default();

    let location = game_data
        .locations
        .iter()
        .choose(&mut rng.0)
        .unwrap()
        .clone();

    for loc in game_data.locations.iter() {
        owned_drugs.insert(loc.clone(), HashMap::default());
        owned_items.insert(loc.clone(), HashMap::default());
    }

    dealers.0.insert(
        prefix.nick.to_owned(),
        Arc::new(RwLock::new(Dealer {
            nick: prefix.nick.to_owned(),
            has_attacked: false,
            health: 100.,
            money: 1000000000000u64.to_bigint().unwrap(),
            laundered_money: 0.to_bigint().unwrap(),
            location,
            capacity: 10,
            owned_drugs,
            owned_items,
            status: DealerStatus::Available,
        })),
    );

    Ok(Msg::new().text(prefix.nick).text("Get Rich or Die Tryin"))
}

fn dealer_info(prefix: IrcPrefix, dealers: Res<Dealers>) -> Result<impl IntoResponse> {
    let dealer = dealers.get_dealer(prefix.nick)?;
    Ok((false, render_info(&dealer)))
}

fn melp() -> impl IntoResponse {
    (false, Msg::new().text("explodes.").as_action())
}

fn show_help() -> impl IntoResponse {
    (false, render_help())
}

fn show_admin_help() -> impl IntoResponse {
    (false, render_admin_help())
}

fn show_market(
    prefix: IrcPrefix,
    settings: Res<Settings>,
    mut rng: ResMut<DrugWarsRng>,
    dealers: Res<Dealers>,
    location_data: Res<LocationData>,
    messages: Res<Messages>,
) -> Result<impl IntoResponse> {
    let dealer = dealers.get_dealer(prefix.nick)?;
    let loc_data = location_data.get(&dealer.location).unwrap();

    Ok((
        false,
        render_market(
            settings.width,
            &mut rng.0,
            prefix.nick,
            &dealer,
            &loc_data.read().unwrap(),
            &messages,
        ),
    ))
}

fn show_people(
    prefix: IrcPrefix,
    settings: Res<Settings>,
    dealers: Res<Dealers>,
    location_data: Res<LocationData>,
) -> Result<impl IntoResponse> {
    let dealer = dealers.get_dealer(prefix.nick)?;
    let loc_data = location_data.get(&dealer.location).unwrap();

    Ok((
        false,
        render_people(settings.width, &loc_data.read().unwrap()),
    ))
}

fn check_flight_prices(
    prefix: IrcPrefix,
    dealers: Res<Dealers>,
    game_data: Res<GameData>,
) -> Result<impl IntoResponse> {
    let dealer = dealers.get_dealer(prefix.nick)?;

    Ok((
        false,
        render_prices_from(&dealer.location, &game_data.locations),
    ))
}

fn fly_to(
    prefix: IrcPrefix,
    arguments: Arguments<'_, 1>,
    dealers: Res<Dealers>,
    game_data: Res<GameData>,
    mut flights: ResMut<Flights>,
    location_data: Res<LocationData>,
) -> Result<impl IntoResponse> {
    let mut dealer = dealers.get_dealer_available_mut(prefix.nick)?;
    let destination = game_data.locations.get_matching(arguments[0])?;

    let current_location_data = location_data.get(&dealer.location).unwrap();

    dealer.fly_to(
        &mut flights,
        destination,
        &mut current_location_data.write().unwrap(),
    )
}

fn show_time(settings: Res<Settings>) -> impl IntoResponse {
    get_date_and_time(&settings)
}

fn buy<E: Element + 'static>(
    prefix: IrcPrefix,
    arguments: Arguments<'_, 2>,
    dealers: Res<Dealers>,
    game_data: Res<GameData>,
    location_data: Res<LocationData>
) -> Result<impl IntoResponse> {
    let dealer = dealers.get_dealer_available(prefix.nick)?;

    let elem = game_data.get_matching::<E>(arguments[0])?;
    let loc_data = location_data.get(&dealer.location).unwrap();

    let market_elem = loc_data.read().unwrap().get_market_element::<E>(elem)?;

    let amount = arguments[1].parse::<usize>()?;

    Ok(format!("Buying {} {}", amount, drug.name()))
}
