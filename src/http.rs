extern crate curl;

use std::fs::File;
use std::io::Write;

pub fn download_wr_tables() {
    let mut easy = curl::easy::Easy::new();
    easy.url("http://ldev.no/wr-stats/data-v1.csv").unwrap();
    let mut buffer = File::create("wr-stats_tables.csv").unwrap();
    easy.write_function(move |data| {
        buffer.write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();

    println!("Response code: {}", easy.response_code().unwrap());
}

pub fn download_targets() {
    let mut easy = curl::easy::Easy::new();
    easy.url("http://ldev.no/wr-stats/wr-stats_targets.csv").unwrap();
    let mut buffer = File::create("wr-stats_targets.csv").unwrap();
    easy.write_function(move |data| {
        buffer.write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
    easy.perform().unwrap();

    println!("Response code: {}", easy.response_code().unwrap());
}