extern crate csv;
extern crate regex;
//extern crate serde;
//#[macro_use]
//extern crate serde_derive;


use std::io::prelude::*;

//#[derive(Debug,Deserialize)]
struct Time {
    minutes : i32,
    seconds : i32,
    milliseconds : i32
}

//#[derive(Debug,Deserialize)]
struct WR {
    table : i32,
    lev : i32,
    time : Time,
    kuski : String
}

fn compare_times(t1 : &Time, t2 : &Time) -> bool {
    t1.minutes * 60 * 100 + t1.seconds * 100 + t1.milliseconds <= t2.minutes * 60 * 100 + t2.seconds * 100 + t2.milliseconds
}

fn time_to_string(t : &Time) -> String {
    format!("{min:02}:{sec:02},{ms:02}", min=t.minutes, sec=t.seconds, ms=t.milliseconds)
}

fn time_from_string(st : &String) -> Time {

    let re = regex::Regex::new(r":|,").unwrap();
    let t : Vec<&str> = re.split(&st).collect();
    let (m, s, ms) = if t.len() == 3 {
        (t[0].parse::<i32>().unwrap(), t[1].parse::<i32>().unwrap(), t[2].parse::<i32>().unwrap())
    } else {
        (0, t[0].parse::<i32>().unwrap(), t[1].parse::<i32>().unwrap())
    };

    Time { minutes : m, seconds : s, milliseconds : ms }
}

fn time_difference(t1 : &Time, t2 : &Time) -> Time {
    let mut m = t1.minutes - t2.minutes;
    let mut s = 0;
    let mut ms = 0;

    if ms < 0 {
        s = t1.seconds - t2.seconds - 1;
        ms = ms + 100;
    } else {
        s = t1.seconds - t2.seconds;
    }

    if s < 0 {
        m = t1.minutes - t2.minutes - 1;
        s = 60 + s;
    } else {
        m = t1.minutes - t2.minutes;
    }

    Time{ minutes : m, seconds : s, milliseconds: ms }
}

fn add_times(t1 : Time, t2 : Time) -> Time{
    let mut m = t1.minutes + t2.minutes;
    let mut s = 0;
    let mut ms = 0;

    if ms > 99 {
        s = t1.seconds + t2.seconds + 1;
        ms = ms - 100;
    } else {
        s = t1.seconds + t2.seconds;
    }

    if s > 60 {
        m = t1.minutes + t2.minutes + 1;
        s = s-60;
    } else {
        m = t1.minutes + t2.minutes;
    }

    Time{ minutes : m, seconds : s, milliseconds: ms }
}

#[test]
fn test_time_to_string() {
    let t = Time { minutes : 1, seconds : 32, milliseconds : 56 };
    println!("{}", time_to_string(t));
 }

fn main() {
    let level_names = [
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
         "Apple Harvest"];
        
    let t = Time { minutes : 1, seconds : 32, milliseconds : 56 };
    println!("{}", time_to_string(&t));

    println!("{}", time_to_string(&time_from_string(&String::from("00:15,31"))));


    // Read WR table data
    let mut wr_tables = Vec::new();

    {
        let mut r = csv::Reader::from_file("2018-04-19_elma_wrs.csv").unwrap();

        for record in r.records() {
            let row = record.unwrap();
            wr_tables.push(WR{ 
                table: row[0].parse::<i32>().unwrap(),
                lev : row[1].parse::<i32>().unwrap(),
                time : time_from_string(&row[3].to_string()),
                kuski : row[4].to_string() });
        }
    }

    // Read PR data
    let mut time_table = Vec::new();

    {
        let mut f = std::fs::File::open("stats.txt").expect("Cannot open file: stats.txt");
        let mut c = String::new();
        f.read_to_string(&mut c).expect("Cannot read file: stats.txt");
        
        let mut level_counter = 0;
        let mut level_found = false;
        for line in c.lines() {
            let mut data : Vec<&str> = line.trim().split_whitespace().collect();

            if data.len() != 0 && level_found {
                time_table.push(time_from_string(&String::from(data[0])));
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
    }

    let headers = vec!["Lev", "Name", "PR", "Table", "Time", "Kuski", "Target", "Diff", "Kuski"];

    let mut last_table_beat : String;
    let mut last_time_beat : String;
    let mut last_kuski_beat : String;
    let mut next_target : String;
    let mut diff : String;
    let mut next_kuski : String;

    let mut data = String::new();

    data.push_str(&format!("{:<5}{:<19}{:<10}{:<7}{:<10}{:<13}{:<10}{:<11}{:<13}\r\n", headers[0], headers[1], headers[2], headers[3], headers[4], headers[5], headers[6], headers[7], headers[8]));

    for i in 0..54 {
        let t = &time_table[i];
        let lev : i32 = (i as i32) + 1;
        let last_wr_beat = &wr_tables.iter().filter(|x| (x.lev == lev) && compare_times(t, &x.time)).last();
        let first_wr_not_beat = &wr_tables.iter().filter(|x| (x.lev == lev) && !compare_times(t, &x.time)).nth(0);

        let lev_number = lev.to_string();
        let lev_name = level_names[i];
        let pr = time_to_string(t);
    
        if last_wr_beat.is_some() {
            last_table_beat = last_wr_beat.unwrap().table.to_string();
            last_time_beat = time_to_string(&last_wr_beat.unwrap().time);
            last_kuski_beat = last_wr_beat.unwrap().kuski.clone();
        }
        else {
            last_table_beat = String::from("-");
            last_time_beat = String::from("-");
            last_kuski_beat = String::from("-");
        }

        if first_wr_not_beat.is_some() {
            next_target = time_to_string(&first_wr_not_beat.unwrap().time);
            diff = "+".to_owned() + &time_to_string(&time_difference(t, &first_wr_not_beat.unwrap().time));
            next_kuski = first_wr_not_beat.unwrap().kuski.clone();
        }
        else {
            next_target = String::from("-");
            diff = String::from("-");
            next_kuski = String::from("-");
        }
        
        data.push_str(&format!("{:<5}{:<19}{:<10}{:<7}{:<10}{:<13}{:<10}{:<11}{:<13}\r\n",
            lev_number,
            lev_name,
            pr,
            last_table_beat,
            last_time_beat,
            last_kuski_beat,
            next_target,
            diff,
            next_kuski));
    }

    {
        let mut f = std::fs::File::create("wrs_beat.txt").expect("Could not create file: wrs_beat.txt");
        f.write_all(&data.into_bytes()).expect("Could not write to file: wrs_beat.txt");
        //std::fs::write("wrs_beat.txt", data).expect("Unable to write file");
    }
    println!("Script is finished running. Data saved in wrs_beat.txt. (jk)");
}