extern crate csv;
extern crate elma;
extern crate htmlescape;
extern crate notify;
extern crate web_view;

use web_view::WebView;

use notify::Watcher;

//use elma::state::*;
//let state = State::load("state.dat").unwrap();

//mod time;
//use time::Time;
mod html;
mod io;

#[derive(Clone)]
pub struct WR {
    table: i32,
    lev: i32,
    time: elma::Time,
    kuski: String,
}

pub struct Targets {
    godlike: elma::Time,
    legendary: elma::Time,
    world_class: elma::Time,
    professional: elma::Time,
    good: elma::Time,
    ok: elma::Time,
    beginner: elma::Time,
}

pub struct DataRow {
    lev_number: i32,
    lev_name: String,
    pr: elma::Time,
    wr_beat: Option<WR>,
    wr_not_beat: Option<WR>,
}

//TODO(edahl): fix no times read
//TODO(edahl): read lev names from a file

fn create_html_table() -> String {
    let data = io::populate_table_data(&io::read_state);
    let targets_table = io::read_targets_table();

    let headers = vec![
        "Level".to_string(),
        "PR".to_string(),
        "Target".to_string(),
        "Diff".to_string(),
        "Kuski to beat".to_string(),
        "Time beat".to_string(),
        "Kuski beat".to_string(),
    ];
    let mut html_table = String::new();
    html_table.push_str(&html::inline_tr(html::table_header(headers)));

    for (i, r) in data.iter().enumerate() {
        html_table.push_str(&html::table_data_s(&format!(
            "{}. {}",
            &r.lev_number.to_string(),
            &r.lev_name
        )));
        html_table.push_str(&html::time_to_tagged_td(&r.pr, &targets_table[i]));

        if let Some(wr) = r.wr_not_beat.clone() {
            html_table.push_str(&html::time_to_tagged_td(&wr.time, &targets_table[i]));
            html_table.push_str(&html::time_to_diff(&(r.pr - wr.time)));
            html_table.push_str(&html::table_data_s(&format!(
                "{} {}",
                wr.kuski,
                html::table_num(wr.table.to_string())
            )));
        } else {
            html_table.push_str(&html::table_data_s(&"-".to_string()));
            html_table.push_str(&html::table_data_s(&"-".to_string()));
            html_table.push_str(&html::table_data_s(&"-".to_string()));
        }

        if let Some(wr) = r.wr_beat.clone() {
            html_table.push_str(&html::time_to_tagged_td(&wr.time, &targets_table[i]));
            html_table.push_str(&html::table_data_s(&format!(
                "{} {}",
                wr.kuski,
                html::table_num(wr.table.to_string())
            )));
        } else {
            html_table.push_str(&html::table_data_s(&"-".to_string()));
            html_table.push_str(&html::table_data_s(&"-".to_string()));
        }

        html_table = html::inline_tr(html_table);
    }

    html_table = html::inline_table(html_table);

    html_table
}

fn create_html(html_table: String) -> String {
    format!(
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
    )
}

fn main() {
    let html_table = create_html_table();
    let html = create_html(html_table);

    //TODO?(edahl): <link rel=\"stylesheet\" type=\"text/css\" href=\"/styles.css\">

    let size = (900, 778);
    let resizable = false;
    let debug = true;
    //let init_cb = |_webview| {};
    let frontend_cb = |_webview: &mut _, _arg: &_, _userdata: &mut _| {};
    let userdata = ();

    web_view::run(
        "WR-stats",
        web_view::Content::Html(html),
        Some(size),
        resizable,
        debug,
        move |webview| {
            webview.dispatch(|webview, userdata| {
                //update_html(webview);
                //if let Err(e) = watch(&move || update_html(webview)) {
                //    println!("error: {:?}", e)
                //}
            })
        },
        frontend_cb,
        userdata,
    );
}

/*
fn watch(f: &Fn() -> ()) -> notify::Result<()> {
    // Create a channel to receive the events.
    let (tx, rx) = std::sync::mpsc::channel();

    // Automatically select the best implementation for your platform.
    // You can also access each implementation directly e.g. INotifyWatcher.
    let mut watcher: notify::RecommendedWatcher =
        try!(notify::Watcher::new(tx, std::time::Duration::from_secs(1)));

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    try!(watcher.watch("state.dat", notify::RecursiveMode::NonRecursive));

    // This is a simple loop, but you may want to use more complex logic here,
    // for example to handle I/O.
    loop {
        match rx.recv() {
            Ok(event) => {
                println!("{:?}", event);
                f()
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
*/

fn update_html<'a, T>(webview: &mut WebView<'a, T>) {
    let html_table = create_html_table();
    let mut html = create_html(html_table);
    // Hacking a JSEscapeString equivalent.
    html = html.replace(r#"""#, r#"\""#)
        .replace("/", r"\/")
        .replace(r"'", r"\'")
        .replace("\n", r"\n")
        .replace("\r", r"\r");
    webview.eval(&format!("document.documentElement.innerHTML=\"{}\";", html));
}
