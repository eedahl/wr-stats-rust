//#![windows_subsystem = "windows"]
#![feature(proc_macro)]
#![feature(proc_macro_non_items)]
extern crate elma;
extern crate failure;
extern crate maud;
extern crate notify;
extern crate web_view;
#[macro_use]
extern crate serde_derive;
extern crate serde;
#[macro_use]
extern crate serde_json;

mod cont;
mod html;
mod http;
mod model;
mod shared;
mod templ;

use model::Model;

use notify::{DebouncedEvent, RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::thread::spawn;
use std::time::Duration;
use web_view::WebView;

//TODO(edahl): refactor sorting rust-side
//TODO(edahl): colour tts
//TODO(edahl): multiple pages, like say you want to see the development in a lev over all wr tables
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
// ? an alternative would be a header with "Overview" and "Additional stats" that lets you
// ? click between the table and additional stats like different tts and table counts
/*so have header of PR be a dropdown menu of like "PR, avg" etc and you choose what to display to compare against
and then have a stat column where you choose like
"wr stats"
"top 10 table"
"target times"
but you click the target WR column to expand/collapse
that's the functionality explanation, here's the new feature
take the same idea, make the "PR" column clickable
and expandong to(edited)
http://stats.sshoyer.net/player.php?player=Haruhi
top 10 table
in same way as elmastats has it
here's the part i haven't worked out
reason for top 10 stats is
so can compare to WR tables in same way as PR is currently
ie
what tables does my 10th time in int06 beat?
what tables does my avg in int35 beat
etc
Also there used to be some interest long ago in TT of 2nd times in top 10, TT of 3rd times in top 10 etc.
but afaik those programs to calculate it are gone
can do on elmastats if coded ofc
but may as well add to your program, if want*/
/*
? ideas for data
TT of current WRs
TT if times at least beginner
TT if times at least ok
TT if times at least good
TT if times at least professional
TT if times at least world class
TT if times at least legendary
TT if times at least godlike
Number of times past table 1
Number of times past table 50
Number of times past table 100
Number of times past table 150
Number of times past table 200
Number of times past table 250
Number of times past table 300
Number of times past table 350
Number of times past table 400
Your closest targets are: 18. Spiral (+,01), 3. Twin Peaks (+,01) etc.
Worst differences to see where need to improve a lot
Graph for tt
*/

fn main() {
    http::download_wr_tables().unwrap_or_else(|e| {
        println!("Error updating WR tables: {:?}", e);
    });
    http::download_targets().unwrap_or_else(|e| {
        println!("Error getting targets table: {:?}", e);
    });

    let mut model = Model::new().expect("Could not create model.");

    let html = html::index();
    let size = (1000, 925);
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
                                webview.eval(&format!("views.updateView();"));
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
                    "table" => display_view(webview, "table", &html::table_view()),
                    "level" => display_view(webview, "level", &html::level_view()),
                    "tt" => display_view(webview, "tt", &html::tt_view()),
                    v => println!("View in display request not recognised: {}", v),
                },
                updateView { view, arg } => match view.as_ref() {
                    "table" => {
                        let ascending: bool =
                            serde_json::from_value(arg["ascending"].clone()).unwrap();
                        let param: String = serde_json::from_value(arg["param"].clone()).unwrap();
                        let sort_by = shared::get_sort_hint(&param, ascending);

                        model.update_pr_table().expect("Failed to update PR table.");

                        let data = cont::build_table_update_data(&model, sort_by).unwrap();
                        update_view(webview, "table", data);
                    }
                    "level" => {
                        let level: i32 = serde_json::from_value(arg["level"].clone()).unwrap();

                        model.update_pr_table().expect("Failed to update PR table.");

                        let data = cont::get_level_update_data(&model, level).unwrap();
                        update_view(webview, "level", data);
                    }
                    "tt" => {
                        model.update_pr_table().expect("Failed to update PR table.");

                        let data = cont::get_tt_update_data(&model).unwrap();
                        update_view(webview, "tt", data);
                    }
                    v => println!("View in update request not recognised: {}", v),
                },
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
    displayView {
        view: String,
    },
    updateView {
        view: String,
        arg: serde_json::Value,
    },
    log {
        text: String,
    },
    // * Admissible commands go here
}

fn display_view<'a, T>(webview: &mut WebView<'a, T>, view: &str, template: &str) {
    webview.eval(&format!(
        "views.display({});",
        web_view::escape(&json!({ "view": view, "template": template, }).to_string()),
    ));
}

fn update_view<'a, T>(webview: &mut WebView<'a, T>, view: &str, data: serde_json::Value) {
    webview.eval(&format!(
        "views.updateView({})",
        web_view::escape(&json!({ "view": view, "data": data}).to_string()),
    ));
}
