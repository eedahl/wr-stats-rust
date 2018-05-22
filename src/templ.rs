use elma::Time;
use maud::html;
use maud::Markup;
use shared;

pub fn table_footer(p_tt: Time, target_wr_tt: Time, target_tt: Time) -> Markup {
    html!({
        tr{
            td
            td class="tt" id="p_tt" (p_tt.to_string())
            td
            td
            td class="tt" id="target_wr_tt" { 
                (target_wr_tt.to_string()) "" (diff(p_tt - target_wr_tt))
            }
            td
            td class="tt" id="target_wr_tt" { 
                (target_tt.to_string()) "" (diff(p_tt - target_tt))
            }
        }
    })
}

pub fn diff(diff: Time) -> Markup {
    html!({
        "(" em {
             strong { (shared::time_to_diff_string(diff)) }
        } ")"
    })
}
