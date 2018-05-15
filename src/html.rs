use elma::Time;
use failure::Error;
use shared::get_next_target;
use shared::DataRow;
use shared::Targets;

// * col-sm-9
pub fn format_html(tables: &str, sidebar: &str) -> String {
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
        <div class="container-fluid">
            <div class="row content">
                <!--<div class="col-sm-2" id="sidebar">
                    
                    <div class="col-sm-2" >
                    <div class="row-sm-4" id="plot"></div>
                </div>-->
                <div class="col-sm-12" id="table-container">
                    <table id="wr-table" class="table table-sm table-condensed table-dark table-striped table-hover thead-dark">
                        <thead>
                            <tr>
                                <th scope="col" id="lev" class="sort">Level</th>
                                <th scope="col" id="pr" class="sort">PR</th>
                                <th scope="col" id="wr_beat" class="sort"">WR beat</th>
                                <th scope="col" id="kuski_beat" class="sort">Kuski beat (<strong><em>table</em></strong>)</th>
                                <th scope="col" id="target_wr" class="sort">Target WR (<strong><em>diff</em></strong>)</th>
                                <th scope="col" id="kuski_to_beat" class="sort">Kuski to beat (<strong><em>table</em></strong>)</th>
                                <th scope="col" id="target" class="sort">Next target</th>
                            </tr>
                        </thead>
                        <tbody id="wr-table-rows">
                            {table_rows}
                        </tbody>
                        <tfooter id="sidebar">{sidebar}</tfooter> 
                    </table>
                </div>
            </div>
        </div>
        {jquery}
        {bootstrap_js}
        {plotly}
        {scripts}
    </body>
</html>
            "#,// ! sidebar footer temporary
        bootstrap = inline_style(include_str!("bootstrap-4.1.1/css/bootstrap.min.css")),
        styles = inline_style(include_str!("styles.css")),
        bootstrap_js = inline_script(include_str!("bootstrap-4.1.1/js/bootstrap.min.js")),
        jquery = inline_script(include_str!("jquery-3.3.1.min.js")),
        plotly = inline_script(include_str!("plotly-latest.min.js")),
        scripts = inline_script(include_str!("wr-stats.js")),
        table_rows = tables,
        sidebar = sidebar
    )
}

pub fn format_sidebar(p_tt: &Time, wr_tt: &Time) -> String {
    format!(
        r#"
<tr>
    <td></td>
    <td id="p_tt" class="tt">{p_tt}</td>
    <td></td>
    <td id="wr_tt" class="tt">{wr_tt}</td>
    <td id="diff" class="tt">PTT-TWRTT: {diff}</td>
    <td></td>
    <td></td>
</tr>"#,
        p_tt = p_tt,
        wr_tt = wr_tt,
        diff = &(*p_tt - *wr_tt)
    )
}

/*
<div id="tt-table">
    <p id="p_tt" class="tt">Personal total time: {p_tt}</p>
    <p id="wr_tt" class="tt">Target WRs total time: {wr_tt}</p>
    <p id="diff" class="tt">Difference: {diff}</p>
</div>

<ul id="tts" class="ul">
    <li id="p_tt" class="tt">Personal total time (TT): {p_tt}</li>
    <li id="wr_tt" class="tt">Target WRs TT: {wr_tt}</li>
    <li id="diff" class="tt">Difference: {diff}</li>
    <li class="tt">TT of current WRs</li>
    <li class="tt">TT if times at least beginner</li>
    <li class="tt">TT if times at least ok</li>
    <li class="tt">TT if times at least good</li>
    <li class="tt">TT if times at least professional</li>
    <li class="tt">TT if times at least world class</li>
    <li class="tt">TT if times at least legendary</li>
    <li class="tt">TT if times at least godlike</li>
    <li class="tt">Number of times after table 1</li>
    <li class="tt">Number of times after table 50</li>
    <li class="tt">Number of times after table 100</li>
    <li class="tt">Number of times after table 150</li>
    <li class="tt">Number of times after table 200</li>
    <li class="tt">Number of times after table 250</li>
    <li class="tt">Number of times after table 300</li>
    <li class="tt">Number of times after table 350</li>
    <li class="tt">Number of times after table 400</li>
</ul>
*/

pub fn default_error_message(e: Error) -> String {
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
                    <p>{error:?}</p>
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
    //let pos_t: Time = if t.0 < 0 { Time(-t.0) } else { *t };

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
