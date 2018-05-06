extern crate csv;
//extern crate serde;
//#[macro_use]
//extern crate serde_derive;

use std::io::prelude::*;

mod time;
use time::Time;

struct WR {
    table: i32,
    lev: i32,
    time: Time,
    kuski: String,
}

//TODO(edahl): put time stuff in a module
//TODO(edahl): store time as single int and use display/format for string
//TODO(edahl): fix time methods to account for single int storage
//TODO(edahl): read lev names from a file
//TODO(edahl): find sensible html-view crate
//TODO(edahl): code to decrypt state or use crate
fn main() {
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

    // Read WR table data
    let mut wr_tables = Vec::new();

    let mut r = csv::Reader::from_file("2018-04-19_elma_wrs.csv").unwrap();

    for record in r.records() {
        let row = record.unwrap();
        wr_tables.push(WR {
            table: row[0].parse::<i32>().unwrap(),
            lev: row[1].parse::<i32>().unwrap(),
            time: time::from_string(&row[3].to_string()),
            kuski: row[4].to_string(),
        });
    }

    // Read PR data
    let mut time_table = Vec::new();

    let mut f = std::fs::File::open("stats.txt").expect("Cannot open file: stats.txt");
    let mut c = String::new();
    f.read_to_string(&mut c)
        .expect("Cannot read file: stats.txt");

    let mut level_counter = 0;
    let mut level_found = false;
    for line in c.lines() {
        let mut data: Vec<&str> = line.trim().split_whitespace().collect();

        if data.len() != 0 && level_found {
            time_table.push(time::from_string(&String::from(data[0])));
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

    let headers = vec![
        "Lev", "Name", "PR", "Table", "Time", "Kuski", "Target", "Diff", "Kuski"
    ];

    let mut data = String::new();

    data.push_str(&format!(
        "{:<5}{:<19}{:<10}{:<7}{:<10}{:<13}{:<10}{:<11}{:<13}\r\n",
        headers[0],
        headers[1],
        headers[2],
        headers[3],
        headers[4],
        headers[5],
        headers[6],
        headers[7],
        headers[8]
    ));

    for i in 0..54 {
        let t = &time_table[i];
        let lev: i32 = (i as i32) + 1;
        let last_wr_beat = wr_tables
            .iter()
            .filter(|x| (x.lev == lev) && time::compare(t, &x.time))
            .last();
        let first_wr_not_beat = wr_tables
            .iter()
            .filter(|x| (x.lev == lev) && !time::compare(t, &x.time))
            .nth(0);

        let lev_number = lev.to_string();
        let lev_name = level_names[i];
        let pr = time::to_string(t);

        let (last_table_beat, last_time_beat, last_kuski_beat) = if let Some(wr) = last_wr_beat {
            (
                wr.table.to_string(),
                time::to_string(&wr.time),
                wr.kuski.clone(),
            )
        } else {
            ("-".into(), "-".into(), "-".into())
        };

        let (next_target, diff, next_kuski) = if let Some(wr) = first_wr_not_beat {
            (
                time::to_string(&wr.time),
                "+".to_owned() + &time::to_string(&time::difference(t, &wr.time)),
                wr.kuski.clone(),
            )
        } else {
            ("-".into(), "-".into(), "-".into())
        };

        data.push_str(&format!(
            "{:<5}{:<19}{:<10}{:<7}{:<10}{:<13}{:<10}{:<11}{:<13}\r\n",
            lev_number,
            lev_name,
            pr,
            last_table_beat,
            last_time_beat,
            last_kuski_beat,
            next_target,
            diff,
            next_kuski
        ));
    }

    let mut f = std::fs::File::create("wrs_beat.txt").expect("Could not create file: wrs_beat.txt");
    f.write_all(&data.into_bytes())
        .expect("Could not write to file: wrs_beat.txt");
    //std::fs::write("wrs_beat.txt", data).expect("Unable to write file");

    println!("Script is finished running. Data saved in wrs_beat.txt.");
}
