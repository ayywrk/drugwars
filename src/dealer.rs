use std::{
    collections::HashMap,
    ops::Deref,
    sync::{Arc, RwLock, RwLockReadGuard},
};

use chrono::NaiveDate;
use num_bigint::BigInt;

use crate::{
    element::OwnedElement,
    error::{Error, Result},
    resources::{Drug, Item, Location},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DealerStatus {
    Available,
    Flying,
    Dead(NaiveDate),
}

impl DealerStatus {
    pub fn pretty(&self) -> String {
        match self {
            DealerStatus::Available => "Available".to_owned(),
            DealerStatus::Flying => "Flying".to_owned(),
            DealerStatus::Dead(since) => format!("Dead since {}", since.format("%Y-%m-%d")),
        }
    }
}

#[derive(Default)]
pub struct Dealers(pub HashMap<String, Arc<RwLock<Dealer>>>);
impl Deref for Dealers {
    type Target = HashMap<String, Arc<RwLock<Dealer>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Dealers {
    pub fn get_dealer(&self, nick: &str) -> Result<RwLockReadGuard<Dealer>> {
        match self.get(nick) {
            Some(dealer) => Ok(dealer.read().unwrap()),
            None => Err(Error::DealerNotFound(nick.to_owned())),
        }
    }
}

pub struct Dealer {
    pub nick: String,
    pub has_attacked: bool,
    pub health: f32,
    pub money: BigInt,
    pub laundered_money: BigInt,
    pub location: Arc<Location>,
    pub capacity: usize,
    pub owned_drugs: HashMap<Arc<Location>, HashMap<Arc<Drug>, OwnedElement>>,
    pub owned_items: HashMap<Arc<Location>, HashMap<Arc<Item>, OwnedElement>>,
    pub status: DealerStatus,
    //pub looters: HashSet<String>,
}
