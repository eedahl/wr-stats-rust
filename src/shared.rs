use elma::Time;

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
