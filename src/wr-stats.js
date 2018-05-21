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
            //this.log('request:', JSON.stringify(arg));
        }
        window.external.invoke(JSON.stringify(arg));
    },
    displayView: function (view) {
        this.request({
            cmd: 'displayView',
            view: view,
        });
    },
    updateView: function (view, arg) {
        this.request({
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
        this.request({
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
    updateView: function (arg) {
        if (arg) {
            var obj = JSON.parse(arg);
            if (this.activeView == obj['view']) {
                switch (this.activeView) {
                    case 'table':
                        tableView.update(obj['data']);
                        break;
                    case 'level':
                        levelView.update(obj['data']);
                        break;
                }
            }
        } else {
            switch (this.activeView) {
                case 'table':
                    rpc.updateView(this.activeView, tableView.getArg());
                    break;
                case 'level':
                    rpc.updateView(this.activeView, levelView.getArg());
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
    update: function (data) {
        var row_data = data['rows'];

        var rows = row_data.map(function (row) {
            return formatRow(row)
        }).reduce(function (acc, next) {
            return acc + next;
        }, "");

        document.getElementById('table-body').innerHTML = rows;
        var footer = formatFooter(data['footer']);
        document.getElementById('table-footer').innerHTML = footer;

        util.range(54).map(function (i) {
            document.getElementById('lev-' + (i + 1).toString())
                .addEventListener("click", function () {
                    levelView.level = i + 1;
                    rpc.request({
                        cmd: 'displayView',
                        view: 'level'
                    });
                })
        });
    },
    getArg: function () {
        return {
            'param': this.param,
            'ascending': this.ascending
        };
    }
}

var levelView = {
    level: 1,
    //chart: null,
    init: function () {},
    update: function (data) {
        var level = data['level'];
        var times = data['times'];
        var targets = data['targets'];
        var pr = data.pr;
        this.level = level;
        c3.generate({
            bindto: '#chart',
            data: {
                columns: [
                    ['Times'].concat(times)
                ],
            },
            axis: {
                x: {
                    label: {
                        text: 'Level ' + level.toString(),
                        position: 'outer-left'
                    },
                    tick: {
                        fit: true,
                        count: 20,
                        format: function (d) {
                            return Math.round(d);
                        }
                    },
                },
                y: {
                    max: pr > times[0] ? Math.ceil(pr) : Math.ceil(times[0]),
                    label: {
                        text: 'Times',
                        position: 'outer-middle'
                    },
                    tick: {
                        fit: true,
                        format: formatTimeShort
                    }
                },
            },
            point: {
                show: false
            },
            size: {
                // width: 768,
                height: 900,
            },
            padding: {
                right: 90,
            },
            grid: {
                x: {
                    show: false
                },
                y: {
                    show: false,
                    lines: [{
                            value: pr,
                            text: 'PR ' + formatTimeShort(pr),
                            position: 'left'
                        },
                        {
                            value: targets.godlike,
                            text: 'Godlike ' + formatTimeShort(targets.godlike),
                            class: 'godlike'
                        },
                        {
                            value: targets.legendary,
                            text: 'Legendary ' + formatTimeShort(targets.legendary),
                            class: 'legendary'
                        },
                        {
                            value: targets.world_class,
                            text: 'World class ' + formatTimeShort(targets.world_class),
                            class: 'world_class'
                        },
                        {
                            value: targets.professional,
                            text: 'Professional ' + formatTimeShort(targets.professional),
                            class: 'professional'
                        },
                        {
                            value: targets.good,
                            text: 'Good ' + formatTimeShort(targets.good),
                            class: 'good'
                        },
                        {
                            value: targets.ok,
                            text: 'Ok ' + formatTimeShort(targets.ok),
                            class: 'ok'
                        },
                        {
                            value: targets.beginner,
                            text: 'Beginner',
                            class: 'beginner'
                        },
                    ]
                }
            },
            zoom: {
                enabled: true,
                rescale: true
            }
        });
    },
    getArg: function () {
        return {
            'level': levelView.level
        };
    }
}

var util = {
    range: function (i) {
        return Array.apply(null, Array(i)).map(function (_, j) {
            return j;
        });
    }
}

// ! Time formats
function formatTime(time) {
    if (isNaN(time)) {
        return '-'
    }
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

function formatTimeShort(time) {
    if (isNaN(time)) {
        return '-'
    }
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
    str = str + ((hrs > 0) ? (hrs + ':') : '');
    str = str + ((hrs > 0) ? ((min > 0) ? lz(min) : '') : (min > 0) ? (min + ':') : '');
    str = str + ((min > 0) ? lz(sec) : sec) + ',';
    str = str + lz(hdrs);
    return str;
}

function formatTimeDiff(time) {
    if (isNaN(time)) {
        return '-'
    }
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

// ! HTML generation
// * Row
// * {"lev_number": lev_number,
// * "lev_name": lev_name,
// * "pr" : {"time": pr, "class": pr_class},
// * "wr_beat": { "time": time_b, "class": wr_b_class, "table": table_b, "kuski": kuski_b },
// * "wr_not_beat": { "time": time_nb, "class": wr_nb_class, "table": table_nb, "kuski": kuski_nb },
// * "target": {"time": target, "class": target_class}}
function formatRow(row) {
    var lev_number = row.lev_number;
    var lev_name = row.lev_name;
    var pr = row['pr'];
    return "<tr id=\"lev-" + lev_number + "\"><td>" +
        lev_number + ". " + lev_name +
        "</td><td class=\"" +
        pr.class + "\">" + // * pr
        formatTime(pr.time) +
        "</td>" +
        formatTimeEntry(row['wr_beat'], pr.time) +
        formatTimeEntry(row['wr_not_beat'], pr.time) +
        formatTimeEntry(row['target'], pr.time) +
        "</tr>"
}

// * Not very robust
function formatTimeEntry(entry, pr) {
    if (entry['time'] == 0) {
        return "<td>-</td><td>-</td>";
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

// * Footer
// * {"p_tt": p_tt.0, "target_wr_tt": target_wr_tt.0, "target_tt": target_tt.0}
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
        "<\/em><\/strong>)<\/td><td><\/td><td>" +
        formatTime(target_tt) +
        " (<em><strong>" +
        formatTimeDiff(p_tt - target_tt) +
        "</em></strong>) \
        <\/td> \
    <\/tr>"
}

/*
function (p_tt, tt) {
    return "<td id=\"target_wr_tt\" class=\"tt\">" +
        formatTime(tt) +
        " (<em><strong>" +
        formatTimeDiff(p_tt - tt) +
        "<\/em><\/strong>)<\/td>";
}
*/