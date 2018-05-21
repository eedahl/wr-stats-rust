use elma::Time;
use failure::Error;
use serde_json;

use model::Model;
use model::ModelSelects;
use shared::{DataRow, SortBy, SortOrder, WR};

#[allow(dead_code)]
pub fn get_level_update_data(model: &Model, level: i32) -> Result<serde_json::Value, Error> {
    let targets = model.get_targets((level - 1) as usize);

    Ok(json!({"level": level, "times": serde_json::to_value(
        model.collect_wrs_for_lev((level-1) as usize))?,
    "pr": model.get_pr((level-1) as usize).0,
    "targets": {
        "godlike":targets.godlike.0,
        "legendary": targets.legendary.0,
        "world_class": targets.world_class.0,
        "professional": targets.professional.0,
        "good": targets.good.0,
        "ok": targets.ok.0,
        "beginner": targets.beginner.0}}))
}

pub fn build_table_update_data(model: &Model, sort_by: SortBy) -> Result<serde_json::Value, Error> {
    let data = populate_table_data(&model);

    // * Footer
    let (p_tt, target_wr_tt) = compute_tts(&data);
    let target_tt = model.get_targets_tt();
    let footer_json =
        json!({"p_tt": p_tt.0, "target_wr_tt": target_wr_tt.0, "target_tt": target_tt.0});

    // * Body
    let mut data_vec = data
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
                let pr_class = model.get_time_class(pr, i);
                let target = model.get_next_target(pr, i);
                let target_class = if target != Time(0) {
                    model.get_time_class(&target, i)
                } else {
                    "".to_owned()
                };
                let (table_b, _, time_b, kuski_b) = wr_to_values(wr_beat);
                let wr_b_class = model.get_time_class(&time_b, i);
                let (table_nb, _, time_nb, kuski_nb) = wr_to_values(wr_not_beat);
                let wr_nb_class = model.get_time_class(&time_nb, i);
                json!({"lev_number": lev_number,
                        "lev_name": lev_name,
                        "pr" : {"time": pr.0, "class": pr_class},
                        "wr_beat": { "time": time_b.0, "class": wr_b_class, "table": table_b, "kuski": kuski_b },
                        "wr_not_beat": { "time": time_nb.0, "class": wr_nb_class, "table": table_nb, "kuski": kuski_nb },
                        "target": {"time": target.0, "class": target_class}})
            },
        )
        .collect::<Vec<serde_json::Value>>();

    sort(&mut data_vec, sort_by).expect("Error while sorting");
    let json_row_data: serde_json::Value = data_vec.into();

    Ok(json!({"rows": json_row_data, "footer": footer_json}))
}

fn sort(data: &mut Vec<serde_json::Value>, sort_by: SortBy) -> Result<(), Error> {
    match sort_by {
        SortBy::Table(ord) => data.sort_by(|x, y| {
            let table1: i32 = serde_json::from_value(x["wr_beat"]["table"].clone()).unwrap();
            let table2: i32 = serde_json::from_value(y["wr_beat"]["table"].clone()).unwrap();
            match ord {
                SortOrder::Ascending => table1.cmp(&table2),
                SortOrder::Descending => table2.cmp(&table1),
            }
        }),
        SortBy::PR(ord) => data.sort_by(|x, y| {
            let pr1: i32 = serde_json::from_value(x["pr"]["time"].clone()).unwrap();
            let pr2: i32 = serde_json::from_value(y["pr"]["time"].clone()).unwrap();
            match ord {
                SortOrder::Ascending => pr1.cmp(&pr2),
                SortOrder::Descending => pr2.cmp(&pr1),
            }
        }),
        SortBy::DiffToNextTarget(ord) => data.sort_by(|x, y| {
            let pr1: i32 = serde_json::from_value(x["pr"]["time"].clone()).unwrap();
            let tar1: i32 = serde_json::from_value(x["target"]["time"].clone()).unwrap();
            let pr2: i32 = serde_json::from_value(y["pr"]["time"].clone()).unwrap();
            let tar2: i32 = serde_json::from_value(y["target"]["time"].clone()).unwrap();
            match ord {
                SortOrder::Ascending => (pr1 - tar1).cmp(&(pr2 - tar2)),
                SortOrder::Descending => (pr2 - tar2).cmp(&(pr1 - tar1)),
            }
        }),
        SortBy::DiffToPrevWR(ord) => data.sort_by(|x, y| {
            let pr1: i32 = serde_json::from_value(x["pr"]["time"].clone()).unwrap();
            let wr1: i32 = serde_json::from_value(x["wr_beat"]["time"].clone()).unwrap();
            let pr2: i32 = serde_json::from_value(y["pr"]["time"].clone()).unwrap();
            let wr2: i32 = serde_json::from_value(y["wr_beat"]["time"].clone()).unwrap();
            match ord {
                SortOrder::Ascending => (pr1 - wr1).cmp(&(pr2 - wr2)),
                SortOrder::Descending => (pr2 - wr2).cmp(&(pr1 - wr1)),
            }
        }),
        SortBy::DiffToNextWR(ord) => data.sort_by(|x, y| {
            let pr1: i32 = serde_json::from_value(x["pr"]["time"].clone()).unwrap();
            let wr1: i32 = serde_json::from_value(x["wr_not_beat"]["time"].clone()).unwrap();
            let pr2: i32 = serde_json::from_value(y["pr"]["time"].clone()).unwrap();
            let wr2: i32 = serde_json::from_value(y["wr_not_beat"]["time"].clone()).unwrap();
            match ord {
                SortOrder::Ascending => (pr1 - wr1).cmp(&(pr2 - wr2)),
                SortOrder::Descending => (pr2 - wr2).cmp(&(pr1 - wr1)),
            }
        }),
        SortBy::LevelNum(ord) => match ord {
            SortOrder::Ascending => {}
            SortOrder::Descending => data.sort_by(|x, y| {
                let lev_num1: i32 = serde_json::from_value(x["lev_number"].clone()).unwrap();
                let lev_num2: i32 = serde_json::from_value(y["lev_number"].clone()).unwrap();
                lev_num2.cmp(&lev_num1)
            }),
        },
    }
    Ok(())
}

pub fn populate_table_data(model: &Model) -> Vec<DataRow> {
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
            let level = i as i32 + 1;
            let pr = model.get_pr(i);
            let last_wr_beat = model.get_last_wr_beat(&pr, i);
            let first_wr_not_beat = model.get_first_wr_not_beat(&pr, i);

            DataRow {
                lev_number: level,
                lev_name: lev_name.to_string(),
                pr: pr,
                wr_beat: last_wr_beat.cloned(),
                wr_not_beat: first_wr_not_beat.cloned(),
            }
        })
        .collect()
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
