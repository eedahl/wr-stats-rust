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
pub enum Cmd {
    sort { param: String, ascending: bool },
    // * Admissible commands go here
}

//TODO(edahl): refactor sorting rust-side
//TODO(edahl): colour tts
//TODO(edahl): a better data structure
//TODO(edahl): multiple pages, like say you want to see the development over all wr tables
//TODO(edahl): functionality to browse WR tables
//TODO(edahl): browse targets
//TODO(edahl): a side pane that can possibly be collapsable, should be simple JS, with:
//TODO --- how many wrs in table #001 tabel #050 100 150 200 250 300 350 400 osv
//TODO if you have times in 55 and 56, it should be counted for 50. and if i have times on 100, it be counted for table 1
//TODO --- your tt if all your times were at least beginner, ok, good, pro, etc.
//TODO --- always show the closest targets (independently of ordering), maybe at the bottom of the list,
//TODO something like "Your closest targets are: 18. Spiral (+,01), 3. Twin Peaks (+,01)" etc
//TODO also can show worst differences to see where need to improve a lot
//TODO --- like the improvements tab on elmastats, you can click on a level and it takes you there
//TODO nice thing would be to have one app that can do everything, meaning add target times in yours as well
//TODO ---- it's already a lot of into in one screen, so maybe some selects like show only target times,
//TODO show only target wrs or whatever, or very simple starting view with just your times
//TODO and then click on a level to see all the info for that level
//TODO --- letting you left click a level to see a graph, kinda like the improvements tab in elma stats,
//TODO that show the development from table 1 to whichever is the last one
//TODO together with info like targets, etc., that you would expect for that one level
//TODO and a horizontal line showing your current time
//TODO --- but kinda what i'm getting is that the  left pane would be mostly extra clutter
//TODO an alternative would be a header with "Overview" and "Additional stats" that lets you
//TODO click between the table and additional stats like different tts and table counts
//TODO together with the more focused pr level pane
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

    let html = match shared::build_initial_html(&wr_tables, &targets_table) {
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
                                webview.eval(&format!("sortUpdate();"));
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
                sort { param, ascending } => {
                    ////println!("param: {:?}, ascending: {:?}", &param, ascending);
                    let sort_hint = shared::get_sort_hint(&param, ascending);

                    let (ref rows, ref sidebar) =
                        match shared::build_update_data(&wr_tables, &targets_table, sort_hint) {
                            Ok((rows, sidebar)) => (rows, sidebar),
                            Err(e) => (html::default_error_message(e), String::from("")),
                        };
                    update_table(webview, &rows);
                    update_sidebar(webview, &sidebar);
                }
            }
        },
        userdata,
    );
}

fn update_table<'a, T>(webview: &mut WebView<'a, T>, rows: &str) {
    webview.eval(&format!("updateTable({});", web_view::escape(rows)));
}

fn update_sidebar<'a, T>(webview: &mut WebView<'a, T>, sidebar: &str) {
    webview.eval(&format!("updateSidebar({});", web_view::escape(sidebar)));
}

////#[derive(Debug, Serialize, Deserialize)]
////struct SortHint {
////	param: String,
////	ascending: bool,
