extern crate reqwest;

use failure::Error;
use std::fs::File;
use std::io::Write;

pub fn download_wr_tables() -> Result<(), Error> {
    let data = reqwest::get("http://ldev.no/wr-stats/data-v1.csv")?.text()?;
    let mut buffer = File::create("wr-stats_tables.csv")?;
    Ok(buffer.write_all(data.as_bytes())?)
    
}

pub fn download_targets() -> Result<(), Error> {
    let data = reqwest::get("http://ldev.no/wr-stats/wr-stats_targets.csv")?.text()?;
    let mut buffer = File::create("wr-stats_targets.csv")?;
    Ok(buffer.write_all(data.as_bytes())?)
}
