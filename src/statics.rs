use std::collections::HashMap;

use num_bigint::BigInt;
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
