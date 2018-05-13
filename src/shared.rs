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