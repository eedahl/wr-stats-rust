extern crate csv;
extern crate elma;
extern crate web_view;

use web_view::*;

use std::io::prelude::*;

//use elma::state::*;
//let state = State::load("state.dat").unwrap();

mod time;
use time::Time;
mod html;
mod io;

struct WR {
    table: i32,
    lev: i32,
    time: Time,
    kuski: String,
}

struct Targets {
    godlike: Time,
    legendary: Time,
    world_class: Time,
    professional: Time,
    good: Time,
    ok: Time,
    beginner: Time,
}

//TODO(edahl): store time as single int and use display/format for string
//TODO(edahl): fix time methods to account for single int storage
//TODO(edahl): read lev names from a file

fn main() {
    let data = io::populate_table_data();

    let headers = vec![
        "Lev".to_string(),
        "Name".to_string(),
        "PR".to_string(),
        "Table beat".to_string(),
        "Time beat".to_string(),
        "Kuski beat".to_string(),
        "Target".to_string(),
        "Diff".to_string(),
        "Kuski to beat".to_string(),
    ];
    let mut html_table = String::new();
    html_table.push_str(&html::inline_tr(html::table_header(headers)));

    for r in data {
        html_table.push_str(&html::inline_tr(html::table_data(r)));
    }

    html_table = html::inline_table(html_table);

    let html = format!(
        r#"
            <!doctype html>
            <html>
                <head>
                    {styles}
                </head>
                <body>
                    {table}
                </body>
            </html>
            "#,
        styles = html::inline_style(include_str!("styles.css")),
        table = html_table
    );

    //TODO?(edahl): <link rel=\"stylesheet\" type=\"text/css\" href=\"/styles.css\">

    let size = (900, 820);
    let resizable = true;
    let debug = true;
    let init_cb = |_webview| {};
    let frontend_cb = |_webview: &mut _, _arg: &_, _userdata: &mut _| {};
    let userdata = ();
    run(
        "WR-stats",
        Content::Html(html),
        Some(size),
        resizable,
        debug,
        init_cb,
        frontend_cb,
        userdata,
    );
}
