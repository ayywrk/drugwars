use chrono::NaiveDate;

pub struct Settings {
    pub day_duration: u32,
    pub current_day: NaiveDate,
    pub save_path: String,
    pub config_path: String,
    pub width: usize,
}
