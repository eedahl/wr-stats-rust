'use strict';

var ascending = true;
var param = "LevelNum"

window.onload = function () {
    document.getElementById('lev')
        .addEventListener("click", function () {
            updateSortedBy('LevelNum')
        });
    document.getElementById('pr')
        .addEventListener("click", function () {
            updateSortedBy('PR')
        });
    document.getElementById('wr_beat')
        .addEventListener("click", function () {
            updateSortedBy('DiffToPrevWR')
        });
    document.getElementById('kuski_beat')
        .addEventListener("click", function () {
            updateSortedBy('Table')
        });
    document.getElementById('target_wr')
        .addEventListener("click", function () {
            updateSortedBy('DiffToNextWR')
        });
    document.getElementById('kuski_to_beat')
        .addEventListener("click", function () {
            updateSortedBy('Table')
        });
    document.getElementById('target')
        .addEventListener("click", function () {
            updateSortedBy('DiffToNextTarget')
        });
    /*
        c3.generate({
            bindto: '#chart',
            data: {
                columns: [
                    ['data1', 30, 200, 100, 400, 150, 250],
                    ['data2', 50, 20, 10, 40, 15, 25]
                ]
            }
        });
    */
};

function updateSortedBy(par) {
    param = par;
    ascending = !ascending;
    rpc.updateSorted(param, ascending);

}

function updateSorted() {
    rpc.sort(param, ascending);
}

function updateTableRows(rows) {
    document.getElementById('table-body').innerHTML = rows;
}

function updateTableFooter(footer) {
    document.getElementById('table-footer').innerHTML = footer;
}

var rpc = {
    invoke: function (arg) {
        window.external.invoke(JSON.stringify(arg));
    },
    updateSorted: function (param, ascending) {
        rpc.invoke({
            cmd: 'updateSorted',
            param: param,
            ascending: ascending
        });
    },
}