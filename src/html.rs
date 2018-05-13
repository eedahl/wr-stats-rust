extern crate pad;
use elma::Time;
use failure::Error;
use shared::DataRow;
use shared::Targets;

pub fn format_html(html_table: &str, p_tt: &Time, t_tt: &Time) -> String {
    format!(
        r#"
            <!doctype html>
            <html lang="en">
                <head>
                    <meta charset="utf-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no">
                    <link rel="icon" src="http://ldev.no/wr-stats/wr-stats.png">
                    {bootstrap}
                    {styles}
                </head>
                <body>
                    <div class="container-fluid">
                        <table id="wr_table" class="table table-sm table-condensed table-dark table-striped table-hover thead-dark">
                            <thead>
                                <tr>
                                    <th scope="col" id="lev">Level</th>
                                    <th scope="col" id="pr">PR</th>
                                    <th scope="col" id="wr_beat">WR beat</th>
                                    <th scope="col" id="k_beat">Kuski beat</th>
                                    <th scope="col" id="target_wr">Target WR <em>(Diff)</em></th>
                                    <th scope="col" id="k_to_beat">Kuski to beat</th>
                                    <th scope="col" id="target">Next target</th>
                                </tr>
                            </thead>
                            <tbody>
                                {table_rows}
                            </tbody>
                        </table>
                        <table id="tt_table" class="table">
                            <tr>
                                <td id="p_tt" class="tt">Personal total time: {p_tt}</td>
                                <td id="t_tt" class="tt">Target total time: {t_tt}</td>
                                <td id="diff" class="tt">Difference: {diff}</td>
                            </tr>
                        </table>
                    </div>
                    {jquery}
                    {bootstrap_js}
                    {scripts}
                </body>
            </html>
            "#,
        bootstrap = inline_style(include_str!("bootstrap-4.1.1/css/bootstrap.min.css")),
        styles = inline_style(include_str!("styles.css")),
        bootstrap_js = inline_script(include_str!("bootstrap-4.1.1/js/bootstrap.min.js")),
        jquery = inline_script(include_str!("jquery-3.3.1.min.js")),
        scripts = inline_script(include_str!("wr-stats.js")),
        table_rows = html_table,
        p_tt = p_tt,
        t_tt = t_tt,
        diff = &(*p_tt - *t_tt)
    )
}

pub fn default_error_message(e: Error) -> String {
    format!(
        r#"
            <!doctype html>
            <html lang="en">
                <head>
                    <meta charset="utf-8">
                    <meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no">
                    <link rel="icon" src="http://ldev.no/wr-stats/wr-stats.png">
                    {bootstrap}
                    {styles}
                </head>
                <body>
                    <h2>There was an error while running the program</h2>
                    <p>Likely causes are:</p>
                    <ul> 
                        <li>Could not find state.dat in folder, and fallback to stats.txt failed</li>
                        <li>Could not download either of wr-stats_tables.csv or wr-stats_targets.csv, and could not find local copies in folder</li>
                    </ul>
                    <p>{error:?}</p>
                </body>
            </html>
            "#,
        bootstrap = inline_style(include_str!("bootstrap-4.1.1/css/bootstrap.css")),
        styles = inline_style(include_str!("styles.css")),
        error = e
    )
}

