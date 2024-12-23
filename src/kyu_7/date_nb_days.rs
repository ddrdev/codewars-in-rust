use chrono::{Duration, NaiveDate};

pub fn date_nb_days(a0: i32, a: i32, p: i32) -> String {
    let daily_interest_rate = p as f64 / 36_000f64;
    let target_amount = a as f64;
    let current_amount = a0 as f64;

    let days =
        ((target_amount / current_amount).ln() / (1f64 + daily_interest_rate).ln()).ceil() as i64;

    let start_date = NaiveDate::from_ymd(2016, 1, 1);
    let target_date = start_date + Duration::days(days);

    target_date.format("%Y-%m-%d").to_string()
}
