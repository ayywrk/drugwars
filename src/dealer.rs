use std::collections::HashMap;

use chrono::NaiveDate;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DealerStatus {
    Available,
    Flying,
    Dead(NaiveDate),
}

#[derive(Default)]
pub struct Dealers(pub HashMap<String, Dealer>);

pub struct Dealer {
    pub nick: String,
    pub has_attacked: bool,
    pub health: f32,
    pub money: BigInt,
    pub laundered_money: BigInt,
    pub location: String,
    pub capacity: usize,
    //pub owned_drugs: HashMap<String, HashMap<String, OwnedElement>>,
    //pub owned_items: HashMap<String, HashMap<String, OwnedElement>>,
    pub status: DealerStatus,
    //pub looters: HashSet<String>,
}
