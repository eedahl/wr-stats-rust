extern crate elma;
use Targets;
use DataRow;
//use ::io;

pub fn create_html_table(data: &Vec<DataRow>, targets_table: &Vec<Targets>) -> String {
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
    html_table.push_str(&inline_tr(table_header(headers)));

    for (i, r) in data.iter().enumerate() {
        html_table.push_str(&table_data_s(&format!(
            "{}. {}",
            &r.lev_number.to_string(),
            &r.lev_name
        )));
        html_table.push_str(&time_to_tagged_td(&r.pr, &targets_table[i]));

        if let Some(wr) = r.wr_not_beat.clone() {
            html_table.push_str(&time_to_tagged_td(&wr.time, &targets_table[i]));
            html_table.push_str(&time_to_diff(&(r.pr - wr.time)));
            html_table.push_str(&table_data_s(&format!(
                "{} {}",
                wr.kuski,
                table_num(wr.table.to_string())
            )));
        } else {
            html_table.push_str(&table_data_s(&"-".to_string()));
            html_table.push_str(&table_data_s(&"-".to_string()));
            html_table.push_str(&table_data_s(&"-".to_string()));
        }

        if let Some(wr) = r.wr_beat.clone() {
            html_table.push_str(&time_to_tagged_td(&wr.time, &targets_table[i]));
            html_table.push_str(&table_data_s(&format!(
                "{} {}",
                wr.kuski,
                table_num(wr.table.to_string())
            )));
        } else {
            html_table.push_str(&table_data_s(&"-".to_string()));
            html_table.push_str(&table_data_s(&"-".to_string()));
        }

        html_table = inline_tr(html_table);
    }

    html_table = inline_table(html_table);

    html_table
}

pub fn create_html(html_table: String) -> String {
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
        styles = inline_style(include_str!("styles.css")),
        table = html_table
    )
}

fn table_header(h: Vec<String>) -> String {
    h.iter().map(|x| format!("<th>{}</th>", *x)).collect()
}

fn table_data_s(s: &String) -> String {
    format!("<td>{}</td>", s)
}

fn inline_tr(h: String) -> String {
    format!("<tr>{}</tr>", h)
}

fn table_num(h: String) -> String {
    format!("(<i>{}</i>)", h)
}

fn inline_table(s: String) -> String {
    format!(r#"<table>{}</table>"#, s)
}

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn time_to_diff(t: &elma::Time) -> String {
    format!("<td>+{}</td>", t.to_string())
}

fn time_to_tagged_td(t: &elma::Time, tar: &Targets) -> String {
    if !(t <= &tar.beginner) {
        return format!("<td class=\"unclassified\">{}</td>", t.to_string());
    } else {
        if !(t <= &tar.ok) {
            return format!("<td class=\"beginner\">{}</td>", t.to_string());
        } else {
            if !(t <= &tar.good) {
                return format!("<td class=\"ok\">{}</td>", t.to_string());
            } else {
                if !(t <= &tar.professional) {
                    return format!("<td class=\"good\">{}</td>", t.to_string());
                } else {
                    if !(t <= &tar.world_class) {
                        return format!("<td class=\"professional\">{}</td>", t.to_string());
                    } else {
                        if !(t <= &tar.legendary) {
                            return format!("<td class=\"world_class\">{}</td>", t.to_string());
                        } else {
                            if !(t <= &tar.godlike) {
                                return format!("<td class=\"legendary\">{}</td>", t.to_string());
                            } else {
                                return format!("<td class=\"godlike\">{}</td>", t.to_string());
                            }
                        }
                    }
                }
            }
        }
    }
}
