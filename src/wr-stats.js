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
            this.log('request:', JSON.stringify(arg));
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
            case 'tt':
                ttView.init();
                rpc.updateView('tt', {
                    'none': 'none'
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
                    case 'tt':
                        ttView.update(obj['data']);
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
                case 'tt':
                    rpc.updateView(this.activeView, ttView.getArg());
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
        //var row_data = data['rows'];

        //var rows = row_data.map(function (row) {
        //    return formatRow(row)
        //}).reduce(function (acc, next) {
        //    return acc + next;
        //}, "");

        document.getElementById('table-body').innerHTML = data['rows'];
        //var footer = formatFooter(data['footer']);
        document.getElementById('table-footer').innerHTML = data['footer'];

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
    init: function () { },
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
                height: 800,
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
                    lines: getGridTargetLines(pr, targets)
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

var ttView = {
    init: function () { },
    update: function (data) {
        var target_tts = data['target_tts'];
        var pr = data['pr_tt'];
        var wr_tts = data['wr_tts'];
        c3.generate({
            bindto: '#chart',
            data: {
                columns: [
                    ['TTs'].concat(wr_tts)
                ],
            },
            axis: {
                x: {
                    label: {
                        // text: 'Level ' + level.toString(),
                        // position: 'outer-left'
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
                    max: pr > wr_tts[0] ? Math.ceil(pr) : Math.ceil(wr_tts[0]),
                    label: {
                        text: 'TTs',
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
                height: 800,
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
                    lines: getGridTargetLines(pr, target_tts)
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
            'none': 'none'
        };
    }
}

function getGridTargetLines(pr, targets) {
    return [{
        value: pr,
        text: 'PR ' + formatTimeShort(pr),
        position: 'left'
    },
    {
        value: targets.godlike,
        text: 'Godlike ' + formatTimeShort(targets.godlike) + " (" + formatTimeDiff(pr - targets.godlike) +
            ")",
        class: 'godlike'
    },
    {
        value: targets.legendary,
        text: 'Legendary ' + formatTimeShort(targets.legendary) + " (" + formatTimeDiff(pr - targets.legendary) + ")",
        class: 'legendary'
    },
    {
        value: targets.world_class,
        text: 'World class ' + formatTimeShort(targets.world_class) + " (" + formatTimeDiff(pr - targets.world_class) + ")",
        class: 'world_class'
    },
    {
        value: targets.professional,
        text: 'Professional ' + formatTimeShort(targets.professional) + " (" + formatTimeDiff(pr - targets.professional) + ")",
        class: 'professional'
    },
    {
        value: targets.good,
        text: 'Good ' + formatTimeShort(targets.good) + " (" + formatTimeDiff(pr - targets.good) + ")",
        class: 'good'
    },
    {
        value: targets.ok,
        text: 'Ok ' + formatTimeShort(targets.ok) + " (" + formatTimeDiff(pr - targets.ok) + ")",
        class: 'ok'
    },
    {
        value: targets.beginner,
        text: 'Beginner' + formatTimeShort(targets.beginner) + " (" + formatTimeDiff(pr - targets.beginner) + ")",
        class: 'beginner'
    },
    ]
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