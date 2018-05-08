extern crate csv;
extern crate elma;

use WR;
use Targets;
use DataRow;
use std::io::prelude::*;
use elma::Time;

pub fn read_targets_table() -> Vec<Targets> {
    let mut tst = Vec::new();
    let mut r = csv::Reader::from_file("targets.csv").unwrap();
    for record in r.records() {
        if let Ok(row) = record {
            tst.push(Targets {
                godlike: Time::from(&row[0]),
                legendary: Time::from(&row[1]),
                world_class: Time::from(&row[2]),
                professional: Time::from(&row[3]),
                good: Time::from(&row[4]),
                ok: Time::from(&row[5]),
                beginner: Time::from(&row[6]),
            });
        }
    }
    tst
}

pub fn read_wr_tables() -> Vec<WR> {
    let mut wrt = Vec::new();
    let mut r = csv::Reader::from_file("2018-04-19_elma_wrs.csv").unwrap();
    for record in r.records() {
        let row = record.unwrap();
        wrt.push(WR {
            table: row[0].parse::<i32>().unwrap(),
            lev: row[1].parse::<i32>().unwrap(),
            time: Time::from(&row[3]),
            kuski: row[4].to_string(),
        });
    }
    wrt
}

pub fn read_state() -> Result<Vec<Time>, elma::ElmaError> {
    let mut prt = Vec::new();

    let state = elma::state::State::load("state.dat")?;

    for lev in state.times.iter().take(54) {
        if let Some(t) = lev.single.first() {
            prt.push(t.time);
        } else {
            prt.push(Time::from("10:00,00"))
        }
    }

    Ok(prt)
}

pub fn populate_table_data(pr_table: &[Time], wr_tables: &[WR]) -> Vec<DataRow> {
    let mut data: Vec<DataRow> = Vec::new();

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
    for (i, lev_name) in level_names.iter().enumerate() {
        //this if is unnecessary but ...
        let t = if i < pr_table.len() {
            pr_table[i]
        } else {
            Time::from("10:00,00")
        };
        let lev = i as i32 + 1;
        let last_wr_beat = wr_tables
            .iter()
            .filter(|x| (x.lev == lev) && (t <= x.time))
            .last();
        let first_wr_not_beat = wr_tables
            .iter()
            .filter(|x| (x.lev == lev) && !(t <= x.time))
            .nth(0);

        data.push(DataRow {
            lev_number: lev,
            lev_name: lev_name.to_string(),
            pr: t,
            wr_beat: last_wr_beat.cloned(),
            wr_not_beat: first_wr_not_beat.cloned(),
        });
    }
    data
}

pub fn read_stats() -> Vec<Time> {
    let mut prt = Vec::new();

    let mut f = ::std::fs::File::open("stats.txt").expect("Cannot open file: stats.txt");
    let mut c = String::new();
    f.read_to_string(&mut c)
        .expect("Cannot read file: stats.txt");

    let mut level_counter = 0;
    let mut level_found = false;
    for line in c.lines() {
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
    prt
}
