use chrono::{DateTime, Datelike, FixedOffset, NaiveDate, NaiveDateTime, TimeDelta, Utc};
use std::fs;

pub fn calculate_age(age: String) -> String {
    println!("Calculating age...");
    let now = Utc::now().naive_local();
    let naive_date = NaiveDate::parse_from_str(&age, "%Y-%m-%d").unwrap();
    let age: String = format!("{:?}",NaiveDate::years_since( &NaiveDate::from(now),naive_date));
    age
}

pub async fn get_time() -> String {
    let now = Utc::now();
    let current_date_time = now.format("%Y-%m-%d %H:%M").to_string();
    current_date_time
}

pub fn read_file(file_name:String) -> String {
    let data = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    data
}