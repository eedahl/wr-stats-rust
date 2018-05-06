pub fn table_header(h: Vec<String>) -> String {
    h.iter().map(|x| format!("<th>{}</th>", *x)).collect()
}

pub fn table_data(h: Vec<String>) -> String {
    h.iter().map(|x| format!("<td>{}</td>", *x)).collect()
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
