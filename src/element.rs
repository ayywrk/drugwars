use std::{hash::Hash, sync::Arc};

use num_bigint::BigInt;

use crate::resources::{Drug, Item, Location};

pub trait Element: Eq + Hash + 'static {
    fn name(&self) -> &str;
}

impl Element for Drug {
    fn name(&self) -> &str {
        &self.name
    }
}
impl Element for Item {
    fn name(&self) -> &str {
        &self.name
    }
}
impl Element for Location {
    fn name(&self) -> &str {
        &self.name
    }
}

pub trait ArcElement: Eq + Hash + 'static {
    fn name(&self) -> &str;
}

impl<E: Element> ArcElement for Arc<E> {
    fn name(&self) -> &str {
        self.as_ref().name()
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
