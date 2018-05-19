'use strict';

window.onload = function () {
    window.onerror = function () {
        rpc.log(arguments);
    }
    rpc.displayView('table');
};

var rpc = {
    request: function (arg) {
        if (arg['cmd'] != 'log') {
            rpc.log('request:', JSON.stringify(arg));
        }
        window.external.invoke(JSON.stringify(arg));
    },
    displayView: function (view) {
        rpc.request({
            cmd: 'displayView',
            view: view,
        });
    },
    updateView: function (view, arg) {
        rpc.request({
            cmd: 'updateView',
            view: view,
            arg: arg,
        });
    },
    log: function () {
        var s = '';
        for (var i = 0; i < arguments.length; i++) {
            if (i != 0) {
                s = s + ' ';
            }
            s = s + JSON.stringify(arguments[i]);
        }
        rpc.request({
            cmd: 'log',
            text: s
        });
    },

}

var views = {
    activeView: 'none',
    display: function (arg) {
        var obj = JSON.parse(arg);
        this.activeView = obj['view'];
        document.getElementById('view').innerHTML = obj['template'];
        switch (this.activeView) {
            case 'table':
                tableView.init();
                rpc.updateView('table', {
                    'param': tableView.param,
                    'ascending': tableView.ascending
                });
                break;
            case 'level':
                levelView.init();
                rpc.updateView('level', {
                    'level': levelView.level
                });
                break;
        }

    },
    // TODO(edahl): make no argument update function
    updateView: function (arg) {
        var obj = JSON.parse(arg);
        if (this.activeView == obj['view']) {
            switch (this.activeView) {
                case 'table':
                    //tableView.update(obj['rows'], obj['footer']);
                    tableView.update_json(obj['data']);
                    break;
                case 'level':
                    levelView.update(obj['data']);
                    break;
            }
        }
    }
}

var tableView = {
    param: "LevelNum",
    ascending: true,
    init: function () {
        var colSortHint = [{
                'id': 'lev',
                'hint': 'LevelNum'
            },
            {
                'id': 'pr',
                'hint': 'PR'
            },
            {
                'id': 'wr-beat',
                'hint': 'DiffToPrevWR'
            },
            {
                'id': 'kuski-beat',
                'hint': 'Table'
            },
            {
                'id': 'target-wr',
                'hint': 'DiffToNextWR'
            },
            {
                'id': 'kuski-to-beat',
                'hint': 'Table'
            },
            {
                'id': 'target',
                'hint': 'DiffToNextTarget'
            },
        ];
        colSortHint.map(function (val) {
            document.getElementById(val.id).addEventListener("click", function () {
                rpc.log('sorting', val);
                tableView.param = val.hint;
                tableView.ascending = !tableView.ascending;
                rpc.updateView('table', {
                    'param': tableView.param,
                    'ascending': tableView.ascending
                });

            })
        });
    },
    update: function (rows, footer) {
        document.getElementById('table-body').innerHTML = rows;
        document.getElementById('table-footer').innerHTML = footer;
    },
    update_json: function (data) {
        // ! Rows
        var row_data = data['rows'];

        /*        var rows = row_data.map(function (row) {
                    return formatRow(row)
                }).reduce(function (acc, next) {
                    acc + next;
                }, "");
        */
        var rows = row_data.map(function (row) {
            return formatRow(row)
        }).reduce(function (acc, next) {
            return acc + next;
        }, "");

        rpc.log(rows);

        rpc.log(rows);
        document.getElementById('table-body').innerHTML = rows;

        // ! Footer
        var footer = formatFooter(data['footer']);
        document.getElementById('table-footer').innerHTML = footer;
    },
    /*

        ! Footer:
    */
}

var levelView = {
    level: 0,
    //chart: null,
    init: function () {
        this.level = 48;
        //var chart = 
        //this.chart = 
        //c3.generate
        //rpc.log("CHART", this.chart)
    },
    update: function (data) {
        var level = data['level'];
        var times = data['times'];
        this.level = level;
        //chart.load
        //targets horizontal bars/colouring
        c3.generate({
            bindto: '#chart',
            data: {
                columns: [
                    ['Times'].concat(times)
                ]
            },
            axis: {
                x: {
                    tick: {
                        count: 20,
                        format: function (d) {
                            return Math.floor(d);
                        }
                    },
                },
                y: {
                    label: {
                        text: 'Times',
                        position: 'outer-middle'
                    },
                    tick: {
                        format: d3.format("") // ADD
                    }
                },
                y2: {
                    show: true,
                    label: {
                        text: 'Targets',
                        position: 'outer-middle'
                    }
                }
            },
            point: {
                show: false
            },
            size: {
                width: 768,
                height: 450,
            },
            padding: {
                right: 20,
            },
            grid: {
                x: {
                    show: false
                },
                y: {
                    lines: [{
                            value: 4500,
                            text: 'Label on line 1',
                            class: 'professional'
                        },
                        {
                            value: 4200,
                            text: 'Label on line 2',
                            class: 'godlike'
                        },
                        {
                            value: 3000,
                            text: 'Label on line 3',
                            class: 'wr'
                        }
                    ]
                }
            },
            zoom: {
                enabled: true,
                rescale: true
            }
        });
    }
}

