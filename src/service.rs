use chrono::{NaiveDate, Utc};
use std::fs;

pub fn calculate_age(age: String) -> String {
    println!("Calculating age...");
    let now = Utc::now().naive_local();
    let naive_date = NaiveDate::parse_from_str(&age, "%Y-%m-%d").unwrap();
    let age: String = format!("{:?}",NaiveDate::years_since( &NaiveDate::from(now),naive_date));
    age
}

pub fn get_time() -> String {
    let now = Utc::now();
    let current_date_time = now.format("%Y-%m-%d %H:%M").to_string();
    current_date_time
}

pub fn read_file(file_name:String) -> String {
    let data = fs::read_to_string(file_name).expect("Something went wrong reading the file");
    data
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_get_time() {
        let now = get_time();
        assert_eq!(get_time(), now);
    }

    #[test]
    fn test_calculate_age(){
        let age: &str = "1989-10-11";
        let now = Utc::now().naive_local();
        let naive_date = NaiveDate::parse_from_str(&age, "%Y-%m-%d").unwrap();
        let age_calculated: String = format!("{:?}",NaiveDate::years_since( &NaiveDate::from(now),naive_date));
        assert_eq!(calculate_age("1989-10-11".parse().unwrap()), age_calculated);
    }

    #[test]
    fn test_read_file(){
        assert!(read_file(String::from("someFile")).len() > 0);
    }

}