pub mod config;
pub mod settings;
pub mod statics;
pub mod utils;

use std::error::Error;

use ircie::{
    format::{Color, Msg},
    system::IntoResponse,
    system_params::Res,
    Irc,
};
use statics::*;
use utils::{load_config, PrettyMoney};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    let drugwars_config = load_config("drugwars_config.yaml").await?;
    let (drugs, locations, items, messages) = get_statics_from_config(&drugwars_config);

    let mut irc = Irc::from_config("irc_config.yaml").await?;

    irc.add_resource(drugs)
        .await
        .add_resource(items)
        .await
        .add_resource(locations)
        .await
        .add_resource(messages)
        .await
        //.add_interval_task(Duration::from_secs(10), my_task)
        //.await
        .add_system("t", test)
        .await;

    irc.run().await?;

    Ok(())
}

fn test(drugs: Res<Drugs>) -> impl IntoResponse {
    let mut lines = vec![];

    for drug in drugs.0.iter() {
        lines.push(
            Msg::new()
                .text("name: ")
                .color(Color::Yellow)
                .text(drug.1.name.clone())
                .reset()
                .text(" price: ")
                .color(Color::Green)
                .text(drug.1.nominal_price.pretty_money()),
        )
    }
    lines
}
