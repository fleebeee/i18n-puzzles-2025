use chrono::{DateTime, NaiveDateTime, Offset, TimeZone};
use chrono_tz::Tz;

pub fn parse_date_with_iana(date_str: &str, iana: &str, date_format: &str) -> DateTime<Tz> {
    let naive = NaiveDateTime::parse_from_str(date_str, date_format).unwrap();
    let timezone: Tz = iana.parse().unwrap();

    timezone.from_local_datetime(&naive).unwrap()
}

pub fn date_belongs_to_iana(date_str: &str, iana: &str) -> bool {
    let date = DateTime::parse_from_rfc3339(date_str).unwrap();
    // let date = DateTime::parse_from_str(date_str, date_format).unwrap();
    let offset = date.offset().fix();
    let utc_date = date.naive_utc();

    let timezone: Tz = iana.parse().unwrap();
    let guess = timezone.from_utc_datetime(&utc_date).offset().fix();

    guess == offset
}

pub fn determine_iana_from_options(date_str: &str, options: &[String]) -> Option<String> {
    let date = DateTime::parse_from_rfc3339(date_str).unwrap();
    // let date = DateTime::parse_from_str(date_str, date_format).unwrap();
    let offset = date.offset().fix();
    let utc_date = date.naive_utc();

    for option in options {
        let timezone: Tz = option.parse().unwrap();
        let guess = timezone.from_utc_datetime(&utc_date).offset().fix();

        if guess == offset {
            return Some(option.clone());
        }
    }

    None
}

fn _test_that_shit() {
    let input = vec![
        "2004-10-08T20:17:00.000-04:00",
        "2002-05-18T14:54:00.000-04:00",
        "2016-12-08T10:18:00.000-03:00",
    ];

    let options = vec![
        "America/Halifax".to_string(),
        "America/Santiago".to_string(),
    ];

    for date_str in input {
        let result = determine_iana_from_options(date_str, &options).unwrap();
        dbg!(&result);
    }
}
