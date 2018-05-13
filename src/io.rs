extern crate csv;
extern crate elma;

use elma::Time;
use failure::Error;
use shared::{DataRow, Targets, WR};
use std::path::Path;

pub fn populate_table_data(pr_table: &[Time], wr_tables: &[WR]) -> Vec<DataRow> {
    let level_names = vec![
        "Warm Up",
        "Flat Track",
        "Twin Peaks",
        "Over and Under",
        "Uphill Battle",
        "Long Haul",
        "Hi Flyer",
        "Tag",
        "Tunnel Terror",
        "The Steppes",
        "Gravity Ride",
        "Islands in the Sky",
        "Hill Legend",
        "Loop-de-Loop",
        "Serpents Tale",
        "New Wave",
        "Labyrinth",
        "Spiral",
        "Turnaround",
        "Upside Down",
        "Hangman",
        "Slalom",
        "Quick Round",
        "Ramp Frenzy",
        "Precarious",
        "Circuitous",
        "Shelf Life",
        "Bounce Back",
        "Headbanger",
        "Pipe",
        "Animal Farm",
        "Steep Corner",
        "Zig-Zag",
        "Bumpy Journey",
        "Labyrinth Pro",
        "Fruit in the Den",
        "Jaws",
        "Curvaceous",
        "Haircut",
        "Double Trouble",
        "Framework",
        "Enduro",
        "He He",
        "Freefall",
        "Sink",
        "Bowling",
        "Enigma",
        "Downhill",
        "What the Heck",
        "Expert System",
        "Tricks Abound",
        "Hang Tight",
        "Hooked",
        "Apple Harvest",
    ];

    level_names
        .iter()
        .enumerate()
        .map(|(i, lev_name)| {
            let pr = pr_table[i];
            let lev = i as i32 + 1;
            let last_wr_beat = wr_tables
                .iter()
                .filter(|wr| (wr.lev == lev) && (pr <= wr.time))
                .last();
            let first_wr_not_beat = wr_tables
                .iter()
                .filter(|wr| (wr.lev == lev) && !(pr <= wr.time))
                .nth(0);

            DataRow {
                lev_number: lev,
                lev_name: lev_name.to_string(),
                pr: pr,
                wr_beat: last_wr_beat.cloned(),
                wr_not_beat: first_wr_not_beat.cloned(),
            }
        })
        .collect()
}

pub fn load_targets_table() -> Result<Vec<Targets>, Error> {
    let path = Path::new("wr-stats_targets.csv");
    let mut r = csv::Reader::from_path(path)?;
    
    Ok(r.records()
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
    let path = Path::new("wr-stats_tables.csv");
    let mut r = csv::Reader::from_path(path)?;
    
    Ok(r.records()
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
    let state = elma::state::State::load("state.dat")?;

    Ok(state
        .times
        .iter()
        .take(54)
        .map(|x| x.single.first().map_or(Time::from("10:00,00"), |x| x.time))
        .collect())
}

pub fn read_stats() -> Result<Vec<Time>, Error> {
    let s = ::std::fs::read_to_string("stats.txt")?;

    let mut prt = Vec::new();
    let mut level_counter = 0;
    let mut level_found = false;
    for line in s.lines() {
        let mut data: Vec<&str> = line.trim().split_whitespace().collect();

        if data.len() != 0 && level_found {
            prt.push(Time::from(data[0].as_ref()));
            level_counter += 1;
            level_found = false;
        }

        if data.len() != 0 && data[0] == "Level" {
            level_found = true;
        }

        if level_counter == 54 {
            break;
        }
    }
    Ok(prt)
}
