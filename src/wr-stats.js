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
    updateTableView: function () {
        rpc.request({
            cmd: 'updateTableView',
            param: tableView.param,
            ascending: tableView.ascending
        });
    },
    updateLevelView: function () {
        rpc.request({
            cmd: 'updateLevelView',
            level: levelView.level
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
                break;
            case 'level':
                levelView.init();
                break;
        }

    },
    update: function (arg) {
        var obj = JSON.parse(arg);
        if (this.activeView == obj['view']) {
            switch (this.activeView) {
                case 'table':
                    tableView.update(obj['rows'], obj['footer']);
                    break;
                case 'level':
                    levelView.update(obj['data']['level'], obj['data']['times']);
                    break;
            }
        }
    }
}

var tableView = {
    param: "LevelNum",
    ascending: true,
    init: function () {
        //for (var key in colSortHint) {
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
                rpc.updateTableView();
            })
        });

        rpc.updateTableView();
    },
    update: function (rows, footer) {
        document.getElementById('table-body').innerHTML = rows;
        document.getElementById('table-footer').innerHTML = footer;
        this.ascending = !this.ascending;
    },
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

        rpc.updateLevelView();
    },
    update: function (level, times) {
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