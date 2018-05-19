extern crate csv;
extern crate elma;

use elma::Time;
use failure::Error;
use shared::{DataRow, Targets, WR};

pub fn load_targets_table() -> Result<Vec<Targets>, Error> {
    Ok(csv::Reader::from_path("wr-stats_targets.csv")?
        .records()
        .map(|r| r.unwrap_or(csv::StringRecord::from(vec!["00:00:00"; 7])))
        .map(|row| Targets {
            godlike: Time::from(&row[0]),
            legendary: Time::from(&row[1]),
            world_class: Time::from(&row[2]),
            professional: Time::from(&row[3]),
            good: Time::from(&row[4]),
            ok: Time::from(&row[5]),
            beginner: Time::from(&row[6]),
        })
        .collect())
}

pub fn load_wr_tables() -> Result<Vec<WR>, Error> {
    Ok(csv::Reader::from_path("wr-stats_tables.csv")?
        .records()
        .map(|r| r.unwrap())
        .map(|row| WR {
            table: row[0].parse::<i32>().unwrap(),
            lev: row[1].parse::<i32>().unwrap(),
            time: Time::from(&row[2]),
            kuski: row[3].to_string(),
        })
        .collect())
}

pub fn load_state() -> Result<Vec<Time>, elma::ElmaError> {
    Ok(elma::state::State::load("state.dat")?
        .times
        .iter()
        .take(54)
        .map(|x| x.single.first().map_or(Time::from("10:00,00"), |x| x.time))
        .collect())
}

pub fn load_stats() -> Result<Vec<Time>, Error> {
    Ok(::std::fs::read_to_string("stats.txt")?
        .lines()
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|entry_pair| entry_pair[0].starts_with("Level"))
        .take(54)
        .map(|entry_pair| {
            Time::from(
                entry_pair[1]
                    .split_whitespace()
                    .next()
                    .unwrap_or("10:00:00"),
            )
        })
        .collect())
}
