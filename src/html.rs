extern crate elma;
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

pub fn table_num(h: String) -> String {
    format!("(<i>{}</i>)", h)
}

pub fn inline_table(s: String) -> String {
    format!(r#"<table>{}</table>"#, s)
}

pub fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

pub fn time_to_diff(t: &elma::Time) -> String {
    format!("<td>+{}</td>", t.to_string())
}

pub fn time_to_tagged_td(t: &elma::Time, tar: &Targets) -> String {
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
