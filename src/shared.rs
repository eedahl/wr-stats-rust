use elma::Time;
use failure::Error;
use html;
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

pub enum SortBy {
    PR(SortOrder),
    DiffToPrevWR(SortOrder),
    DiffToNextWR(SortOrder),
    DiffToNextTarget(SortOrder),
    LevelNum(SortOrder),
    Table(SortOrder),
}

pub enum SortOrder {
    Ascending,
    Descending,
}

pub fn get_next_target(t: &Time, tar: &Targets, cur_wr: &Time) -> Time {
    match *t {
        t if t > tar.beginner => tar.beginner,
        t if t > tar.ok => tar.ok,
        t if t > tar.good => tar.good,
        t if t > tar.professional => tar.professional,
        t if t > tar.world_class => tar.world_class,
        t if t > tar.legendary => tar.legendary,
        t if t > tar.godlike => tar.godlike,
        t if t > *cur_wr => *cur_wr,
        _ => *cur_wr,
    }
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

pub fn build_level_update_data(wr_tables: &[WR], level: i32) -> Result<serde_json::Value, Error> {
    Ok(serde_json::to_value(
        wr_tables
            .into_iter()
            .filter(|x| (*x).lev == level)
            .map(|x| x.time.0)
            .collect::<Vec<_>>(),
    )?)
}

pub fn build_table_update_data(
    wr_tables: &[WR],
    targets_table: &[Targets],
    sort_by: SortBy,
) -> Result<(String, String), Error> {
    let pr_table = match io::load_state() {
        Ok(t) => t,
        Err(_) => io::load_stats()?,
    };

    let data = io::populate_table_data(&pr_table, &wr_tables);
    let last_wr_table = collect_last_wr_table(&wr_tables);
    let current_wrs = collect_current_wrs(&pr_table, &last_wr_table);

    // * Footer
    let (p_tt, target_wr_tt) = compute_tts(&data);
    let mut target_tt = Time(0);
    for (i, pr) in pr_table.iter().enumerate() {
        target_tt = target_tt + get_next_target(&pr, &targets_table[i], &current_wrs[i]);
    }

    let table_footer = html::format_table_footer(&p_tt, &target_wr_tt, &target_tt);

    // * Body
    let mut collate = data.into_iter()
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
    let (unpack, wrs_sorted): (Vec<(DataRow, Targets)>, Vec<Time>) = collate.into_iter().unzip();
    let (data_sorted, targets_sorted): (Vec<DataRow>, Vec<Targets>) = unpack.into_iter().unzip();

    let table_rows = html::create_table_rows(&data_sorted, &targets_sorted, &wrs_sorted);

    Ok((table_rows, table_footer))
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
