use chrono::{DateTime, NaiveDateTime, TimeZone};
use chrono_tz::Tz;

pub fn parse_date_with_iana(date_str: &str, iana: &str, date_format: &str) -> DateTime<Tz> {
    let naive = NaiveDateTime::parse_from_str(date_str, date_format).unwrap();
    let timezone: Tz = iana.parse().unwrap();

    timezone.from_local_datetime(&naive).unwrap()
}
