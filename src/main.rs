#![windows_subsystem = "windows"]
extern crate elma;
extern crate failure;
extern crate notify;
extern crate web_view;

mod html;
mod http;
mod io;
mod shared;

use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use shared::SortBy;
use std::sync::mpsc::channel;
use std::time::Duration;
use web_view::WebView;

//TODO(edahl): table sorting -- js?
//TODO(edahl): functionality to browse WR tables
//TODO(edahl): browse targets
//TODO(edahl): read lev names from a file
//TODO(edahl): would appreciate if writen how many wrs in table #001 tabel #050 100 150 200 250 300 350 400 osv
//if you have times in 55 and 56, it should be counted for 50. and if i have times on 100, it be counted for table 1
/*
struct TableData {
    data: Vec<DataRow>,
    targets: Vec<Targets>,
}
*/

fn main() {
    http::download_wr_tables().unwrap_or_else(|e| {
        println!("Error updating WR tables: {:?}", e);
    });
    http::download_targets().unwrap_or_else(|e| {
        println!("Error getting targets table: {:?}", e);
    });

    let wr_tables = io::load_wr_tables().unwrap_or_else(|e| {
        println!("Error loading WR tables: {:?}", e);
        Vec::new()
    });

    let targets_table = match io::load_targets_table() {
        Ok(tt) => tt,
        Err(e) => {
            html::default_error_message(e);
            Vec::new()
        }
    };

    let html = match shared::build_html(&wr_tables, &targets_table) {
        Ok(h) => h,
        Err(e) => html::default_error_message(e),
    };

    let size = (960, 1020);
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
                            let tables = match shared::build_tables(&wr_tables, &targets_table, SortBy::DiffToNextWRA) {
                                Ok(h) => h,
                                Err(e) => html::default_error_message(e),
                            };

                            webview.dispatch(move |webview, _userdata| {
                                update_tables(webview, &tables);
                            });
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

    /*|webview, arg, tasks: &mut Vec<Task>| {
		use Cmd::*;
		match serde_json::from_str(arg).unwrap() {
			init => (),
			log { text } => println!("{}", text),
			addTask { name } => tasks.push(Task { name, done: false }),
			markTask { index, done } => tasks[index].done = done,
			clearDoneTasks => tasks.retain(|t| !t.done),
		}
		render(webview, tasks);
	}*/
}

fn update_tables<'a, T>(webview: &mut WebView<'a, T>, tables: &str) {
    webview.eval(&format!(
        "$('#tables_container').html({});",
        web_view::escape(tables)
    ));
}

