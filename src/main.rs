pub mod config;
pub mod dealer;
pub mod render;
pub mod renderer;
pub mod resources;
pub mod utils;
pub mod error;

use std::{
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
use num_bigint::ToBigInt;
use rand::{rngs::StdRng, seq::IteratorRandom, SeedableRng};
use render::{render_admin_help, render_help, render_info};
use resources::{DrugWarsRng, Locations};
use utils::load_config;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let drugwars_config = load_config("drugwars_config.yaml").await?;
    let (drugs, locations, items, messages) = get_statics_from_config(&drugwars_config);

    let settings = get_settings_from_config(&drugwars_config, "drugwars_config.yaml");
    let dur = settings.day_duration as u64;

    let mut irc = Irc::from_config("irc_config.yaml").await?;

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
        .add_resource(DrugWarsRng(StdRng::from_entropy()))
        .await
        .add_interval_task(std::time::Duration::from_secs(dur), new_day)
        .await
        .add_system("melp?", melp)
        .await
        .add_system("register", register)
        .await
        .add_system("i", dealer_info)
        .await
        .add_system("h", show_help)
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
        .text("new day! ")
        .color(Color::Cyan)
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

    let location = locations.values().choose(&mut rng.0).unwrap().clone();

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
    "\x01ACTION explodes.\x01"
}

fn show_help() -> impl IntoResponse {
    render_help()
}

fn show_admin_help() -> impl IntoResponse {
    render_admin_help()
}
