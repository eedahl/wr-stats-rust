extern crate csv;
extern crate elma;

use elma::Time;
use failure::Error;
use shared::{DataRow, Targets, WR};

pub struct Model {
    wr_tables: Vec<WR>,
    targets_table: Vec<Targets>,
    pr_table: Vec<Time>,
}

impl Model {
    pub fn new() -> Result<Self, Error> {
        let wr_tables = load_wr_tables()?;
        let targets_table = load_targets_table()?;
        let pr_table = match load_state() {
            Ok(t) => t,
            Err(_) => load_stats()?,
        };

        Ok(Self {
            wr_tables,
            targets_table,
            pr_table,
        })
    }

    pub fn update_pr_table(&mut self) -> Result<(), Error> {
        self.pr_table = match load_state() {
            Ok(t) => t,
            Err(_) => load_stats()?,
        };

        Ok(())
    }
}

pub trait ModelSelects {
    fn compute_tts(&self, drs: &[DataRow]) -> (Time, Time);
    fn collect_last_wr_table(&self) -> Vec<WR>;
    fn collect_current_wrs(&self) -> Vec<Time>;
    fn collect_wrs_for_lev(&self, level: usize) -> Vec<i32>;
    fn get_current_wr(&self, level: usize) -> Time;
    fn get_pr(&self, level: usize) -> Time;
    fn get_targets(&self, level: usize) -> &Targets;
    fn get_time_class(&self, time: &Time, level: usize) -> String;
    fn get_next_target(&self, time: &Time, level: usize) -> Time;
    fn get_last_wr_beat(&self, time: &Time, level: usize) -> Option<&WR>;
    fn get_first_wr_not_beat(&self, time: &Time, level: usize) -> Option<&WR>;
    fn get_targets_tt(&self) -> Time;
}

impl ModelSelects for Model {
    fn compute_tts(&self, drs: &[DataRow]) -> (Time, Time) {
        drs.iter().fold((Time(0), Time(0)), |acc, dr| {
            (
                acc.0 + dr.pr,
                acc.1 + if let Some(wr) = dr.wr_not_beat.clone() {
                    wr.time
                } else {
                    dr.pr
                },
            )
        })
    }

    fn collect_last_wr_table(&self) -> Vec<WR> {
        let last_table_num = self.wr_tables.iter().last().unwrap().table;
        self.wr_tables
            .iter()
            .filter(|x| x.table == last_table_num)
            .cloned()
            .collect()
    }

    fn collect_current_wrs(&self) -> Vec<Time> {
        let cur_wrt = self.collect_last_wr_table();
        self.pr_table
            .iter()
            .zip(cur_wrt.iter())
            .map(|(x, y)| *x.min(&y.time))
            .collect()
    }

    fn collect_wrs_for_lev(&self, level: usize) -> Vec<i32> {
        self.wr_tables
            .iter()
            .filter(|x| ((*x).lev - 1) as usize == level)
            .map(|x| x.time.0)
            .collect::<Vec<_>>()
    }

    fn get_pr(&self, level: usize) -> Time {
        self.pr_table[level]
    }

    fn get_current_wr(&self, level: usize) -> Time {
        self.collect_current_wrs()[level]
    }

    fn get_targets(&self, level: usize) -> &Targets {
        &self.targets_table[level]
    }

    fn get_time_class(&self, time: &Time, level: usize) -> String {
        let targets = self.get_targets(level);
        let current_wr = self.get_current_wr(level);
        match *time {
            t if t > targets.beginner => "unclassified",
            t if t > targets.ok => "beginner",
            t if t > targets.good => "ok",
            t if t > targets.professional => "good",
            t if t > targets.world_class => "professional",
            t if t > targets.legendary => "world_class",
            t if t > targets.godlike => "legendary",
            t if t > current_wr => "godlike",
            _ => "wr",
        }.to_string()
    }

    fn get_next_target(&self, time: &Time, level: usize) -> Time {
        let targets = self.get_targets(level);
        let current_wr = self.get_current_wr(level);
        match *time {
            t if t > targets.beginner => targets.beginner,
            t if t > targets.ok => targets.ok,
            t if t > targets.good => targets.good,
            t if t > targets.professional => targets.professional,
            t if t > targets.world_class => targets.world_class,
            t if t > targets.legendary => targets.legendary,
            t if t > targets.godlike => targets.godlike,
            t if t > current_wr => current_wr,
            _ => current_wr,
        }
    }

    fn get_last_wr_beat(&self, time: &Time, level: usize) -> Option<&WR> {
        self.wr_tables
            .iter()
            .filter(|wr| (wr.lev == level as i32 + 1) && (time <= &wr.time))
            .last()
    }

    fn get_first_wr_not_beat(&self, time: &Time, level: usize) -> Option<&WR> {
        self.wr_tables
            .iter()
            .filter(|wr| (wr.lev == level as i32 + 1) && !(time <= &wr.time))
            .nth(0)
    }

    fn get_targets_tt(&self) -> Time {
        self.pr_table
            .iter()
            .enumerate()
            .fold(Time(0), |acc, (i, pr)| acc + self.get_next_target(&pr, i))
    }
}

fn load_wr_tables() -> Result<Vec<WR>, Error> {
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

fn load_targets_table() -> Result<Vec<Targets>, Error> {
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

fn load_state() -> Result<Vec<Time>, elma::ElmaError> {
    Ok(elma::state::State::load("state.dat")?
        .times
        .iter()
        .take(54)
        .map(|x| x.single.first().map_or(Time::from("10:00,00"), |x| x.time))
        .collect())
}

fn load_stats() -> Result<Vec<Time>, Error> {
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
