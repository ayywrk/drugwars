use std::{
    collections::HashMap,
    hash::Hash,
    ops::{Deref, DerefMut},
    sync::Arc,
};

use itertools::Itertools;
use num_bigint::BigInt;
use rand::rngs::StdRng;

use crate::{
    element::Element,
    error::{Error, Result},
};

#[derive(Debug)]
pub struct Position {
    pub lat: f32,
    pub long: f32,
}

#[derive(Debug)]
pub struct Location {
    pub name: String,
    pub position: Position,
}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Location {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Hash for Location {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Debug)]
pub struct Drug {
    pub name: String,
    pub nominal_price: BigInt,
}

impl PartialEq for Drug {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Drug {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Hash for Drug {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub nominal_price: BigInt,
    pub kind: ItemKind,
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Item {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Hash for Item {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

#[derive(Debug)]
pub enum ItemKind {
    Weapon(Weapon),
    Ammo,
    Armor(Armor),
}

#[derive(Debug)]
pub struct Armor {
    pub block: f32,
}

#[derive(Debug)]
pub struct Weapon {
    pub ammo: Option<String>,
    pub damage: f32,
}

pub trait Matching {
    type Elem: Element;

    fn get_matching(&self, val: &str) -> Result<&Self::Elem>
    where
        Self: Deref<Target = Vec<Self::Elem>>,
    {
        let val = val.to_lowercase();
        let matching = self
            .iter()
            .filter(|elem| {
                elem.name()
                    .to_lowercase()
                    .replace(" ", "")
                    .starts_with(&val)
            })
            .sorted_by_key(|elem| elem.name().len())
            .collect::<Vec<_>>();

        if matching.len() == 0 {
            return Err(Error::ElementNotFound(val));
        } else if matching.len() > 1 {
            return Err(Error::ElementAmbiguous(val));
        }

        Ok(matching[0])
    }
}

#[derive(Default)]
pub struct Drugs(Vec<Arc<Drug>>);
impl Deref for Drugs {
    type Target = Vec<Arc<Drug>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Drugs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Matching for Drugs {
    type Elem = Arc<Drug>;
}

#[derive(Default)]
pub struct Items(Vec<Arc<Item>>);
impl Deref for Items {
    type Target = Vec<Arc<Item>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Items {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Matching for Items {
    type Elem = Arc<Item>;
}
#[derive(Default)]
pub struct Locations(Vec<Arc<Location>>);
impl Deref for Locations {
    type Target = Vec<Arc<Location>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Locations {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Matching for Locations {
    type Elem = Arc<Location>;
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

#[derive(Default)]
pub struct Flights(pub HashMap<String, Arc<Location>>);
impl Deref for Flights {
    type Target = HashMap<String, Arc<Location>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Flights {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
