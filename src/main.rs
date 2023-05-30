pub mod config;
pub mod dealer;
pub mod statics;
pub mod utils;

use std::error::Error;

use chrono::Duration;
use config::{get_settings_from_config, get_statics_from_config, Settings};
use dealer::{Dealer, DealerStatus, Dealers};
use ircie::{
    format::{Color, Msg},
    system::IntoResponse,
    system_params::ResMut,
    Irc, IrcPrefix,
};
use num_bigint::ToBigInt;
use utils::load_config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
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
        .add_interval_task(std::time::Duration::from_secs(dur), new_day)
        .await
        .add_system("register", register)
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

fn register(prefix: IrcPrefix, mut dealers: ResMut<Dealers>) -> impl IntoResponse {
    if dealers.0.contains_key(prefix.nick) {
        return Msg::new()
            .text(prefix.nick)
            .text(": You already registerd.");
    }

    dealers.0.insert(
        prefix.nick.to_owned(),
        Dealer {
            nick: prefix.nick.to_owned(),
            has_attacked: false,
            health: 100.,
            money: 10000000.to_bigint().unwrap(),
            laundered_money: 0.to_bigint().unwrap(),
            location: "aa".to_owned(),
            capacity: 10,
            status: DealerStatus::Available,
        },
    );

    Msg::new().text(prefix.nick).text(": Get Rich or Die Tryin")
}
