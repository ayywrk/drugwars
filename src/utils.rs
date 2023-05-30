use std::{path::Path, str};

use num_bigint::BigInt;
use tokio::{fs::File, io::AsyncReadExt};

use crate::config::DrugWarsConfig;

pub async fn load_config(path: impl AsRef<Path>) -> std::io::Result<DrugWarsConfig> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(serde_yaml::from_str(&contents).unwrap())
}

pub trait PrettyMoney {
    fn pretty_money(&self) -> String;
}

pub trait PrettyAmount {
    fn pretty_amount(&self) -> String;
}

impl PrettyMoney for BigInt {
    fn pretty_money(&self) -> String {
        let unit: BigInt = self.clone() / 10000;
        let dec: BigInt = self % 10000;

        let pretty_money = unit
            .to_string()
            .as_bytes()
            .rchunks(3)
            .rev()
            .map(str::from_utf8)
            .collect::<std::result::Result<Vec<&str>, _>>()
            .unwrap()
            .join(",");

        let dec_str = format!("{:0>2}", dec);
        format!("${}.{:0>2}", pretty_money, &dec_str[..2])
    }
}

impl PrettyAmount for usize {
    fn pretty_amount(&self) -> String {
        self.to_string()
            .as_bytes()
            .rchunks(3)
            .rev()
            .map(str::from_utf8)
            .collect::<std::result::Result<Vec<&str>, _>>()
            .unwrap()
            .join(",")
    }
}
