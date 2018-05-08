#![windows_subsystem = "windows"]
extern crate elma;
extern crate web_view;
//extern crate notify;

use elma::Time;
use web_view::WebView;
//use notify::Watcher;

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

fn compute_tts(d: &[DataRow]) -> (elma::Time, elma::Time) {
    let mut p_tt = Time::from("00:00,00");
    let mut t_tt = Time::from("00:00,00");

    for r in d {
        p_tt = p_tt + r.pr;

        t_tt = t_tt + if let Some(wr) = r.wr_not_beat.clone() {
            wr.time
        } else {
            r.pr
        };
    }

    (p_tt, t_tt)
}

//TODO(edahl): notify
//TODO(edahl): diff
//TODO(edahl): read lev names from a file
//TODO(edahl): stats.txt fallback
fn main() {
    let wr_tables = io::load_wr_tables();
    let targets_table = io::load_targets_table();

    let pr_table = io::load_state().expect("Could not load file: state.dat");

    //repeat code
    let data = io::populate_table_data(&pr_table, &wr_tables);
    let html_table = html::create_html_table(&data, &targets_table);
    let (p_tt, t_tt) = compute_tts(&data);
    let html = html::create_html(&html_table, &p_tt, &t_tt);

    //TODO?(edahl): <link rel=\"stylesheet\" type=\"text/css\" href=\"/styles.css\">
    let size = (900, 790);
    let resizable = true;
    let debug = true;
    //let init_cb = |_webview| {};
    let frontend_cb = |_webview: &mut _, _arg: &_, _userdata: &mut _| {};
    let userdata = ();

    web_view::run(
        "WR Stats",
        web_view::Content::Html(html),
        Some(size),
        resizable,
        debug,
        move |webview| {
            std::thread::spawn(move || {
                loop {
                    if let Ok(pr_table) = io::load_state() {

                        //repeat code
                        let data = io::populate_table_data(&pr_table, &wr_tables);
                        let html_table = html::create_html_table(&data, &targets_table);
                        let (p_tt, t_tt) = compute_tts(&data);
                        let html = html::create_html(&html_table, &p_tt, &t_tt);

                        webview.dispatch(move |webview, _userdata| {
                            update_html(webview, &html);
                        });
                    }
                    std::thread::sleep(std::time::Duration::from_secs(5));
                }
            });
        },
        frontend_cb,
        userdata,
    );
}

fn update_html<'a, T>(webview: &mut WebView<'a, T>, html: &str) {
    webview.eval(&format!(
        "document.documentElement.innerHTML={};",
        web_view::escape(html)
    ));
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
