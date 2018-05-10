#![windows_subsystem = "windows"]
extern crate elma;
extern crate notify;
extern crate web_view;
extern crate serde;

#[macro_use]
extern crate serde_derive;

use elma::Time;
use web_view::WebView;
use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

mod html;
mod io;

/*
mod elma {
    pub struct Time {
        pub time: i32,
    }
}
*/
#[derive(Deserialize)]
#[serde(remote = "Time")]
struct TimeDef(i32);

#[derive(Deserialize)]
pub struct Targets {
    #[serde(with = "TimeDef")]
    godlike: Time,
    #[serde(with = "TimeDef")]
    legendary: Time,
    #[serde(with = "TimeDef")]
    world_class: Time,
    #[serde(with = "TimeDef")]
    professional: Time,
    #[serde(with = "TimeDef")]
    good: Time,
    #[serde(with = "TimeDef")]
    ok: Time,
    #[serde(with = "TimeDef")]
    beginner: Time,
}

#[derive(Clone)]
pub struct WR {
    table: i32,
    lev: i32,
    time: Time,
    kuski: String,
}


pub struct DataRow {
    lev_number: i32,
    lev_name: String,
    pr: Time,
    wr_beat: Option<WR>,
    wr_not_beat: Option<WR>,
}

//TODO(edahl): table sorting -- js?
//TODO(edahl): stats.txt fallback?
//TODO(edahl): functionality to browse WR tables
//TODO(edahl): browse targets
//TODO(edahl): read lev names from a file
//TODO(edahl): table order
//I think it would be more intuitive to have it your time -> beated table -> table to beat next
fn main() {
    let wr_tables = io::load_wr_tables();
    let targets_table = io::load_targets_table();

    let pr_table = io::load_state().expect("Could not load file: state.dat");
    let html = build_html(&pr_table, &wr_tables, &targets_table);

    let size = (960, 925);
    let resizable = true;
    let debug = true;
    let userdata = ();

    web_view::run(
        "WR Stats",
        web_view::Content::Html(html),
        Some(size),
        resizable,
        debug,
        move |webview| {
            std::thread::spawn(move || {
                let (tx, rx) = channel();
                let mut watcher: RecommendedWatcher =
                    Watcher::new(tx, Duration::from_secs(1)).unwrap();
                watcher
                    .watch("state.dat", RecursiveMode::NonRecursive)
                    .unwrap();
                loop {
                    match rx.recv() {
                        Ok(DebouncedEvent::Write(_path)) => {
                            if let Ok(pr_table) = io::load_state() {
                                let html = build_html(&pr_table, &wr_tables, &targets_table);

                                webview.dispatch(move |webview, _userdata| {
                                    update_html(webview, &html);
                                });
                            }
                        }
                        Ok(_event) => (),
                        Err(e) => println!("Error while watching state.dat: {:?}", e),
                    }
                }
            });
        },
        |_webview: &mut _, _arg: &_, _userdata: &mut _| {},
        userdata,
    );
}

fn update_html<'a, T>(webview: &mut WebView<'a, T>, html: &str) {
    webview.eval(&format!(
        "document.documentElement.innerHTML={};",
        web_view::escape(html)
    ));
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

fn compute_tts(drs: &[DataRow]) -> (elma::Time, elma::Time) {
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

fn build_html(pr_table: &[Time], wr_tables: &[WR], targets_table: &[Targets]) -> String {
    let data = io::populate_table_data(&pr_table, &wr_tables);
    let last_wr_table = get_last_wr_table(&wr_tables);
    let current_wrs = collect_current_wrs(&pr_table, &last_wr_table);
    let html_table = html::create_html_table(&data, &targets_table, &current_wrs);
    let (p_tt, t_tt) = compute_tts(&data);
    html::create_html(&html_table, &p_tt, &t_tt)
}