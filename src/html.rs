use elma::Time;
use failure::Error;
use shared::{build_update_data, get_next_target, DataRow, SortBy, SortOrder, Targets, WR};

pub fn build_initial_html(wr_tables: &[WR], targets_table: &[Targets]) -> Result<String, Error> {
    let (table_rows, table_footer) = build_update_data(
        wr_tables,
        targets_table,
        SortBy::LevelNum(SortOrder::Ascending),
    )?;
    Ok(format_html(&table_rows, &table_footer))
}

pub fn format_html(table_rows: &str, table_footer: &str) -> String {
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
                        <tbody id="table-body">
                            {table_rows}
                        </tbody>
                        <tfoot id="table-footer">
                            {table_footer}
                        </tfoot> 
                    </table>
                </div>
            </div>
        </div>
        {scripts}
    </body>
</html>
            "#,
        // ? {bootstrap_js}
        // ? bootstrap_js = inline_script(include_str!("bootstrap-4.1.1/js/bootstrap.min.js")),
        // ? {plotly}
        // ? plotly = inline_script(include_str!("plotly-latest.min.js")),
        // ? {jquery}
        // ? jquery = inline_script(include_str!("jquery-3.3.1.min.js")),
        bootstrap = inline_style(include_str!("bootstrap-4.1.1/css/bootstrap.min.css")),
        styles = inline_style(include_str!("styles.css")),
        scripts = inline_script(include_str!("wr-stats.js")),
        table_rows = table_rows,
        table_footer = table_footer
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
    <td>{target_tt} (<em><strong>{target_tt_diff}</td>
</tr>"#,
        p_tt = p_tt,
        target_wr_tt = target_wr_tt,
        target_wr_tt_diff = time_to_diff_string(&(*p_tt - *target_wr_tt)),
        target_tt = target_tt,
        target_tt_diff = time_to_diff_string(&(*p_tt - *target_tt))
    )
}

/*
ideas for data
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
