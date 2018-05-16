//#![windows_subsystem = "windows"]
extern crate elma;
extern crate failure;
extern crate notify;
extern crate web_view;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

mod html;
mod http;
mod io;
mod shared;

use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::thread::spawn;
use std::time::Duration;
use web_view::WebView;

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
#[serde(tag = "cmd")]
enum Cmd {
    updateSorted { param: String, ascending: bool },
    // * Admissible commands go here
}

//TODO(edahl): refactor sorting rust-side
//TODO(edahl): colour tts
//TODO(edahl): multiple pages, like say you want to see the development in a lev over all wr tables
// ? a better data structure
// ? functionality to browse WR tables
// ? browse targets
// ? a side pane that can possibly be collapsable, should be simple JS
// ? --- it's already a lot of into in one screen, so maybe some selects like show only target times,
// ? show only target wrs or whatever, or very simple starting view with just your times
// ? and then click on a level to see all the info for that level
// ? --- letting you left click a level to see a graph, kinda like the improvements tab in elma stats,
// ? that show the development from table 1 to whichever is the last one
// ? together with info like targets, etc., that you would expect for that one level
// ? and a horizontal line showing your current time
// ? --- seems that a left pane would be mostly extra clutter
// ? an alternative would be a header with "Overview" and "Additional stats" that lets you
// ? click between the table and additional stats like different tts and table counts
// ? together with the more focused pr level pane
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

    let html = match html::build_initial_html(&wr_tables, &targets_table) {
        Ok(h) => h,
        Err(e) => html::default_error_message(e),
    };

    let size = (1000, 1000);
    let resizable = true;
    let debug = true;
    let userdata = ();

    web_view::run(
        "WR Stats",
        web_view::Content::Html(html),
        Some(size),
        resizable,
        debug,
        // * Init
        |webview| {
            spawn(move || {
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
                                webview.eval(&format!("updateSorted();"));
                            });
                        }
                        Ok(_event) => (),
                        Err(e) => println!("Error while watching state.dat: {:?}", e),
                    }
                }
            });
        },
        // * Frontend
        move |webview, arg, _userdata: &mut _| {
            use Cmd::*;
            match serde_json::from_str(arg).unwrap() {
                updateSorted { param, ascending } => {
                    let sort_by = shared::get_sort_hint(&param, ascending);
                    let (ref rows, ref footer) =
                        match shared::build_update_data(&wr_tables, &targets_table, sort_by) {
                            Ok((rows, footer)) => (rows, footer),
                            Err(e) => (html::default_error_message(e), String::from("")),
                        };
                    update_table_rows(webview, &rows);
                    update_table_footer(webview, &footer);
                }
            }
        },
        userdata,
    );
}

fn update_table_rows<'a, T>(webview: &mut WebView<'a, T>, rows: &str) {
    webview.eval(&format!("updateTableRows({});", web_view::escape(rows)));
}

fn update_table_footer<'a, T>(webview: &mut WebView<'a, T>, footer: &str) {
    webview.eval(&format!("updateTableFooter({});", web_view::escape(footer)));
}
