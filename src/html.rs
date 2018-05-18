use elma::Time;
use shared::{get_next_target, DataRow, Targets};
use std::fmt::Debug;

/*
? ideas for data
Personal total time (TT)
Target WRs TT
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

// ? Use any of these?
// ? {bootstrap_js}
// ? bootstrap_js = inline_script(include_str!("bootstrap-4.1.1/js/bootstrap.min.js")),
// ? {plotly}
// ? plotly = inline_script(include_str!("plotly-latest.min.js")),
// ? {jquery}
// ? jquery = inline_script(include_str!("jquery-3.3.1.min.js")),
*/

// ! Index
pub fn index() -> String {
    format!(
r#"
<!doctype html>
<html lang="en">
    <head>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no">
        {bootstrap}
        {c3_styles}
        {styles}
    </head>
    <body>
    <!--[if lt IE 9]>
    <div class="ie-upgrade-container">
        <p class="ie-upgrade-message">Please, upgrade Internet Explorer to continue using this software.</p>
        <a class="ie-upgrade-link" target="_blank" href="https://www.microsoft.com/en-us/download/internet-explorer.aspx">Upgrade</a>
    </div>
    <![endif]-->
    <!--[if gte IE 9 | !IE ]> <!-->
        <div class="container-fluid" id="view"></div>
        <script charset="utf-8">{d3_script}</script>
        {c3_script}
        {script}
    <![endif]-->
    </body>
</html>
"#,     
        bootstrap = inline_style(include_str!("bootstrap-4.1.1/css/bootstrap.min.css")),
        d3_script = include_str!("d3/d3.min.js"),
        c3_styles = inline_style(include_str!("c3-0.6.0/c3.css")),
        c3_script = inline_script(include_str!("c3-0.6.0/c3.min.js")),
        styles = inline_style(include_str!("styles.css")),
        script = inline_script(include_str!("wr-stats.js")),
    )
}

// ! Table view
pub fn table_view() -> String {
    format!(
r#"
<p>Table view</p>
<p id="to-chart-view" onclick="rpc.request({{cmd: 'displayView', view: 'level', }});">Go to chart view</p>
<table id="wr-table" class="table table-sm table-condensed table-dark table-striped table-hover thead-dark">
    <thead>
        <tr>
            <th scope="col" id="lev" class="sort">Level</th>
            <th scope="col" id="pr" class="sort">PR</th>
            <th scope="col" id="wr-beat" class="sort"">WR beat</th>
            <th scope="col" id="kuski-beat" class="sort">Kuski beat (<strong><em>table</em></strong>)</th>
            <th scope="col" id="target-wr" class="sort">Target WR (<strong><em>diff</em></strong>)</th>
            <th scope="col" id="kuski-to-beat" class="sort">Kuski to beat (<strong><em>table</em></strong>)</th>
            <th scope="col" id="target" class="sort">Next target</th>
        </tr>
    </thead>
    <tbody id="table-body">
    </tbody>
    <tfoot id="table-footer">
    </tfoot> 
</table>
"#)
}

// ! Level view
pub fn level_view() -> String {
    format!(
        r#"
<p>Level view</p>
<p class="to-table-view" onclick="rpc.request({{cmd: 'displayView', view: 'table', }});">Go to table view</p>
<p onclick="rpc.updateLevelView();">Update level view</p>
<div id="chart" class="container-fluid"></div>
"#)
}

// ? Default error message?
pub fn default_error_message(e: impl Debug) -> String {
    format!(
r#"
<!doctype html>
<html lang="en">
    <head>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no">
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
        <p>{error:#?}</p>
    </body>
</html>
"#,
        bootstrap = inline_style(include_str!("bootstrap-4.1.1/css/bootstrap.css")),
        styles = inline_style(include_str!("styles.css")),
        error = e
    )
}

pub fn create_table_rows(
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

// TODO(edahl)
fn tt_to_target_tt_td_with_diff(t: &Time, tar: &Targets, cur_wr: &Time) -> String {
    let target = &get_next_target(&t, &tar, &cur_wr);
    let class = &get_time_class(&target, &tar, &cur_wr);
    format!(
        "<td class=\"{}\">{} {}</td>",
        class,
        target.to_string(),
        times_to_diff(t, &target)
    )
}

fn time_to_tagged_target_td_with_diff(t: &Time, tar: &Targets, cur_wr: &Time) -> String {
    let target = &get_next_target(&t, &tar, &cur_wr);
    let class = &get_time_class(&target, &tar, &cur_wr);
    format!(
        "<td class=\"{}\">{} {}</td>",
        class,
        target.to_string(),
        times_to_diff(t, &target)
    )
}

pub fn format_table_footer(p_tt: &Time, target_wr_tt: &Time, target_tt: &Time) -> String {
    format!(
        r#"
<tr>
    <td></td>
    <td id="p_tt" class="tt">{p_tt}</td>
    <td></td>
    <td></td>
    <td id="target_wr_tt" class="tt">{target_wr_tt} (<em><strong>{target_wr_tt_diff}</em></strong>)</td>
    <td></td>
    <td>{target_tt} (<em><strong>{target_tt_diff})</td>
</tr>
"#,
        p_tt = p_tt,
        target_wr_tt = target_wr_tt,
        target_wr_tt_diff = time_to_diff_string(&(*p_tt - *target_wr_tt)),
        target_tt = target_tt,
        target_tt_diff = time_to_diff_string(&(*p_tt - *target_tt))
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
    format!(
        r#"<span class="diff">(<strong><em>{}</em></strong>)</span>"#,
        time_to_diff_string(&(*t1 - *t2))
    )
}

fn time_to_diff_string(t: &Time) -> String {
    let (negative, h, m, s, hd) = t.to_parts();
    let sign = if negative { "-" } else { "+" };

    match (h, m, s, hd) {
        (0, 0, 0, hd) => format!("{sign}0,{hd:02}", sign = sign, hd = hd),
        (0, 0, s, hd) => format!("{sign}{s},{hd:02}", sign = sign, s = s, hd = hd),
        (0, m, s, hd) => format!(
            "{sign}{m}:{s:02},{hd:02}",
            sign = sign,
            m = m,
            s = s,
            hd = hd
        ),
        (h, m, s, hd) => format!(
            "{sign}{h}:{m:02}:{s:02},{hd:02}",
            sign = sign,
            h = h,
            m = m,
            s = s,
            hd = hd
        ),
    }
}

fn inline_style(s: &str) -> String {
    format!(r#"<style rel="stylesheet" type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script>{}</script>"#, s)
}
