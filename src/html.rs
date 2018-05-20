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
    <!--[if lt IE 9]>
    <div class="ie-upgrade-container">
        <p class="ie-upgrade-message">Please, upgrade Internet Explorer to continue using this software.</p>
        <a class="ie-upgrade-link" target="_blank" href="https://www.microsoft.com/en-us/download/internet-explorer.aspx">Upgrade</a>
    </div>
    <![endif]-->
    <!--[if gte IE 9 | !IE ]> <!-->
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
<table id="wr-table" class="table table-sm table-condensed table-dark table-striped table-hover thead-dark">
    <thead>
        <tr>
            <th scope="col" id="lev" class="sort">Level</th>
            <th scope="col" id="pr" class="sort">PR</th>
            <th scope="col" id="kuski-beat" class="sort">Kuski beat (<strong><em>table</em></strong>)</th>
            <th scope="col" id="wr-beat" class="sort"">WR beat</th>
            <th scope="col" id="kuski-to-beat" class="sort">Kuski to beat (<strong><em>table</em></strong>)</th>
            <th scope="col" id="target-wr" class="sort">Target WR (<strong><em>diff</em></strong>)</th>
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
