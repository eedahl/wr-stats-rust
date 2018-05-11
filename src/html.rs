use shared::Targets;
use shared::DataRow;
use elma::Time;

pub fn create_html(html_table: &str, p_tt: &Time, t_tt: &Time) -> String {
    format!(
        r#"
            <!doctype html>
            <html>
                <head>
                    <link rel="icon" src="wr-stats.png">
                    {styles}
                </head>
                <body>
                    <table>
                        <tr>
                            <th>Level</th>
                            <th>PR</th>
                            <th>Target WR</th>
                            <th>Difference</th>
                            <th>Kuski to beat</th>
                            <th>WR beat</th>
                            <th>Kuski beat</th>
                        </tr>
                        {table_rows}
                    </table>
                    <table id="tt_table">
                        <tr>
                            <td id="p_tt" class="tt">Personal total time: {p_tt}</td>
                            <td id="t_tt" class="tt">Target total time: {t_tt}</td>
                            <td id="diff" class="tt">Difference: {diff}</td>
                        </tr>
                    </table>
                </body>
            </html>
            "#,
        styles = inline_style(include_str!("styles.css")),
        table_rows = html_table,
        p_tt = p_tt,
        t_tt = t_tt,
        diff = &(*p_tt - *t_tt)
    )
}
//current_wrs: &[Time], 
pub fn create_html_table(data: &[DataRow], targets_table: &[Targets], current_wrs: &[Time]) -> String {
    let mut html_table_rows = String::new();

    for (i, r) in data.iter().enumerate() {
        let mut row = String::new();
        row.push_str(&table_data_s(&format!(
            "{}. {}",
            &r.lev_number.to_string(),
            &r.lev_name
        )));

        if let Some(ref wr) = r.wr_not_beat {
            row.push_str(&time_to_tagged_td(&r.pr, &targets_table[i], &current_wrs[i]));
            row.push_str(&time_to_tagged_td(&wr.time, &targets_table[i], &current_wrs[i]));
            row.push_str(&time_to_diff(&(r.pr - wr.time)));
            row.push_str(&table_data_s(&format!(
                "{} {}",
                wr.kuski,
                table_num(&wr.table.to_string())
            )));
        } else {
            row.push_str(&time_to_wr_tagged_td(&r.pr));
            row.push_str(&table_data_s(&"-".to_string()));
            row.push_str(&table_data_s(&"-".to_string()));
            row.push_str(&table_data_s(&"-".to_string()));
        }

        if let Some(ref wr) = r.wr_beat {
            row.push_str(&time_to_tagged_td(&wr.time, &targets_table[i], &current_wrs[i]));
            row.push_str(&table_data_s(&format!(
                "{} {}",
                wr.kuski,
                table_num(&wr.table.to_string())
            )));
        } else {
            row.push_str(&table_data_s(&"-".to_string()));
            row.push_str(&table_data_s(&"-".to_string()));
        }

        html_table_rows.push_str(&inline_tr(&row));
    }

    html_table_rows
}

fn table_data_s(s: &str) -> String {
    format!("<td>{}</td>", s)
}

fn inline_tr(h: &str) -> String {
    format!("<tr>{}</tr>", h)
}

fn table_num(h: &str) -> String {
    format!("(<i>{}</i>)", h)
}

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn time_to_diff(t: &Time) -> String {
    format!("<td>+{}</td>", t.to_string())
}

fn time_to_wr_tagged_td(t: &Time) -> String {
    format!("<td class=\"wr\">{}</td>", t.to_string())
}

fn time_to_tagged_td(t: &Time, tar: &Targets, cur_wr: &Time) -> String {
    let class = match *t {
        t if t > tar.beginner => "unclassified",
        t if t > tar.ok => "beginner",
        t if t > tar.good => "ok",
        t if t > tar.professional => "good",
        t if t > tar.world_class => "professional",
        t if t > tar.legendary => "world_class",
        t if t > tar.godlike => "legendary",
        t if t > *cur_wr => "godlike",
        _ => "wr",
    };
    format!("<td class=\"{}\">{}</td>", class, t.to_string())
}
