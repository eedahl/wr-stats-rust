extern crate csv;
extern crate elma;
extern crate web_view;

use web_view::*;

//use elma::state::*;
//let state = State::load("state.dat").unwrap();

mod time;
use time::Time;
mod html;
mod io;

#[derive(Clone)]
pub struct WR {
    table: i32,
    lev: i32,
    time: Time,
    kuski: String,
}

pub struct Targets {
    godlike: Time,
    legendary: Time,
    world_class: Time,
    professional: Time,
    good: Time,
    ok: Time,
    beginner: Time,
}

pub struct DataRow {
    lev_number: i32,
    lev_name: String,
    pr: Time,
    wr_beat: Option<WR>,
    wr_not_beat: Option<WR>,
}

//TODO(edahl): store time as single int and use display/format for string
//TODO(edahl): fix time methods to account for single int storage
//TODO(edahl): read lev names from a file

fn main() {
    let data = io::populate_table_data();
    let data_alt = io::populate_table_data_alt();

    let targets_table = io::read_targets_table();

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
    html_table.push_str(&html::inline_tr(html::table_header(headers.clone())));
    let mut html_table_alt = String::new();
    html_table_alt.push_str(&html::inline_tr(html::table_header(headers.clone())));

    for (i, r) in data_alt.iter().enumerate() {
        html_table_alt.push_str("<tr>");
        html_table_alt.push_str(&html::table_data_s(&r.lev_number.to_string()));
        html_table_alt.push_str(&html::table_data_s(&r.lev_name));
        html_table_alt.push_str(&html::time_to_tagged_td(&r.pr, &targets_table[i]));
        
        if let Some(wr) = r.wr_beat.clone() {
            html_table_alt.push_str(&html::table_data_s(&wr.table.to_string()));
            html_table_alt.push_str(&html::time_to_tagged_td(&wr.time, &targets_table[i]));
            html_table_alt.push_str(&html::table_data_s(&wr.kuski));
        } else {
            html_table_alt.push_str(&html::table_data_s(&"-".to_string()));
            html_table_alt.push_str(&html::table_data_s(&"-".to_string()));
            html_table_alt.push_str(&html::table_data_s(&"-".to_string()));
        }

        if let Some(wr) = r.wr_not_beat.clone() {
            html_table_alt.push_str(&html::time_to_tagged_td(&wr.time, &targets_table[i]));
            html_table_alt.push_str(&html::time_to_diff(&time::difference(&r.pr, &wr.time)));
            html_table_alt.push_str(&html::table_data_s(&wr.kuski));
        } else {
            html_table_alt.push_str(&html::table_data_s(&"-".to_string()));
            html_table_alt.push_str(&html::table_data_s(&"-".to_string()));
            html_table_alt.push_str(&html::table_data_s(&"-".to_string()));
        }

        html_table_alt.push_str("</tr>");
    }

    for r in data {
        html_table.push_str(&html::inline_tr(html::table_data(r)));
    }

    html_table = html::inline_table(html_table);
    html_table_alt = html::inline_table(html_table_alt);


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
        table = html_table_alt
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
