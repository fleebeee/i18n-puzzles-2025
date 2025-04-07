i18n_puzzles::solution!(15);
use chrono::{DateTime, Datelike, Duration, NaiveDate, TimeZone, Utc};
use chrono_tz::Tz;
use itertools::Itertools;

#[derive(Debug)]
struct Office {
    timezone: Tz,
    holidays: Vec<DateTime<Tz>>,
}

#[derive(Debug)]
struct Customer {
    timezone: Tz,
    holidays: Vec<DateTime<Tz>>,
    overtime: u64,
}

// 6 June 2022
const DATE_FORMAT: &str = "%d %B %Y";

fn parse_date(date_str: &str, tz: Tz) -> DateTime<Tz> {
    let naive = NaiveDate::parse_from_str(date_str, DATE_FORMAT).unwrap();
    let naive_datetime = naive.and_hms_opt(0, 0, 0).unwrap();
    tz.from_local_datetime(&naive_datetime).single().unwrap()
}

fn parse_input(input: &str) -> (Vec<Office>, Vec<Customer>) {
    let mut offices = vec![];
    let mut customers = vec![];

    let (offices_str, customers_str) = input.split_once("\n\n").unwrap();

    for line in offices_str.lines() {
        let (_label, timezone, holidays) = line.split('\t').collect_tuple().unwrap();
        let tz: Tz = timezone.parse().unwrap();

        let holidays: Vec<_> = holidays.split(';').map(|h| parse_date(h, tz)).collect();

        offices.push(Office {
            timezone: tz,
            holidays,
        })
    }

    for line in customers_str.lines() {
        let (_label, timezone, holidays) = line.split('\t').collect_tuple().unwrap();
        let tz: Tz = timezone.parse().unwrap();

        let holidays: Vec<_> = holidays.split(';').map(|h| parse_date(h, tz)).collect();

        customers.push(Customer {
            timezone: tz,
            holidays,
            overtime: 0,
        })
    }

    (offices, customers)
}

// Function to check if a timestamp falls on a specific date in a specific timezone
fn is_same_date(timestamp: DateTime<Tz>, date: DateTime<Tz>) -> bool {
    let timestamp_date = timestamp.date_naive();
    let comparison_date = date.date_naive();

    timestamp_date == comparison_date
}

pub fn part_one(input: &str) -> Option<u64> {
    let (offices, mut customers) = parse_input(input);

    let start_date = NaiveDate::from_ymd_opt(2022, 1, 1).unwrap();
    let end_date = NaiveDate::from_ymd_opt(2022, 12, 31).unwrap();

    let start = Utc.from_utc_datetime(&start_date.and_hms_opt(0, 0, 0).unwrap());
    let end = Utc.from_utc_datetime(&end_date.and_hms_opt(23, 59, 59).unwrap());

    // Iterate over every day of 2022
    let mut current_date = start;
    while current_date <= end {
        let day_start = current_date.timestamp();
        let day_end = (current_date + Duration::days(1)).timestamp();

        // Iterate over each minute in the day
        for m in (day_start..day_end).step_by(60) {
            'customer: for customer in customers.iter_mut() {
                // Time at customer
                let customer_date = Utc
                    .timestamp_opt(m, 0)
                    .unwrap()
                    .with_timezone(&customer.timezone);

                // No support on weekends
                let weekday = customer_date.weekday().num_days_from_monday();
                if weekday >= 5 {
                    // 5 = Saturday, 6 = Sunday
                    continue;
                }

                // No support on holidays
                if customer
                    .holidays
                    .iter()
                    .any(|h| is_same_date(customer_date, *h))
                {
                    continue;
                }

                // Figure out if there's an office working right now
                for office in &offices {
                    let office_date = Utc
                        .timestamp_opt(m, 0)
                        .unwrap()
                        .with_timezone(&office.timezone);

                    // No support on weekends
                    let weekday = office_date.weekday().num_days_from_monday();
                    if weekday >= 5 {
                        // 5 = Saturday, 6 = Sunday
                        continue;
                    }

                    // No support on holidays
                    if office
                        .holidays
                        .iter()
                        .any(|h| is_same_date(office_date, *h))
                    {
                        continue;
                    }

                    // Check if the office time is between 8:30 and 17:00
                    let office_time = office_date.time();
                    let start_time = chrono::NaiveTime::from_hms_opt(8, 30, 0).unwrap();
                    let end_time = chrono::NaiveTime::from_hms_opt(17, 0, 0).unwrap();

                    // If they are working, skip this customer
                    if office_time >= start_time && office_time < end_time {
                        continue 'customer;
                    }
                }

                customer.overtime += 1;
            }
        }

        // Move to the next day
        current_date = current_date + Duration::days(1);
    }

    let overtimes: Vec<u64> = customers.iter().map(|c| c.overtime).collect();
    let biggest = overtimes.iter().max().unwrap();
    let smallest = overtimes.iter().min().unwrap();

    Some(biggest - smallest)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&i18n_puzzles::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&i18n_puzzles::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