function formatTime(time) {
    var pos = true;
    if (time < 0) {
        pos = false;
        time = -time;
    }
    var hdrs = parseInt(time % 100);
    var sec = parseInt((time / 100)) % 60;
    var min = parseInt((time / (100 * 60))) % 60;
    var hrs = parseInt(time / (100 * 60 * 60));
    var lz = d3.format("02d");
    var str = (pos) ? '' : '-';
    str = str + ((hrs > 0) ? (lz(hrs) + ':') : '');
    str = str + lz(min) + ':' + lz(sec) + ',' + lz(hdrs);
    return str;
}

function formatTimeDiff(time) {
    var pos = true;
    if (time < 0) {
        pos = false;
        time = -time;
    }

    var hdrs = parseInt(time % 100);
    var sec = parseInt((time / 100)) % 60;
    var min = parseInt((time / (100 * 60))) % 60;
    var hrs = parseInt(time / (100 * 60 * 60));
    var lz = d3.format("02d");
    var str = (pos) ? '+' : '-';
    str = str + ((hrs > 0) ? (hrs + ':') : '');
    str = str + ((hrs > 0) ? ((min > 0) ? lz(min) : '') : (min > 0) ? (min + ':') : '');
    str = str + ((min > 0) ? lz(sec) : sec) + ',';
    str = str + lz(hdrs);
    return str;
}
// ! Row
// ! {"lev_number": lev_number,
// ! "lev_name": lev_name,
// ! "pr" : {"time": pr, "class": pr_class},
// ! "wr_beat": { "time": time_b, "class": wr_b_class, "table": table_b, "kuski": kuski_b },
// ! "wr_not_beat": { "time": time_nb, "class": wr_nb_class, "table": table_nb, "kuski": kuski_nb },
// ! "target": {"time": target, "class": target_class}}
function formatRow(row) {
    rpc.log(row);
    var lev_number = row.lev_number;
    var lev_name = row.lev_name;
    var pr = row['pr'];
    var wr_beat = row['wr_beat'];
    var wr_beat_time = wr_beat.time != 0 ? wr_beat.time : "-";
    var wr_beat_class = wr_beat.time != 0 ? wr_beat.class : "-";
    var wr_beat_kuski = wr_beat.time != 0 ? wr_beat.kuski : "-";
    var wr_beat_table = wr_beat.time != 0 ? wr_beat.table : "-";
    var wr_not_beat = row['wr_not_beat'];
    var wr_not_beat_time = wr_not_beat.time != 0 ? wr_not_beat.time : "-";
    var wr_not_beat_class = wr_not_beat.time != 0 ? wr_not_beat.class : "-";
    var wr_not_beat_kuski = wr_not_beat.time != 0 ? wr_not_beat.kuski : "-";
    var wr_not_beat_table = wr_not_beat.time != 0 ? wr_not_beat.table : "-";
    var target = row['target'];
    return " \
    <tr> \
        <td>" + // * level
        lev_number + ". " + lev_name +
        "</td> \
        <td class=\"" +
        pr.class + "\">" + // * pr
        (isNaN(pr.time) ? "-" : formatTime(pr.time)) +
        "</td> \
        <td class=\"" +
        wr_beat_class + "\">" + // * wr beat
        (isNaN(wr_beat_time) ? "-" : formatTime(wr_beat_time)) +
        " <span class=\"diff\">(<em><strong>" +
        (isNaN(pr.time - wr_beat_time) ? "-" : formatTimeDiff(pr.time - wr_beat_time)) +
        "</em></strong>)</span></td> \
        <td>" + // * kuski beat
        wr_beat_kuski +
        " (<em><strong>" +
        wr_beat_table +
        "</em></strong>)</td> \
        <td class=\"" +
        wr_not_beat_class + "\">" + // ! target wr
        (isNaN(wr_not_beat_time) ? wr_not_beat_time : formatTime(wr_not_beat_time)) +
        " <span class=\"diff\">(<em><strong>" +
        formatTimeDiff(pr.time - wr_not_beat_time) +
        "</em></strong>)</span></td> \
        <td>" + // ! target kuski
        wr_not_beat_kuski +
        " (<em><strong>" +
        wr_not_beat_table +
        "</em></strong>)</td> \
        <td class=\"" +
        target.class + "\">" + // ! target
        formatTime(target.time) +
        " <span class=\"diff\">(<em><strong>" +
        formatTimeDiff(pr.time - target.time) +
        "</em></strong>)</span></td> \
    </tr>"
}
// ! Footer
// {"p_tt": p_tt.0, "target_wr_tt": target_wr_tt.0, "target_tt": target_tt.0}
function formatFooter(footerData) {
    var p_tt = footerData['p_tt'];
    var target_wr_tt = footerData['target_wr_tt'];
    var target_tt = footerData['target_tt'];
    return " \
    <tr> \
        <td><\/td> \
        <td id=\"p_tt\" class=\"tt\">" +
        formatTime(p_tt) +
        "<\/td><td><\/td><td><\/td>" +
        "<td id=\"target_wr_tt\" class=\"tt\">" +
        formatTime(target_wr_tt) +
        " (<em><strong>" +
        formatTimeDiff(p_tt - target_wr_tt) +
        "<\/em><\/strong>)" +
        "<\/td> \
        <td><\/td> \
        <td>" +
        formatTime(target_tt) +
        " (<em><strong>" +
        formatTimeDiff(p_tt - target_tt) +
        "</em></strong>) \
        <\/td> \
    <\/tr>"
}