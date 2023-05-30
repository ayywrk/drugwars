use std::collections::HashMap;

use num_bigint::{BigInt, ToBigInt};

use crate::config::DrugWarsConfig;

pub struct Position {
    pub lat: f32,
    pub long: f32,
}

pub struct Location {
    pub name: String,
    pub position: Position,
}

pub struct Drug {
    pub name: String,
    pub nominal_price: BigInt,
}

pub struct Item {
    pub name: String,
    pub nominal_price: BigInt,
    pub kind: ItemKind,
}

pub enum ItemKind {
    Weapon(Weapon),
    Ammo,
    Armor(Armor),
}

pub struct Armor {
    pub block: f32,
}

pub struct Weapon {
    pub ammo: Option<String>,
    pub damage: f32,
}

#[derive(Default)]
pub struct Drugs(pub HashMap<String, Drug>);
#[derive(Default)]
pub struct Items(pub HashMap<String, Item>);
#[derive(Default)]
pub struct Locations(pub HashMap<String, Location>);
#[derive(Default)]
pub struct Messages(pub HashMap<String, Vec<String>>);

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
        drugs.0.insert(
            name.to_owned(),
            Drug {
                name: name.to_owned(),
                nominal_price: price.to_bigint().unwrap(),
            },
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

        locations.0.insert(
            name.to_owned(),
            Location {
                name: name.to_owned(),
                position: Position { lat, long },
            },
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

        items.0.insert(
            name.to_owned(),
            Item {
                name: name.to_owned(),
                nominal_price: price.to_bigint().unwrap(),
                kind: ItemKind::Weapon(Weapon { ammo, damage }),
            },
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

        items.0.insert(
            name.to_owned(),
            Item {
                name: name.to_owned(),
                nominal_price: price.to_bigint().unwrap(),
                kind: ItemKind::Ammo,
            },
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

        items.0.insert(
            name.to_owned(),
            Item {
                name: name.to_owned(),
                nominal_price: price.to_bigint().unwrap(),
                kind: ItemKind::Armor(Armor { block }),
            },
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
        messages.0.insert(key.to_owned(), val);
    }

    (drugs, locations, items, messages)
}
