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
