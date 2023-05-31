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
};

use chrono::Duration;
use config::{get_settings_from_config, get_statics_from_config, Settings};
use dealer::{Dealer, DealerStatus, Dealers};
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
use resources::{DrugWarsRng, Locations};
use utils::load_config;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let drugwars_config = load_config("drugwars_config.yaml").await?;
    let (drugs, locations, items, messages) = get_statics_from_config(&drugwars_config);

    let settings = get_settings_from_config(&drugwars_config, "drugwars_config.yaml");
    let dur = settings.day_duration as u64;

    let mut rng = DrugWarsRng(StdRng::from_entropy());

    let mut loc_data = LocationData::default();
    for loc in locations.values() {
        loc_data.0.insert(
            loc.clone(),
            Arc::new(RwLock::new(SingleLocationData::default())),
        );
    }
    loc_data.update_markets(&drugs, &items, &mut rng.0);

    let mut irc = Irc::from_config("irc_config.yaml").await?;

    // -- resources
    irc.add_resource(drugs)
        .await
        .add_resource(items)
        .await
        .add_resource(locations)
        .await
        .add_resource(messages)
        .await
        .add_resource(settings)
        .await
        .add_resource(Dealers::default())
        .await
        .add_resource(rng)
        .await
        .add_resource(loc_data)
        .await;

    // -- intervals
    irc.add_interval_task(std::time::Duration::from_secs(dur), new_day)
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
        .add_system("ha", show_admin_help)
        .await
        .add_system("test", test_args)
        .await;

    irc.run().await?;

    Ok(())
}

fn new_day(mut settings: ResMut<Settings>) -> impl IntoResponse {
    settings.current_day += Duration::days(1);

    Msg::new()
        .text("new day: ")
        .color(Color::Green)
        .text(settings.current_day.format("%Y-%m-%d").to_string())
}

fn test_args(arguments: Arguments) -> impl IntoResponse {
    format!("yo {}", arguments[0])
}

fn register(
    prefix: IrcPrefix,
    mut dealers: ResMut<Dealers>,
    locations: Res<Locations>,
    mut rng: ResMut<DrugWarsRng>,
) -> impl IntoResponse {
    if dealers.0.contains_key(prefix.nick) {
        return Err(Error::AlreadyRegistered(prefix.nick.to_owned()));
    }

    let mut owned_drugs = HashMap::default();
    let mut owned_items = HashMap::default();

    let location = locations.values().choose(&mut rng.0).unwrap().clone();

    for loc in locations.values() {
        owned_drugs.insert(loc.clone(), HashMap::default());
        owned_items.insert(loc.clone(), HashMap::default());
    }

    dealers.0.insert(
        prefix.nick.to_owned(),
        Arc::new(RwLock::new(Dealer {
            nick: prefix.nick.to_owned(),
            has_attacked: false,
            health: 100.,
            money: 10000000.to_bigint().unwrap(),
            laundered_money: 0.to_bigint().unwrap(),
            location,
            capacity: 10,
            owned_drugs,
            owned_items,
            status: DealerStatus::Available,
        })),
    );

    Ok(Msg::new().text(prefix.nick).text(": Get Rich or Die Tryin"))
}

fn dealer_info(prefix: IrcPrefix, dealers: Res<Dealers>) -> Result<Vec<String>> {
    let dealer = dealers.get_dealer(prefix.nick)?;
    Ok(render_info(&dealer))
}

fn melp() -> impl IntoResponse {
    Msg::new().text("explodes.").as_action()
}

fn show_help() -> impl IntoResponse {
    render_help()
}

fn show_admin_help() -> impl IntoResponse {
    render_admin_help()
}

fn show_market(
    prefix: IrcPrefix,
    settings: Res<Settings>,
    mut rng: ResMut<DrugWarsRng>,
    dealers: Res<Dealers>,
    location_data: Res<LocationData>,
) -> Result<Vec<String>> {
    let dealer = dealers.get_dealer(prefix.nick)?;

    let loc_data = location_data.get(&dealer.location).unwrap();

    Ok(render_market(
        settings.width,
        &mut rng.0,
        prefix.nick,
        &dealer,
        &loc_data.read().unwrap(),
    ))
}

fn show_people(
    prefix: IrcPrefix,
    settings: Res<Settings>,
    dealers: Res<Dealers>,
    location_data: Res<LocationData>,
) -> Result<Vec<String>> {
    let dealer = dealers.get_dealer(prefix.nick)?;
    let loc_data = location_data.get(&dealer.location).unwrap();

    Ok(render_people(settings.width, &loc_data.read().unwrap()))
}

fn check_flight_prices(
    prefix: IrcPrefix,
    dealers: Res<Dealers>,
    locations: Res<Locations>,
) -> Result<Vec<String>> {
    let dealer = dealers.get_dealer(prefix.nick)?;

    Ok(render_prices_from(&dealer.location, &locations))
}
