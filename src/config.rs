use serde::Deserialize;
use serde_yaml::{Mapping, Sequence};

#[derive(Deserialize)]
pub struct DrugWarsConfig {
    pub settings: Mapping,
    pub locations: Sequence,
    pub drugs: Sequence,
    pub items: Mapping,
    pub messages: Mapping,
}
