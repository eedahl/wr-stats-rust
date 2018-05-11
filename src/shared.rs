use elma::Time;

#[derive(Debug)]
pub struct Targets {
    pub godlike: Time,
    pub legendary: Time,
    pub world_class: Time,
    pub professional: Time,
    pub good: Time,
    pub ok: Time,
    pub beginner: Time,
}


#[derive(Clone)]
pub struct WR {
    pub table: i32,
    pub lev: i32,
    pub time: Time,
    pub kuski: String,
}

pub struct DataRow {
    pub lev_number: i32,
    pub lev_name: String,
    pub pr: Time,
    pub wr_beat: Option<WR>,
    pub wr_not_beat: Option<WR>,
}