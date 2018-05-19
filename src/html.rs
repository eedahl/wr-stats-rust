/*
? ideas for data
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

// ? Use any of these?
// ? {bootstrap_js}
// ? bootstrap_js = inline_script(include_str!("bootstrap-4.1.1/js/bootstrap.min.js")),
// ? {plotly}
// ? plotly = inline_script(include_str!("plotly-latest.min.js")),
// ? {jquery}
// ? jquery = inline_script(include_str!("jquery-3.3.1.min.js")),
*/

// ! Index
pub fn index() -> String {
    format!(
r#"
<!doctype html>
<html lang="en">
    <head>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1, shrink-to-fit=no">
        {bootstrap}
        {c3_styles}
        {styles}
    </head>
    <body>
    <!--[if lt IE 12]>
    <div class="ie-upgrade-container">
        <p class="ie-upgrade-message">Please, upgrade Internet Explorer to continue using this software.</p>
        <a class="ie-upgrade-link" target="_blank" href="https://www.microsoft.com/en-us/download/internet-explorer.aspx">Upgrade</a>
    </div>
    <![endif]-->
    <!--[if gte IE 12 | !IE ]> <!-->
        <div class="container-fluid" id="view"></div>
        <script charset="utf-8">
            {d3_script}
        </script>
        {c3_script}
        {script}
    <![endif]-->
    </body>
</html>
"#,     
        bootstrap = inline_style(include_str!("bootstrap-4.1.1/css/bootstrap.min.css")),
        d3_script = include_str!("d3/d3.min.js"),
        c3_styles = inline_style(include_str!("c3-0.6.0/c3.css")),
        c3_script = inline_script(include_str!("c3-0.6.0/c3.min.js")),
        styles = inline_style(include_str!("styles.css")),
        script = inline_script(include_str!("wr-stats.js")),
    )
}

// ! Table view
pub fn table_view() -> String {
    format!(
r#"
<p>Table view</p>
<p id="to-chart-view" onclick="rpc.request({{cmd: 'displayView', view: 'level', }});">Go to chart view</p>
<table id="wr-table" class="table table-sm table-condensed table-dark table-striped table-hover thead-dark">
    <thead>
        <tr>
            <th scope="col" id="lev" class="sort">Level</th>
            <th scope="col" id="pr" class="sort">PR</th>
            <th scope="col" id="wr-beat" class="sort"">WR beat</th>
            <th scope="col" id="kuski-beat" class="sort">Kuski beat (<strong><em>table</em></strong>)</th>
            <th scope="col" id="target-wr" class="sort">Target WR (<strong><em>diff</em></strong>)</th>
            <th scope="col" id="kuski-to-beat" class="sort">Kuski to beat (<strong><em>table</em></strong>)</th>
            <th scope="col" id="target" class="sort">Next target</th>
        </tr>
    </thead>
    <tbody id="table-body">
    </tbody>
    <tfoot id="table-footer">
    </tfoot> 
</table>
"#)
}

// ! Level view
pub fn level_view() -> String {
    format!(
        r#"
<p>Level view</p>
<p class="to-table-view" onclick="rpc.request({{cmd: 'displayView', view: 'table', }});">Go to table view</p>
<div id="chart" class="container-fluid"></div>
"#)
}

fn inline_style(s: &str) -> String {
    format!(r#"<style rel="stylesheet" type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script>{}</script>"#, s)
}
