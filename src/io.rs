extern crate csv;

use WR;
use time;
use time::Time;
use ::std::io::prelude::*;

fn read_wr_tables() -> Vec<WR> {
    let mut wrt = Vec::new();
    let mut r = csv::Reader::from_file("2018-04-19_elma_wrs.csv").unwrap();
    for record in r.records() {
        let row = record.unwrap();
        wrt.push(WR {
            table: row[0].parse::<i32>().unwrap(),
            lev: row[1].parse::<i32>().unwrap(),
            time: time::from_string(&row[3].to_string()),
            kuski: row[4].to_string(),
        });
    }
    wrt
}

fn read_stats() -> Vec<Time> {
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
            prt.push(time::from_string(&String::from(data[0])));
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

pub fn populate_table_data() -> Vec<Vec<String>> {
    let mut data: Vec<Vec<String>> = Vec::new();

    // Read WR table data
    let wr_tables = read_wr_tables();

    // Read PR data
    let pr_table = read_stats();

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
        let t = &pr_table[i];
        let lev: i32 = (i as i32) + 1;
        let last_wr_beat = wr_tables
            .iter()
            .filter(|x| (x.lev == lev) && time::compare(t, &x.time))
            .last();
        let first_wr_not_beat = wr_tables
            .iter()
            .filter(|x| (x.lev == lev) && !time::compare(t, &x.time))
            .nth(0);

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

        let lev_number = lev.to_string();
        let pr = time::to_string(t);
        data.push(vec![
            lev_number,
            (*lev_name).into(),
            pr,
            last_table_beat,
            last_time_beat,
            last_kuski_beat,
            next_target,
            diff,
            next_kuski,
        ]);
    }
    data
}