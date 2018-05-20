use elma::Time;
use failure::Error;
use io;
use serde_json;

#[derive(Debug, Clone)]
pub struct Targets {
    pub godlike: Time,
    pub legendary: Time,
    pub world_class: Time,
    pub professional: Time,
    pub good: Time,
    pub ok: Time,
    pub beginner: Time,
}

#[derive(Debug, Clone)]
pub struct WR {
    pub table: i32,
    pub lev: i32,
    pub time: Time,
    pub kuski: String,
}

#[derive(Debug)]
pub struct DataRow {
    pub lev_number: i32,
    pub lev_name: String,
    pub pr: Time,
    pub wr_beat: Option<WR>,
    pub wr_not_beat: Option<WR>,
}

#[derive(Debug, Clone, Copy)]
pub enum SortBy {
    PR(SortOrder),
    DiffToPrevWR(SortOrder),
    DiffToNextWR(SortOrder),
    DiffToNextTarget(SortOrder),
    LevelNum(SortOrder),
    Table(SortOrder),
}

#[derive(Debug, Clone, Copy)]
pub enum SortOrder {
    Ascending,
    Descending,
}

pub fn get_sort_hint(sort_param: &str, ascending: bool) -> SortBy {
    match sort_param {
        "PR" => SortBy::PR(if ascending {
            SortOrder::Ascending
        } else {
            SortOrder::Descending
        }),
        "DiffToPrevWR" => SortBy::DiffToPrevWR(if ascending {
            SortOrder::Ascending
        } else {
            SortOrder::Descending
        }),
        "DiffToNextWR" => SortBy::DiffToNextWR(if ascending {
            SortOrder::Ascending
        } else {
            SortOrder::Descending
        }),
        "DiffToNextTarget" => SortBy::DiffToNextTarget(if ascending {
            SortOrder::Ascending
        } else {
            SortOrder::Descending
        }),
        "LevelNum" => SortBy::LevelNum(if ascending {
            SortOrder::Ascending
        } else {
            SortOrder::Descending
        }),
        "Table" => SortBy::Table(if ascending {
            SortOrder::Ascending
        } else {
            SortOrder::Descending
        }),
        &_ => SortBy::LevelNum(SortOrder::Ascending),
    }
}

pub fn get_level_update_data(wr_tables: &[WR], level: i32) -> Result<serde_json::Value, Error> {
    Ok(json!({"level": level, "times": serde_json::to_value(
        wr_tables
            .into_iter()
            .filter(|x| (*x).lev == level)
            .map(|x| x.time.0)
            .collect::<Vec<_>>(),
    )?}))
}

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

