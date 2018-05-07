use time;
use time::Time;
use Targets;

pub fn table_header(h: Vec<String>) -> String {
    h.iter().map(|x| format!("<th>{}</th>", *x)).collect()
}

pub fn table_data(h: Vec<String>) -> String {
    h.iter().map(|x| format!("<td>{}</td>", *x)).collect()
}

pub fn table_data_s(s: &String) -> String {
    format!("<td>{}</td>", s)
}

pub fn inline_tr(h: String) -> String {
    format!("<tr>{}</tr>", h)
}

pub fn inline_table(s: String) -> String {
    format!(r#"<table>{}</table>"#, s)
}

pub fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

pub fn time_to_diff(t: &Time) -> String {
    format!("<td>+{}</td>", time::to_string(t))
}

pub fn time_to_tagged_td(t: &Time, tar: &Targets) -> String {
    if !time::compare(&t, &tar.beginner) {
        return format!("<td class=\"unclassified\">{}</td>", time::to_string(&t));
    } else {
        if !time::compare(&t, &tar.ok) {
            return format!("<td class=\"beginner\">{}</td>", time::to_string(&t));
        } else {
            if !time::compare(&t, &tar.good) {
                return format!("<td class=\"ok\">{}</td>", time::to_string(&t));
            } else {
                if !time::compare(&t, &tar.professional) {
                    return format!("<td class=\"good\">{}</td>", time::to_string(&t));
                } else {
                    if !time::compare(&t, &tar.world_class) {
                        return format!("<td class=\"professional\">{}</td>", time::to_string(&t));
                    } else {
                        if !time::compare(&t, &tar.legendary) {
                            return format!(
                                "<td class=\"world_class\">{}</td>",
                                time::to_string(&t)
                            );
                        } else {
                            if !time::compare(&t, &tar.godlike) {
                                return format!(
                                    "<td class=\"legendary\">{}</td>",
                                    time::to_string(&t)
                                );
                            } else {
                                return format!(
                                    "<td class=\"godlike\">{}</td>",
                                    time::to_string(&t)
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}