pub fn create_html_table(
    data: &[DataRow],
    targets_table: &[Targets],
    current_wrs: &[Time],
) -> String {
    let mut html_table_rows = String::new();

    for (i, r) in data.iter().enumerate() {
        let mut row = String::new();
        row.push_str(&table_data_s(&format!(
            "{}. {}",
            &r.lev_number.to_string(),
            &r.lev_name
        )));

        row.push_str(&time_to_tagged_td(
            &r.pr,
            &targets_table[i],
            &current_wrs[i],
        ));

        if let Some(ref wr) = r.wr_beat {
            row.push_str(&time_to_tagged_td(
                &wr.time,
                &targets_table[i],
                &current_wrs[i],
            ));
            row.push_str(&table_data_s(&format!(
                "{} {}",
                wr.kuski,
                table_num(&wr.table.to_string())
            )));
        } else {
            row.push_str(&table_data_s("-"));
            row.push_str(&table_data_s("-"));
        }

        if let Some(ref wr) = r.wr_not_beat {
            row.push_str(&time_to_tagged_td_with_diff(
                &wr.time,
                &targets_table[i],
                &current_wrs[i],
                &r.pr,
            ));
            row.push_str(&table_data_s(&format!(
                "{} {}",
                wr.kuski,
                table_num(&wr.table.to_string())
            )));
        } else {
            row.push_str(&table_data_s("-"));
            row.push_str(&table_data_s("-"));
        }

        row.push_str(&time_to_tagged_target_td_with_diff(
            &r.pr,
            &targets_table[i],
            &current_wrs[i],
        ));

        html_table_rows.push_str(&inline_tr(&row));
    }
    html_table_rows
}

fn time_to_tagged_target_td_with_diff(t: &Time, tar: &Targets, cur_wr: &Time) -> String {
    let target = &get_next_target(&t, &tar, &cur_wr);
    let class = &get_time_class(&target, &tar, &cur_wr);
    format!(
        "<td class=\"{}\">{} {}</td>",
        class,
        target.to_string(),
        times_to_diff(t, target)
    )
}

fn time_to_tagged_td_with_diff(t: &Time, tar: &Targets, cur_wr: &Time, t_cmp: &Time) -> String {
    let class = get_time_class(t, tar, cur_wr);
    format!(
        "<td class=\"{}\">{} {}</td>",
        class,
        t.to_string(),
        times_to_diff(t_cmp, t)
    )
}

fn time_to_tagged_td(t: &Time, tar: &Targets, cur_wr: &Time) -> String {
    let class = get_time_class(t, tar, cur_wr);
    format!("<td class=\"{}\">{}</td>", class, t.to_string())
}

fn get_time_class(t: &Time, tar: &Targets, cur_wr: &Time) -> String {
    match *t {
        t if t > tar.beginner => "unclassified",
        t if t > tar.ok => "beginner",
        t if t > tar.good => "ok",
        t if t > tar.professional => "good",
        t if t > tar.world_class => "professional",
        t if t > tar.legendary => "world_class",
        t if t > tar.godlike => "legendary",
        t if t > *cur_wr => "godlike",
        _ => "wr",
    }.to_string()
}

fn get_next_target(t: &Time, tar: &Targets, cur_wr: &Time) -> Time {
    match *t {
        t if t > tar.beginner => tar.beginner,
        t if t > tar.ok => tar.ok,
        t if t > tar.good => tar.good,
        t if t > tar.professional => tar.professional,
        t if t > tar.world_class => tar.world_class,
        t if t > tar.legendary => tar.legendary,
        t if t > tar.godlike => tar.godlike,
        t if t > *cur_wr => *cur_wr,
        _ => *cur_wr,
    }
}

fn table_data_s(s: &str) -> String {
    format!("<td>{}</td>", s)
}

fn inline_tr(h: &str) -> String {
    format!("<tr>{}</tr>", h)
}

fn table_num(h: &str) -> String {
    format!("(<strong><em>{}</em></strong>)", h)
}

fn times_to_diff(t1: &Time, t2: &Time) -> String {
    use html::pad::{Alignment, PadStr};
    format!(
        r#"<span class="diff">{}</span>"#,
        format!(
            r#"(<strong><em>+{}</em></strong>)"#,
            (*t1 - *t2)
            .to_string()
            .trim_left_matches(|x| (x == '0') | (x == ':'))
        )//.pad_to_width_with_alignment(30, Alignment::Right),
    )
}

fn inline_style(s: &str) -> String {
    format!(r#"<style rel="stylesheet" type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script>{}</script>"#, s)
}

fn time_to_wr_tagged_td(t: &Time) -> String {
    format!("<td class=\"wr\">{}</td>", t.to_string())
}
