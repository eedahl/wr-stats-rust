use elma::Time;
use failure::Error;
use html;
use io;

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
    DiffToNextWRA,
    DiffToNextTargetA,
    LevelNumA,
    DiffToNextWRD,
    DiffToNextTargetD,
    LevelNumD,
    TableA,
    TableD,
}

pub fn build_html(wr_tables: &[WR], targets_table: &[Targets]) -> Result<String, Error> {
    let tables = build_tables(wr_tables, targets_table, SortBy::LevelNumA)?;
    Ok(html::format_html(&tables))
}

pub fn build_tables(
    wr_tables: &[WR],
    targets_table: &[Targets],
    sort_by: SortBy,
) -> Result<String, Error> {
    let pr_table = match io::load_state() {
        Ok(t) => t,
        Err(_) => io::read_stats()?,
    };

    let last_wr_table = get_last_wr_table(&wr_tables);
    let current_wrs = collect_current_wrs(&pr_table, &last_wr_table);

    let data = io::populate_table_data(&pr_table, &wr_tables);
    let (p_tt, t_tt) = compute_tts(&data);

    let mut collate = data.into_iter()
        .zip(targets_table.into_iter().cloned())
        .zip(current_wrs.into_iter())
        .collect::<Vec<((DataRow, Targets), Time)>>();

    match sort_by {
        SortBy::TableA => collate.sort_by(|x, y| {
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
            table1.cmp(&table2)
        }),
        SortBy::TableD => collate.sort_by(|x, y| {
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
            table2.cmp(&table1)
        }),
        SortBy::DiffToNextTargetA => collate.sort_by(|x, y| {
            let pr1 = ((x.0).0).pr;
            let tars1 = &(x.0).1;
            let cur_wr1 = x.1;
            let tar1 = get_next_target(&pr1, tars1, &cur_wr1);
            let pr2 = ((y.0).0).pr;
            let tars2 = &(y.0).1;
            let cur_wr2 = y.1;
            let tar2 = get_next_target(&pr2, tars2, &cur_wr2);
            (pr1 - tar1).cmp(&(pr2 - tar2))
        }),
        SortBy::DiffToNextTargetD => collate.sort_by(|x, y| {
            let pr1 = ((x.0).0).pr;
            let tars1 = &(x.0).1;
            let cur_wr1 = x.1;
            let tar1 = get_next_target(&pr1, tars1, &cur_wr1);
            let pr2 = ((y.0).0).pr;
            let tars2 = &(y.0).1;
            let cur_wr2 = y.1;
            let tar2 = get_next_target(&pr2, tars2, &cur_wr2);
            (pr2 - tar2).cmp(&(pr1 - tar1))
        }),
        SortBy::DiffToNextWRA => collate.sort_by(|x, y| {
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
            (pr1 - wr1).cmp(&(pr2 - wr2))
        }),
        SortBy::DiffToNextWRD => collate.sort_by(|x, y| {
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
            (pr2 - wr2).cmp(&(pr1 - wr1))
        }),
        SortBy::LevelNumD => collate.sort_by(|x, y| {
            let lev_num1 = ((x.0).0).lev_number;
            let lev_num2 = ((y.0).0).lev_number;
            lev_num2.cmp(&lev_num1)
        }),
        SortBy::LevelNumA => { /* Default */ }
    }
    let (unpack, wrs_sorted): (Vec<(DataRow, Targets)>, Vec<Time>) = collate.into_iter().unzip();
    let (data_sorted, targets_sorted): (Vec<DataRow>, Vec<Targets>) = unpack.into_iter().unzip();

    Ok(html::format_tables(
        &html::create_wr_table(&data_sorted, &targets_sorted, &wrs_sorted),
        &p_tt,
        &t_tt,
    ))
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

fn collect_current_wrs(prs: &[Time], cur_wrt: &[WR]) -> Vec<Time> {
    prs.iter()
        .zip(cur_wrt.iter())
        .map(|(x, y)| *x.min(&y.time))
        .collect()
}

fn get_last_wr_table(wr_tables: &[WR]) -> Vec<WR> {
    let last_table = wr_tables.iter().last().unwrap().table;
    let current_wrs: Vec<WR> = wr_tables
        .iter()
        .filter(|x| x.table == last_table)
        .cloned()
        .collect();
    current_wrs
}

/*
pub fn time_to_short_string(t: &Time) -> String {
    (h, m, s, hd) = t.to_parts();

10:00:00,00

    format!(
        "{sign}{hr}{m}{s:02}{hd:02}",
        sign = if t.0 < 0 { "-" } else { "" },
        hr = if h > 0 { format!("{}:", h) } else { "".into() },
        m = if m > 0 { format!("{}:", m) } else { "".into() },
        s = if s > 0 { format!("{},", s) } else { "".into() },
        hd = hd
    )
}
*/
