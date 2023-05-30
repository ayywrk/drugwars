use std::{path::Path, str::FromStr, sync::Arc};

use chrono::NaiveDate;
use num_bigint::ToBigInt;
use serde::Deserialize;
use serde_yaml::{Mapping, Sequence};

use crate::resources::*;

#[derive(Deserialize)]
pub struct DrugWarsConfig {
    pub settings: Mapping,
    pub locations: Sequence,
    pub drugs: Sequence,
    pub items: Mapping,
    pub messages: Mapping,
}

pub struct Settings {
    pub day_duration: u32,
    pub current_day: NaiveDate,
    pub save_path: String,
    pub config_path: String,
    pub width: usize,
}

pub fn get_statics_from_config(
    drugwars_config: &DrugWarsConfig,
) -> (Drugs, Locations, Items, Messages) {
    let mut drugs = Drugs::default();
    let mut locations = Locations::default();
    let mut items = Items::default();
    let mut messages = Messages::default();

    for drug in &drugwars_config.drugs {
        let name = drug.as_mapping().unwrap()["name"].as_str().unwrap();
        let price = drug.as_mapping().unwrap()["price"].as_f64().unwrap() * 10000.;
        drugs.insert(
            name.to_owned(),
            Arc::new(Drug {
                name: name.to_owned(),
                nominal_price: price.to_bigint().unwrap(),
            }),
        );
    }

    for location in &drugwars_config.locations {
        let name = location.as_mapping().unwrap()["name"].as_str().unwrap();
        let lat = location.as_mapping().unwrap()["position"]
            .as_mapping()
            .unwrap()["lat"]
            .as_f64()
            .unwrap() as f32;
        let long = location.as_mapping().unwrap()["position"]
            .as_mapping()
            .unwrap()["long"]
            .as_f64()
            .unwrap() as f32;

        locations.insert(
            name.to_owned(),
            Arc::new(Location {
                name: name.to_owned(),
                position: Position { lat, long },
            }),
        );
    }

    let weapons = drugwars_config.items["weapons"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|value| value.as_mapping().unwrap())
        .collect::<Vec<_>>();

    for weapon in weapons {
        let name = weapon["name"].as_str().unwrap();
        let price = weapon["price"].as_f64().unwrap() * 10000.;
        let damage = weapon["damage"].as_f64().unwrap() as f32;

        let mut ammo = None;

        if weapon.contains_key("ammo") {
            ammo = Some(weapon["ammo"].as_str().unwrap().to_owned())
        }

        items.insert(
            name.to_owned(),
            Arc::new(Item {
                name: name.to_owned(),
                nominal_price: price.to_bigint().unwrap(),
                kind: ItemKind::Weapon(Weapon { ammo, damage }),
            }),
        );
    }

    let ammos = drugwars_config.items["ammos"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|value| value.as_mapping().unwrap())
        .collect::<Vec<_>>();

    for ammo in ammos {
        let name = ammo["name"].as_str().unwrap();
        let price = ammo["price"].as_f64().unwrap() * 10000.;

        items.insert(
            name.to_owned(),
            Arc::new(Item {
                name: name.to_owned(),
                nominal_price: price.to_bigint().unwrap(),
                kind: ItemKind::Ammo,
            }),
        );
    }

    let armors = drugwars_config.items["armors"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|value| value.as_mapping().unwrap())
        .collect::<Vec<_>>();

    for armor in armors {
        let name = armor["name"].as_str().unwrap();
        let price = armor["price"].as_f64().unwrap() * 10000.;
        let block = armor["block"].as_f64().unwrap() as f32;

        items.insert(
            name.to_owned(),
            Arc::new(Item {
                name: name.to_owned(),
                nominal_price: price.to_bigint().unwrap(),
                kind: ItemKind::Armor(Armor { block }),
            }),
        );
    }

    for (message_key, message_val) in &drugwars_config.messages {
        let key = message_key.as_str().unwrap();
        let val = message_val
            .as_sequence()
            .unwrap()
            .into_iter()
            .map(|v| v.as_str().unwrap().to_owned())
            .collect::<Vec<_>>();
        messages.insert(key.to_owned(), val);
    }

    (drugs, locations, items, messages)
}

pub fn get_settings_from_config(
    drugwars_config: &DrugWarsConfig,
    config_path: impl AsRef<Path>,
) -> Settings {
    let day_duration = drugwars_config.settings["day_duration"].as_u64().unwrap() as u32;
    let current_day_str = drugwars_config.settings["start_day"].as_str().unwrap();
    let save_path = drugwars_config.settings["save_path"].as_str().unwrap();
    let width = drugwars_config.settings["width"].as_u64().unwrap();

    Settings {
        day_duration,
        current_day: NaiveDate::from_str(current_day_str).unwrap(),
        save_path: save_path.to_owned(),
        config_path: config_path.as_ref().to_str().unwrap().to_string(),
        width: width as usize,
    }
}
