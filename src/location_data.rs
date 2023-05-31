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

#[derive(Debug, Clone)]
pub enum PriceTrend {
    Up,
    Down,
}

#[derive(Debug, Clone)]
pub enum PriceModKind {
    Rumor,
    Spontaneous,
}

#[derive(Debug, Clone)]
pub struct PriceMod {
    pub drug: Arc<Drug>,
    pub trend: PriceTrend,
    pub kind: PriceModKind,
}

#[derive(Debug, Clone)]
pub struct Rumor {
    pub drug: Arc<Drug>,
    pub trend: PriceTrend,
    pub location: String,
    pub confirmed: Option<bool>,
}
#[derive(Default)]
pub struct SingleLocationData {
    pub drug_market: HashMap<Arc<Drug>, MarketElement>,
    pub item_market: HashMap<Arc<Item>, MarketElement>,
    pub messages: Vec<String>,
    pub people: HashSet<String>,
    pub price_mods: Vec<PriceMod>,
    //pub rumors: Vec<Rumor>,
}

impl SingleLocationData {
    pub fn update_markets(&mut self, drugs: &Drugs, items: &Items, rng: &mut dyn RngCore) {
        self.drug_market.clear();
        self.item_market.clear();

        for drug in drugs.values() {
            let mods = self
                    .price_mods
                    .clone()
                    .into_iter()
                    .filter(|price_mod| price_mod.drug.as_ref() == drug.as_ref())
                    .collect::<Vec<_>>();

            if rng.gen_bool(4. / 5.) && mods.len() == 0 {
                continue;
            };

            let supply = rng.gen_range(0..1000000);
            let demand = rng.gen_range(0..1000000);

            let mut price = drug.nominal_price.clone();

            for price_mod in mods {
                match price_mod.trend {
                    PriceTrend::Up => price *= 15,
                    PriceTrend::Down => price /= 6,
                }
            }

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

    pub fn update_price_mods(&mut self, drugs: &Drugs, rng: &mut dyn RngCore) {
        self.price_mods.clear();

        for drug in drugs.values() {
            if rng.gen_bool(0.92) {
                continue;
            }

            match rng.gen_bool(1. / 2.) {
                // Price down
                true => self.price_mods.push(PriceMod {
                    drug: drug.clone(),
                    trend: PriceTrend::Down,
                    kind: PriceModKind::Spontaneous,
                }),
                // Price UP !
                false => self.price_mods.push(PriceMod {
                    drug: drug.clone(),
                    trend: PriceTrend::Up,
                    kind: PriceModKind::Spontaneous,
                }),
            }
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
    pub fn init(&mut self, drugs: &Drugs, items: &Items, rng: &mut dyn RngCore) {
        for data_arc in self.values_mut() {
            let mut data = data_arc.write().unwrap();

            data.update_price_mods(drugs, rng);
            data.update_markets(drugs, items, rng);
        }
    }
}
