use std::{f32::consts::PI, path::Path, str};

use chrono::Duration;
use num_bigint::{BigInt, ToBigInt};
use tokio::{fs::File, io::AsyncReadExt};

use crate::{config::{DrugWarsConfig, Settings}, resources::Location};

pub async fn load_config(path: impl AsRef<Path>) -> std::io::Result<DrugWarsConfig> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(serde_yaml::from_str(&contents).unwrap())
}

pub fn get_flight_price(origin: &Location, other: &Location) -> BigInt {
    let cur_lat = origin.position.lat * (PI / 180.);
    let cur_long = origin.position.long * (PI / 180.);

    let other_lat = other.position.lat * (PI / 180.);
    let other_long = other.position.long * (PI / 180.);

    let float_price = (cur_lat.sin() * other_lat.sin()
        + cur_lat.cos() * other_lat.cos() * (other_long - cur_long).cos())
    .acos()
        * 10000.;

    (float_price * 10000.).to_bigint().unwrap()
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

pub trait StringManips {
    fn irc_safe_len(&self) -> usize;
    fn pretty_truncate(&self, max_width: usize) -> String;
}

impl<T: std::fmt::Display> StringManips for T {
    fn pretty_truncate(&self, max_width: usize) -> String {
        assert!(max_width > 3);

        if self.irc_safe_len() <= max_width {
            return self.to_string();
        }

        format!("{}...", &self.to_string()[..(max_width - 3)])
    }

    fn irc_safe_len(&self) -> usize {
        self.to_string()
            .replace("\x0300", "")
            .replace("\x0301", "")
            .replace("\x0302", "")
            .replace("\x0303", "")
            .replace("\x0304", "")
            .replace("\x0305", "")
            .replace("\x0306", "")
            .replace("\x0307", "")
            .replace("\x0308", "")
            .replace("\x0309", "")
            .replace("\x0310", "")
            .replace("\x0311", "")
            .replace("\x0312", "")
            .replace("\x0313", "")
            .replace("\x0314", "")
            .replace("\x0315", "")
            .chars()
            .filter(|c| !['\x02', '\x1d', '\x1f', '\x1e', '\x12', '\x0f'].contains(c))
            .count()
    }
}

pub fn get_date_and_time(settings: &Settings) -> String {
    let t = settings.timer.elapsed().unwrap().as_secs_f32() / settings.day_duration as f32;

    let current_seconds = t * 86400.;

    let duration = Duration::seconds(current_seconds as i64);

    let current_time = format!(
        "{:0>2}:{:0>2}",
        duration.num_hours(),
        duration.num_minutes() - (60 * duration.num_hours())
    );

    format!(
        "{} {}",
        settings.current_day.format("%Y-%m-%d").to_string(),
        current_time
    )
}
