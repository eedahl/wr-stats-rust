use elma::Time;
use maud::{html, Markup};
use shared;

#[allow(dead_code)]
pub struct Row {
    pub lev_number: i32,
    pub lev_name: String,
    pub pr: Time,
    pub pr_class: String,
    pub kuski_beat: String,
    pub kuski_beat_table: i32,
    pub wr_beat: Time,
    pub wr_beat_class: String,
    pub kuski_not_beat: String,
    pub kuski_not_beat_table: i32,
    pub wr_not_beat: Time,
    pub wr_not_beat_class: String,
    pub target: Time,
    pub target_class: String,
}

pub fn table_body(rows: &[Row]) -> String {
    html!({
        @for r in rows.into_iter() {
            (table_row(r))
        }
    }).into_string()
}

#[allow(dead_code)]
fn table_row(row: &Row) -> Markup {
    let pr = row.pr;
    html!({ tr id={ "lev-" (row.lev_number) } { // id
        td class="lev-td" (format!("{}. {}", row.lev_number, row.lev_name)) //id
        td class={ "pr-td" " " (row.pr_class) } (pr) //id
        @if row.kuski_beat_table != 0 {
            td { (row.kuski_beat) (table_num(row.kuski_beat_table)) }
            (time_td_with_diff(row.wr_beat, &row.wr_beat_class, pr))
        } @else {
            td span class="empty-td" "-"
            td span class="empty-td" "-"
        }
        @if row.kuski_not_beat_table != 0 {
            td { (row.kuski_not_beat) (table_num(row.kuski_not_beat_table)) }
            (time_td_with_diff(row.wr_not_beat, &row.wr_not_beat_class, pr))
        } @else {
            td class="empty-td" "-"
            td class="empty-td" "-"
        }
        (time_td_with_diff(row.target, &row.target_class, pr))
    }})
}
/*
        formatWrBeatEntry(row['wr_beat'], pr.time) +
        formatTimeEntry(row['wr_not_beat'], pr.time) +
        formatTimeEntry(row['target'], pr.time) +
        "</tr>"
}

// * Not very robust
function formatWrBeatEntry(entry, pr) {
    if (entry['time'] == 0) {
        return "<td class=\"kuski-beat-td empty-td\">-</td>" +
            "<td class=\"empty-td\">-</td>";
    }
    var kuskiTd = "";
    if (entry['table'] != 0 && entry['table'] != null) {
        kuskiTd = "<td class=\"kuski-beat-td\">" + entry['kuski'] + " (<em><strong>" +
            entry['table'] + "</em></strong>)</td>";
    }
    var timeTd = "<td class=\"" + entry['class'] + "\">" +
        formatTime(entry['time']) +
        " <span class=\"diff\">(<em><strong>" +
        formatTimeDiff(pr - entry['time']) +
        "</em></strong>)</span></td>";

    return kuskiTd + timeTd;
}

function formatTimeEntry(entry, pr) {
    if (entry['time'] == 0) {
        return "<td class=\"empty-td\">-</td><td class=\"empty-td\">-</td>";
    }
    var kuskiTd = "";
    if (entry['table'] != 0 && entry['table'] != null) {
        kuskiTd = "<td>" + entry['kuski'] + " (<em><strong>" +
            entry['table'] + "</em></strong>)</td>";
    }
    var timeTd = "<td class=\"" + entry['class'] + "\">" +
        formatTime(entry['time']) +
        " <span class=\"diff\">(<em><strong>" +
        formatTimeDiff(pr - entry['time']) +
        "</em></strong>)</span></td>";

    return kuskiTd + timeTd;
}
*/
#[allow(dead_code)]
pub fn table_footer(
    p_tt: Time,
    p_tt_class: &str,
    target_wr_tt: Time,
    target_wr_tt_class: &str,
    target_tt: Time,
    target_tt_class: &str,
) -> String {
    html!({
        tr {
            td
            td class={ (p_tt_class) } (p_tt.to_string())
            td
            td
            (time_td_with_diff(target_wr_tt, &target_wr_tt_class, p_tt))
            td
            (time_td_with_diff(target_tt, &target_tt_class, p_tt))
        }
    }).into_string()
}

pub fn time_td_with_diff(tt: Time, tt_class: &str, t: Time) -> Markup {
    html!({
        td class={ (tt_class) } { 
            (tt.to_string()) "" (diff(t - tt))
        }
    })
}

fn table_num(table: i32) -> Markup {
    html!({
        span class="diff" {
            "(" em {
                strong { (table) }
            } ")"
        }
    })
}

pub fn diff(diff: Time) -> Markup {
    html!({
        span class="diff" {
            "(" em {
                strong { (shared::time_to_diff_string(diff)) }
            } ")"
        }
    })
}
