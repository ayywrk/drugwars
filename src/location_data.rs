use std::{
    collections::{HashMap, HashSet},
    ops::{Deref, DerefMut},
    sync::{Arc, RwLock},
};

use rand::{Rng, RngCore};

use crate::{
    element::MarketElement,
    resources::{Drug, Drugs, Item, Items, Location},
};

#[derive(Default)]
pub struct SingleLocationData {
    pub drug_market: HashMap<Arc<Drug>, MarketElement>,
    pub item_market: HashMap<Arc<Item>, MarketElement>,
    pub messages: Vec<String>,
    pub people: HashSet<String>,
    //pub price_mods: Vec<PriceMod>,
    //pub rumors: Vec<Rumor>,
}

impl SingleLocationData {
    pub fn update_markets(&mut self, drugs: &Drugs, items: &Items, rng: &mut dyn RngCore) {
        self.drug_market.clear();
        self.item_market.clear();

        for drug in drugs.values() {
            if rng.gen_bool(4. / 5.) {
                continue;
            };

            let supply = rng.gen_range(0..1000000);
            let demand = rng.gen_range(0..1000000);

            let price = drug.nominal_price.clone();

            self.drug_market.insert(
                drug.clone(),
                MarketElement {
                    supply,
                    demand,
                    price,
                },
            );
        }

        for item in items.values() {
            if rng.gen_bool(4. / 5.) {
                continue;
            };

            let supply = rng.gen_range(0..1000000);
            let demand = rng.gen_range(0..1000000);

            let price = item.nominal_price.clone();

            self.item_market.insert(
                item.clone(),
                MarketElement {
                    supply,
                    demand,
                    price,
                },
            );
        }
    }
}

#[derive(Default)]
pub struct LocationData(pub HashMap<Arc<Location>, Arc<RwLock<SingleLocationData>>>);
impl Deref for LocationData {
    type Target = HashMap<Arc<Location>, Arc<RwLock<SingleLocationData>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for LocationData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl LocationData {
    pub fn update_markets(&mut self, drugs: &Drugs, items: &Items, rng: &mut dyn RngCore) {
        for data in self.values_mut() {
            data.write().unwrap().update_markets(drugs, items, rng);
        }
    }
}
