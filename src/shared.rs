use elma::Time;
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub struct Targets {
    pub godlike: Time,
    pub legendary: Time,
    pub world_class: Time,
    pub professional: Time,
    pub good: Time,
    pub ok: Time,
    pub beginner: Time,
}

impl Add for Targets {
    type Output = Targets;

    fn add(self, other: Targets) -> Targets {
        Targets {
            godlike: self.godlike + other.godlike,
            legendary: self.legendary + other.legendary,
            world_class: self.world_class + other.world_class,
            professional: self.professional + other.professional,
            good: self.good + other.good,
            ok: self.ok + other.ok,
            beginner: self.beginner + other.beginner,
        }
    }
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
    use self::SortBy::*;
    use self::SortOrder::*;
    match sort_param {
        "PR" => PR(if ascending { Ascending } else { Descending }),
        "DiffToPrevWR" => DiffToPrevWR(if ascending { Ascending } else { Descending }),
        "DiffToNextWR" => DiffToNextWR(if ascending { Ascending } else { Descending }),
        "DiffToNextTarget" => DiffToNextTarget(if ascending { Ascending } else { Descending }),
        "LevelNum" => LevelNum(if ascending { Ascending } else { Descending }),
        "Table" => Table(if ascending { Ascending } else { Descending }),
        &_ => LevelNum(Ascending),
    }
}

#[allow(dead_code)]
pub fn time_to_diff_string(t: Time) -> String {
    let (negative, h, m, s, hd) = t.to_parts();
    let sign = if negative { "-" } else { "+" };

    match (h, m, s, hd) {
        (0, 0, 0, hd) => format!("{sign}0,{hd:02}", sign = sign, hd = hd),
        (0, 0, s, hd) => format!("{sign}{s},{hd:02}", sign = sign, s = s, hd = hd),
        (0, m, s, hd) => format!(
            "{sign}{m}:{s:02},{hd:02}",
            sign = sign,
            m = m,
            s = s,
            hd = hd
        ),
        (h, m, s, hd) => format!(
            "{sign}{h}:{m:02}:{s:02},{hd:02}",
            sign = sign,
            h = h,
            m = m,
            s = s,
            hd = hd
        ),
    }
}
