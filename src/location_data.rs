use std::{
    any::TypeId,
    borrow::Borrow,
    collections::{HashMap, HashSet},
    hash::Hash,
    ops::{Deref, DerefMut},
    sync::{Arc, RwLock},
};

use rand::{seq::IteratorRandom, Rng, RngCore};

use crate::{
    element::{ArcElement, Element, MarketElement},
    error::{Error, Result},
    resources::{Drug, Drugs, GameData, Item, Items, Location, Locations},
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
    pub location: Arc<Location>,
    pub confirmed: Option<bool>,
}
#[derive(Default)]
pub struct SingleLocationData {
    pub drug_market: HashMap<Arc<Drug>, MarketElement>,
    pub item_market: HashMap<Arc<Item>, MarketElement>,
    pub messages: Vec<String>,
    pub people: HashSet<String>,
    pub price_mods: Vec<PriceMod>,
    pub rumors: Vec<Rumor>,
}

impl SingleLocationData {
    pub fn get_market_element<E: ArcElement>(&self, elem: E) -> Result<&MarketElement> {
        if TypeId::of::<E>() == TypeId::of::<Arc<Drug>>() {
            self.drug_market
                .get(&elem)
                .ok_or(Error::ElementNotFound(elem.name().to_owned()))
        } else {
            self.item_market
                .get(&elem)
                .ok_or(Error::ElementNotFound(elem.name().to_owned()))
        }
    }

    pub fn update_markets(&mut self, drugs: &Drugs, items: &Items, rng: &mut dyn RngCore) {
        self.drug_market.clear();
        self.item_market.clear();

        for drug in drugs.iter() {
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

        for item in items.iter() {
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

        for drug in drugs.iter() {
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

    pub fn generate_rumors(
        &mut self,
        drugs: &Drugs,
        locations: &Locations,
        mut rng: &mut dyn RngCore,
    ) {
        self.rumors.clear();

        for drug in drugs.iter() {
            if rng.gen_bool(0.95) {
                continue;
            }

            match rng.gen_bool(1. / 2.) {
                // Price down
                true => self.rumors.push(Rumor {
                    drug: drug.clone(),
                    trend: PriceTrend::Down,
                    location: locations.iter().choose(&mut rng).unwrap().clone(),
                    confirmed: None,
                }),
                // Price UP !
                false => self.rumors.push(Rumor {
                    drug: drug.clone(),
                    trend: PriceTrend::Up,
                    location: locations.iter().choose(&mut rng).unwrap().clone(),
                    confirmed: None,
                }),
            }
        }
    }

    pub fn confirm_rumors(&mut self, rng: &mut dyn RngCore) {
        self.rumors.retain(|rumor| rumor.confirmed.is_none());

        for rumor in &mut self.rumors {
            if rng.gen_bool(1. / 2.) {
                rumor.confirmed = Some(false);
                continue;
            }

            rumor.confirmed = Some(true);

            self.price_mods.push(PriceMod {
                drug: rumor.drug.clone(),
                trend: rumor.trend.clone(),
                kind: PriceModKind::Rumor,
            });
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
    pub fn update(&mut self, game_data: &GameData, rng: &mut dyn RngCore) {
        for data_arc in self.values_mut() {
            let mut data = data_arc.write().unwrap();

            data.update_price_mods(&game_data.drugs, rng);
            data.confirm_rumors(rng);
            data.update_markets(&game_data.drugs, &game_data.items, rng);
            data.generate_rumors(&game_data.drugs, &game_data.locations, rng)
        }
    }
}
