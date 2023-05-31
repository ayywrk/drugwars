use std::{
    collections::HashMap,
    ops::Deref,
    sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use chrono::NaiveDate;
use ircie::format::{Color, Msg};
use num_bigint::BigInt;

use crate::{
    element::OwnedElement,
    error::{Error, Result},
    location_data::SingleLocationData,
    resources::{Drug, Flights, Item, Location},
    utils::{get_flight_price, PrettyMoney},
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

    pub fn description(&self) -> String {
        match self {
            DealerStatus::Available => "".to_owned(),
            DealerStatus::Flying => "can't do business while flying".to_owned(),
            DealerStatus::Dead(_) => "can't do business while dead".to_owned(),
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

    pub fn get_dealer_available(&self, nick: &str) -> Result<RwLockReadGuard<Dealer>> {
        let dealer = self.get_dealer(nick)?;

        if !dealer.available() {
            return Err(Error::DealerNotAvailable(
                nick.to_owned(),
                dealer.status.description(),
            ));
        }

        Ok(dealer)
    }

    pub fn get_dealer_mut(&self, nick: &str) -> Result<RwLockWriteGuard<Dealer>> {
        match self.get(nick) {
            Some(dealer) => Ok(dealer.write().unwrap()),
            None => Err(Error::DealerNotFound(nick.to_owned())),
        }
    }

    pub fn get_dealer_available_mut(&self, nick: &str) -> Result<RwLockWriteGuard<Dealer>> {
        let dealer = self.get_dealer_mut(nick)?;

        if !dealer.available() {
            return Err(Error::DealerNotAvailable(
                nick.to_owned(),
                dealer.status.description(),
            ));
        }

        Ok(dealer)
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

impl Dealer {
    pub fn available(&self) -> bool {
        self.status == DealerStatus::Available
    }

    pub fn fly_to(
        &mut self,
        flights: &mut Flights,
        destination: &Arc<Location>,
        current_location_data: &mut SingleLocationData,
    ) -> Result<Vec<String>> {
        let price = get_flight_price(&self.location, &destination);

        if self.money < price {
            return Err(Error::NotEnoughMoney);
        }

        self.status = DealerStatus::Flying;
        current_location_data.people.remove(&self.nick);
        self.money -= price.clone();

        flights.insert(self.nick.clone(), destination.clone());

        Ok(vec![Msg::new()
            .text("you took a flight to ")
            .color(Color::Purple)
            .text(&destination.name)
            .reset()
            .text(" for ")
            .color(Color::Green)
            .text(price.pretty_money())
            .reset()
            .text(". You'll arrive tomorrow")
            .to_string()])
    }
}
