//#![windows_subsystem = "windows"]
extern crate elma;
extern crate failure;
extern crate notify;
extern crate web_view;
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
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

    let targets_table = io::load_targets_table().unwrap_or_else(|e| {
        println!("Error loading targets tables: {:?}", e);
        Vec::new()
    });

    let html = html::index();

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
            spawn(move || -> Result<(), failure::Error> {
                let (tx, rx) = channel();
                let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(1))?;

                watcher.watch("state.dat", RecursiveMode::NonRecursive)?;
                loop {
                    match rx.recv() {
                        Ok(DebouncedEvent::Write(_path)) => {
                            webview.dispatch(move |webview, _userdata| {
                                webview.eval(&format!("rpc.updateTableView();"));
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
                displayView { view } => match view.as_ref() {
                    "table" => display_table_view(webview),
                    "level" => display_level_view(webview),
                    v => println!("View request not recognised: {}", v),
                },
                updateTableView { param, ascending } => {
                    let sort_by = shared::get_sort_hint(&param, ascending);
                    let (ref rows, ref footer) =
                        shared::build_table_update_data(&wr_tables, &targets_table, sort_by)
                            .unwrap_or_else(|err| {
                                (html::default_error_message(err), String::new())
                            });

                    update_table_view(webview, &rows, &footer)
                }
                updateLevelView { level } => {
                    let data = shared::build_level_update_data(&wr_tables, level).unwrap();
                    update_level_view(webview, level, data)
                }
                log { text } => println!("{}", text),
            }
        },
        userdata,
    );
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
#[serde(tag = "cmd")]
enum Cmd {
    displayView { view: String },
    updateTableView { param: String, ascending: bool },
    updateLevelView { level: i32 },
    log { text: String },
    // * Admissible commands go here
}

fn display_table_view<'a, T>(webview: &mut WebView<'a, T>) {
    webview.eval(&format!(
        "views.display({})",
        web_view::escape(&json!({ "view": "table", "template": html::table_view()}).to_string())
    ));
}

fn display_level_view<'a, T>(webview: &mut WebView<'a, T>) {
    webview.eval(&format!(
        "views.display({})",
        web_view::escape(&json!({ "view": "level", "template": html::level_view(), }).to_string()),
    ));
}

fn update_table_view<'a, T>(webview: &mut WebView<'a, T>, rows: &str, footer: &str) {
    webview.eval(&format!(
        "views.update({})",
        web_view::escape(&json!({ "view": "table", "rows": rows, "footer": footer}).to_string()),
    ));
}

fn update_level_view<'a, T>(webview: &mut WebView<'a, T>, level: i32, data: serde_json::Value) {
    /*println!("{}", data.to_string());
    println!(
        "{}",
        json!({ "view": "level", "level": level, "times": data}).to_string()
    );*/
    webview.eval(&format!(
        "views.update({})",
        web_view::escape(&json!({ "view": "level", "level": level, "times": data}).to_string()),
    ));
}
