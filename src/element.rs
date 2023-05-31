use std::sync::Arc;

use num_bigint::BigInt;

use crate::resources::{Drug, Item, Location};

pub trait Element {
    fn name(&self) -> &str;
}

impl Element for Arc<Drug> {
    fn name(&self) -> &str {
        &self.name
    }
}
impl Element for Arc<Item> {
    fn name(&self) -> &str {
        &self.name
    }
}
impl Element for Arc<Location> {
    fn name(&self) -> &str {
        &self.name
    }
}

pub struct OwnedElement {
    pub amount: usize,
    pub bought_at: BigInt,
}

pub struct MarketElement {
    pub supply: usize,
    pub demand: usize,
    pub price: BigInt,
}
