'use strict';

window.onload = function () {
    window.onerror = function () {
        rpc.log(arguments);
    }
    rpc.displayView('table');
};

var rpc = {
    displayView: function (view) {
        rpc.request({
            cmd: 'displayView',
            view: view,
        });
    },
    updateTableView: function (param, ascending) {
        rpc.request({
            cmd: 'updateTableView',
            param: param,
            ascending: ascending
        });
    },
    request: function (arg) {
        if (arg['cmd'] != 'log') {
            rpc.log('request:', JSON.stringify(arg));
        }
        window.external.invoke(JSON.stringify(arg));
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

var view = {
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
                    document.getElementById('table-body').innerHTML = obj['rows'];
                    document.getElementById('table-footer').innerHTML = obj['footer'];
                    break;
                case 'level':
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
        var colSortHint = {
            'lev': 'LevelNum',
            'pr': 'PR',
            'wr-beat': 'DiffToPrevWR',
            'kuski-beat': 'Table',
            'target-wr': 'DiffToNextWr',
            'kuski-to-beat': 'Table',
            'target': 'DiffToNextTarget',
        };
        for (var key in colSortHint) {
            rpc.log(key, colSortHint[key]);
            (function () {
                var k = colSortHint[key];
                document.getElementById(key).addEventListener("click", function () {
                    rpc.log('sorting', k)
                    tableView.param = k;
                    tableView.update();
                });
            }());
        }
        rpc.updateTableView(this.param, this.ascending);
    },
    update: function () {
        this.ascending = !this.ascending;
        rpc.log('update', this.param);
        rpc.updateTableView(this.param, this.ascending);
    },
}

var levelView = {
    level: 0,
    init: function () {
        c3.generate({
            bindto: '#chart',
            data: {
                columns: [
                    ['data1', 30, 200, 100, 400, 150, 250],
                    ['data2', 50, 20, 10, 40, 15, 25]
                ]
            }
        });
    },
    update: function (level) {
        this.level = level;
    },
}