pub fn build_table_update_data(
    wr_tables: &[WR],
    targets_table: &[Targets],
    sort_by: SortBy,
) -> Result<serde_json::Value, Error> {
    let pr_table = match io::load_state() {
        Ok(t) => t,
        Err(_) => io::load_stats()?,
    };

    let data = populate_table_data(&pr_table, &wr_tables);
    let last_wr_table = collect_last_wr_table(&wr_tables);
    let current_wrs = collect_current_wrs(&pr_table, &last_wr_table);

    // * Footer
    let (p_tt, target_wr_tt) = compute_tts(&data);
    let target_tt = pr_table.iter().enumerate().fold(Time(0), |acc, (i, pr)| {
        acc + get_next_target(&pr, &targets_table[i], &current_wrs[i])
    });

    let footer_json =
        json!({"p_tt": p_tt.0, "target_wr_tt": target_wr_tt.0, "target_tt": target_tt.0});

    // * Body
    let mut collate = data
        .into_iter()
        .zip(targets_table.into_iter().cloned())
        .zip(current_wrs.into_iter())
        .collect::<Vec<((DataRow, Targets), Time)>>();

    match sort_by {
        SortBy::Table(ord) => collate.sort_by(|x, y| {
            let table1 = if let Some(ref wr) = ((x.0).0).wr_beat {
                wr.table
            } else {
                0
            };
            let table2 = if let Some(ref wr) = ((y.0).0).wr_beat {
                wr.table
            } else {
                0
            };
            match ord {
                SortOrder::Ascending => table1.cmp(&table2),
                SortOrder::Descending => table2.cmp(&table1),
            }
        }),
        SortBy::PR(ord) => collate.sort_by(|x, y| {
            let pr1 = ((x.0).0).pr;
            let pr2 = ((y.0).0).pr;
            match ord {
                SortOrder::Ascending => pr1.cmp(&pr2),
                SortOrder::Descending => pr2.cmp(&pr1),
            }
        }),
        SortBy::DiffToNextTarget(ord) => collate.sort_by(|x, y| {
            let pr1 = ((x.0).0).pr;
            let tars1 = &(x.0).1;
            let cur_wr1 = x.1;
            let tar1 = get_next_target(&pr1, tars1, &cur_wr1);
            let pr2 = ((y.0).0).pr;
            let tars2 = &(y.0).1;
            let cur_wr2 = y.1;
            let tar2 = get_next_target(&pr2, tars2, &cur_wr2);
            match ord {
                SortOrder::Ascending => (pr1 - tar1).cmp(&(pr2 - tar2)),
                SortOrder::Descending => (pr2 - tar2).cmp(&(pr1 - tar1)),
            }
        }),
        SortBy::DiffToPrevWR(ord) => collate.sort_by(|x, y| {
            let pr1 = ((x.0).0).pr;
            let wr1 = if let Some(ref wr) = ((x.0).0).wr_beat {
                wr.time
            } else {
                pr1
            };
            let pr2 = ((y.0).0).pr;
            let wr2 = if let Some(ref wr) = ((y.0).0).wr_beat {
                wr.time
            } else {
                pr2
            };
            match ord {
                SortOrder::Ascending => (pr1 - wr1).cmp(&(pr2 - wr2)),
                SortOrder::Descending => (pr2 - wr2).cmp(&(pr1 - wr1)),
            }
        }),
        SortBy::DiffToNextWR(ord) => collate.sort_by(|x, y| {
            let pr1 = ((x.0).0).pr;
            let wr1 = if let Some(ref wr) = ((x.0).0).wr_not_beat {
                wr.time
            } else {
                pr1
            };
            let pr2 = ((y.0).0).pr;
            let wr2 = if let Some(ref wr) = ((y.0).0).wr_not_beat {
                wr.time
            } else {
                pr2
            };
            match ord {
                SortOrder::Ascending => (pr1 - wr1).cmp(&(pr2 - wr2)),
                SortOrder::Descending => (pr2 - wr2).cmp(&(pr1 - wr1)),
            }
        }),
        SortBy::LevelNum(ord) => match ord {
            SortOrder::Ascending => {}
            SortOrder::Descending => collate.sort_by(|x, y| {
                let lev_num1 = ((x.0).0).lev_number;
                let lev_num2 = ((y.0).0).lev_number;
                lev_num2.cmp(&lev_num1)
            }),
        },
    }
    let (unpack, current_wrs_sorted): (Vec<(DataRow, Targets)>, Vec<Time>) =
        collate.into_iter().unzip();
    let (data_sorted, targets_sorted): (Vec<DataRow>, Vec<Targets>) = unpack.into_iter().unzip();

    let json_row_data: serde_json::Value = data_sorted
        .iter()
        .enumerate()
        .map(
            |(
                i,
                DataRow {
                    lev_number,
                    lev_name,
                    pr,
                    wr_beat,
                    wr_not_beat,
                },
            )| {
                let pr_class = get_time_class(pr, &targets_sorted[i], &current_wrs_sorted[i]);
                let target = get_next_target(pr, &targets_sorted[i], &current_wrs_sorted[i]);
                let target_class = if target != Time(0) {
                    get_time_class(&target, &targets_sorted[i], &current_wrs_sorted[i])
                } else {
                    "".to_owned()
                };
                let (table_b, _, time_b, kuski_b) = wr_to_values(wr_beat);
                let wr_b_class =
                    get_time_class(&time_b, &targets_sorted[i], &current_wrs_sorted[i]);
                let (table_nb, _, time_nb, kuski_nb) = wr_to_values(wr_not_beat);
                let wr_nb_class =
                    get_time_class(&time_nb, &targets_sorted[i], &current_wrs_sorted[i]);
                json!({"lev_number": lev_number,
                        "lev_name": lev_name,
                        "pr" : {"time": pr.0, "class": pr_class},
                        "wr_beat": { "time": time_b.0, "class": wr_b_class, "table": table_b, "kuski": kuski_b },
                        "wr_not_beat": { "time": time_nb.0, "class": wr_nb_class, "table": table_nb, "kuski": kuski_nb },
                        "target": {"time": target.0, "class": target_class}})
            },
        )
        .collect::<Vec<_>>()
        .into();

    Ok(json!({"rows": json_row_data, "footer": footer_json}))
}

pub fn wr_to_values(wr: &Option<WR>) -> (i32, i32, Time, String) {
    if let Some(WR {
        table,
        lev,
        time,
        ref kuski,
    }) = *wr
    {
        (table, lev, time, kuski.to_string())
    } else {
        (0, 0, Time(0), "".to_owned())
    }
}

fn compute_tts(drs: &[DataRow]) -> (Time, Time) {
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

fn collect_last_wr_table(wr_tables: &[WR]) -> Vec<WR> {
    let last_table_num = wr_tables.iter().last().unwrap().table;
    wr_tables
        .iter()
        .filter(|x| x.table == last_table_num)
        .cloned()
        .collect()
}

fn collect_current_wrs(prs: &[Time], cur_wrt: &[WR]) -> Vec<Time> {
    prs.iter()
        .zip(cur_wrt.iter())
        .map(|(x, y)| *x.min(&y.time))
        .collect()
}

fn get_time_class(time: &Time, targets: &Targets, current_wr: &Time) -> String {
    match *time {
        t if t > targets.beginner => "unclassified",
        t if t > targets.ok => "beginner",
        t if t > targets.good => "ok",
        t if t > targets.professional => "good",
        t if t > targets.world_class => "professional",
        t if t > targets.legendary => "world_class",
        t if t > targets.godlike => "legendary",
        t if t > *current_wr => "godlike",
        _ => "wr",
    }.to_string()
}

pub fn get_next_target(time: &Time, target: &Targets, current_wr: &Time) -> Time {
    match *time {
        t if t > target.beginner => target.beginner,
        t if t > target.ok => target.ok,
        t if t > target.good => target.good,
        t if t > target.professional => target.professional,
        t if t > target.world_class => target.world_class,
        t if t > target.legendary => target.legendary,
        t if t > target.godlike => target.godlike,
        t if t > *current_wr => *current_wr,
        _ => *current_wr,
    }
}
