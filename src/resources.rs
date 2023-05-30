use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use num_bigint::BigInt;
use rand::rngs::StdRng;
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
pub struct Drugs(HashMap<String, Arc<Drug>>);
impl Deref for Drugs {
    type Target = HashMap<String, Arc<Drug>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Drugs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[derive(Default)]
pub struct Items(HashMap<String, Arc<Item>>);
impl Deref for Items {
    type Target = HashMap<String, Arc<Item>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Items {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[derive(Default)]
pub struct Locations(HashMap<String, Arc<Location>>);
impl Deref for Locations {
    type Target = HashMap<String, Arc<Location>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Locations {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
#[derive(Default)]
pub struct Messages(HashMap<String, Vec<String>>);
impl Deref for Messages {
    type Target = HashMap<String, Vec<String>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Messages {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct DrugWarsRng(pub StdRng);
