use elma::Time;
use maud::html;
use maud::Markup;
use shared;

#[allow(dead_code)]
pub struct Row {
    lev_number: i32,
    lev_name: String,
    pr: Time,
    pr_class: String,
    kuski_beat: String,
    kuski_beat_table: i32,
    wr_beat: Time,
    wr_beat_class: String,
    kuski_not_beat: String,
    kuski_not_beat_table: i32,
    wr_not_beat: Time,
    wr_not_beat_class: String,
    target: Time,
    target_class: String,
}

#[allow(dead_code)]
pub fn table_row(row: &Row) -> Markup {
    html!({ tr { // id
        td.lev-td (format!("{}. {}", row.lev_number, row.lev_name)) //id
        td (row.pr) //id
        td { (row.kuski_not_beat) (diff(row.pr-row.wr_not_beat)) } //id
        td { (row.wr_not_beat) (diff(row.pr-row.wr_not_beat)) } //id
        td { (row.kuski_beat) (diff(row.pr-row.wr_not_beat)) } //id
        td { (row.wr_beat) (diff(row.pr-row.wr_beat)) } //id
    } })
}

#[allow(dead_code)]
pub fn table_footer(p_tt: Time, target_wr_tt: Time, target_tt: Time) -> Markup {
    html!({
        tr {
            td
            td class="tt" id="p_tt" (p_tt.to_string())
            td
            td
            td class="tt" id="target_wr_tt" { 
                (target_wr_tt.to_string()) "" (diff(p_tt - target_wr_tt))
            }
            td
            td class="tt" id="target_tt" { 
                (target_tt.to_string()) "" (diff(p_tt - target_tt))
            }
        }
    })
}

#[allow(dead_code)]
pub fn diff(diff: Time) -> Markup {
    html!({
        span class="diff" {
            "(" em {
                strong { (shared::time_to_diff_string(diff)) }
            } ")"
        }
    })
}
