#![windows_subsystem = "windows"]
extern crate elma;
extern crate failure;
extern crate notify;
extern crate web_view;
#[macro_use]
extern crate serde_derive;
extern crate crossbeam;
extern crate serde;
extern crate serde_json;

mod html;
mod http;
mod io;
mod shared;

use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;
use web_view::WebView;

//TODO(edahl): refactor sorting rust-side
//TODO(edahl): colour tts
//TODO(edahl): a better data structure
//TODO(edahl): a side pane that can possibly be collapsable, should be simple JS, with:
//would appreciate if writen how many wrs in table #001 tabel #050 100 150 200 250 300 350 400 osv
//if you have times in 55 and 56, it should be counted for 50. and if i have times on 100, it be counted for table 1
//your tt if all your times were at least beginner, ok, good, pro, etc.
//always show the closest targets (independently of ordering), maybe at the bottom of the list,
//something like "Your closest targets are: 18. Spiral (+,01), 3. Twin Peaks (+,01)" etc
//also can show worst differences to see where need to improve a lot
//TODO(edahl): multiple pages, like say you want to see the development over all wr tables
//like the improvements tab on elmastats, you can click on a level and it takes you there
//browse targets
//functionality to browse WR tables

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

    let wr_tables_ref = &wr_tables;
    let targets_table_ref = &targets_table;

    let size = (960, 1020);
    let resizable = true;
    let debug = true;
    let userdata = ();

    crossbeam::scope(|scope| {
        web_view::run(
            "WR Stats",
            web_view::Content::Html(html),
            Some(size),
            resizable,
            debug,
            |webview| {
                scope.spawn(move || {
                    let (tx, rx) = channel();
                    let mut watcher: RecommendedWatcher =
                        Watcher::new(tx, Duration::from_secs(1)).unwrap();
                    watcher
                        .watch("state.dat", RecursiveMode::NonRecursive)
                        .unwrap();
                    loop {
                        match rx.recv() {
                            Ok(DebouncedEvent::Write(_path)) => {
                                webview.dispatch(move |webview, _userdata| {
                                    webview.eval(&format!("sortUpdate();"));
                                });
                            }
                            Ok(_event) => (),
                            Err(e) => println!("Error while watching state.dat: {:?}", e),
                        }
                    }
                });
            },
            |webview, arg, _userdata: &mut _| {
                use Cmd::*;
                match serde_json::from_str(arg).unwrap() {
                    sort { param, ascending } => {
                        println!("param: {:?}, ascending: {:?}", &param, ascending);
                        let sort_hint = shared::get_sort_hint(&param, ascending);
                        let tables =
                            match shared::build_tables(wr_tables_ref, targets_table_ref, sort_hint)
                            {
                                Ok(h) => h,
                                Err(e) => html::default_error_message(e),
                            };
                        update_tables(webview, &tables);
                    }
                }
            },
            userdata,
        );
    });
}

fn update_tables<'a, T>(webview: &mut WebView<'a, T>, tables: &str) {
    webview.eval(&format!("updateTables({});", web_view::escape(tables)));
}
/*
#[derive(Debug, Serialize, Deserialize)]
struct SortHint {
	param: String,
	ascending: bool,
}
*/

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
#[serde(tag = "cmd")]
pub enum Cmd {
    sort { param: String, ascending: bool },
}
