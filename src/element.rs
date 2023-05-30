use num_bigint::BigInt;

use crate::resources::{Drug, Item};

pub trait Element {}

impl Element for Drug {}
impl Element for Item {}

pub struct OwnedElement {
    pub amount: usize,
    pub bought_at: BigInt,
}

pub struct MarketElement {
    pub supply: usize,
    pub demand: usize,
    pub price: BigInt,
}