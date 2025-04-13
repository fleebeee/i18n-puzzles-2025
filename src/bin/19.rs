i18n_puzzles::solution!(19);
use chrono::DateTime;
use chrono::Utc;
use hashbrown::HashMap;
use hashbrown::HashSet;
use std::process::Command;

pub fn convert_to_utc(
    project_root: &str,
    date_str: &str,
    tz: &str,
    tz_version: &str,
) -> Option<String> {
    // This command uses gdate (date on Linux) to apply the requested zoneinfo to a date
    let cmd = format!(
        "TZ={project_root}/data/timezones/zoneinfo-{tz_version}/usr/share/zoneinfo/{tz} gdate -d \"{date_str}\" +\"%Y-%m-%dT%H:%M:%S%z\"",
    );

    let date_output = match Command::new("sh").arg("-c").arg(&cmd).output() {
        Ok(output) => output,
        Err(e) => {
            eprintln!("Command execution failed: {}", e);
            eprintln!("Command was: {}", cmd);
            return None;
        }
    };

    if date_output.status.success() {
        let result = String::from_utf8_lossy(&date_output.stdout)
            .trim()
            .to_string();

        // Parse the ISO8601 date string
        let date = DateTime::parse_from_str(&result, "%Y-%m-%dT%H:%M:%S%z").ok()?;

        // Convert to UTC and format back to ISO8601
        let utc_date = date.with_timezone(&Utc);
        let result = utc_date.format("%Y-%m-%dT%H:%M:%S+00:00").to_string();

        return Some(result);
    }

    // eprint!("{}", String::from_utf8(date_output.stderr).ok()?);

    None
}

fn get_project_root() -> String {
    // Find the project root folder, hopefully the current working directory
    // Why? I couldn't get relative paths working for the TZ=... declaration
    let output = Command::new("sh")
        .arg("-c")
        .arg("pwd")
        .output()
        .expect("Failed to execute pwd command");

    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

const VERSIONS: [&str; 4] = ["2018c", "2018g", "2021b", "2023d"];

pub fn part_one(input: &str) -> Option<String> {
    let project_root = get_project_root();

    // Hashmap of UTC date to set of research stations
    let mut map = HashMap::new();
    let mut research_stations = HashSet::new();

    for line in input.lines() {
        let (date_str, tz) = line.split_once("; ")?;
        research_stations.insert(tz);

        for tz_version in VERSIONS {
            match convert_to_utc(&project_root, date_str, tz, tz_version) {
                Some(date) => {
                    map.entry(date).or_insert(HashSet::new()).insert(tz);
                }
                None => {
                    // Some dates fail but fortunately we can still find the answer
                    // I think the failing dates are from where clocks were moved
                    // eprintln!(
                    //     "Failed to convert date: {} to timezone: {} with version: {}\n",
                    //     date_str, tz, tz_version
                    // );
                }
            }
        }
    }

    let result = map
        .iter()
        .find(|(_, v)| v.len() == research_stations.len())?
        .0;

    Some(result.clone())
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
        assert_eq!(result, Some("2024-04-09T17:49:00+00:00".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&i18n_puzzles::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
