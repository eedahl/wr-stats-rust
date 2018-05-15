'use strict';

var ascending = true;
var param = "LevelNum"

$(function () {
    $('#lev').click(function () { sortUpdateBy('LevelNum') });
    $('#pr').click(function () { sortUpdateBy('PR') });
    $('#wr_beat').click(function () { sortUpdateBy('DiffToPrevWR') });
    $('#kuski_beat').click(function () { sortUpdateBy('Table') });
    $('#target_wr').click(function () { sortUpdateBy('DiffToNextWR') });
    $('#kuski_to_beat').click(function () { sortUpdateBy('Table') });
    $('#target').click(function () { sortUpdateBy('DiffToNextTarget') });

    var trace1 = {
        x: [1, 2, 3, 4],
        y: [10, 15, 13, 17],
        type: 'scatter'
      };
      
      var trace2 = {
        x: [1, 2, 3, 4],
        y: [16, 5, 11, 9],
        type: 'scatter'
      };
      
      var data = [trace1, trace2];
      /*
        <!-- Plots go in blank <div> elements. 
    You can size them in the plot layout,
    or give the div a size as shown here.
        -->
    <div id="tester" style="width:90%;height:250px;"></div>
    */
      
      //Plotly.newPlot('wr-table', data);
});

function sortUpdateBy(par) {
    param = par;
    ascending = !ascending;
    rpc.sort(param, ascending);

}

function sortUpdate() {
    rpc.sort(param, ascending);
}

function updateTable(rows) {
    $('#wr-table-rows').html(rows);
}

function updateSidebar(sidebar) {
    $('#sidebar').html(sidebar);
}

var rpc = {
    invoke: function (arg) { window.external.invoke(JSON.stringify(arg)); },
    sort: function (param, ascending) { rpc.invoke({ cmd: 'sort', param: param, ascending: ascending }); },
}

/*

function UI(items) {
    var h = picodom.h;
    function submit(e) {
        e.preventDefault();
        e.stopImmediatePropagation();
        var el = document.getElementById('task-name-input');
        rpc.addTask(el.value);
        el.value = '';
    }
    function clearTasks() { rpc.clearDoneTasks(); }
    function markTask(i) { return function () { rpc.markTask(i, !items[i].done); } };

    var taskItems = [];
    for (var i = 0; i < items.length; i++) {
        var checked = (items[i].done ? 'checked' : 'unchecked');
        taskItems.push(
            h('div', { className: 'task-item ' + checked, onclick: markTask(i) },
                items[i].name));
    }

    return h('div', { className: 'container' },
        h('form', { className: 'text-input-wrapper', onsubmit: submit },
            h('input', {
                id: 'task-name-input',
                className: 'text-input',
                type: 'text',
                autofocus: true
            })),
        h('div', { className: 'task-list' }, taskItems),
        h('div', { className: 'footer' },
            h('div', { className: 'btn-clear-tasks', onclick: clearTasks },
                'Delete completed')));
}

var element;
var oldNode;
var rpc = {
    invoke: function (arg) { window.external.invoke(JSON.stringify(arg)); },
    init: function () { rpc.invoke({ cmd: 'init' }); },
    log: function () {
        var s = '';
        for (var i = 0; i < arguments.length; i++) {
            if (i != 0) {
                s = s + ' ';
            }
            s = s + JSON.stringify(arguments[i]);
        }
        rpc.invoke({ cmd: 'log', text: s });
    },
    addTask: function (name) { rpc.invoke({ cmd: 'addTask', name: name }); },
    clearDoneTasks: function () { rpc.invoke({ cmd: 'clearDoneTasks' }); },
    markTask: function (index, done) {
        rpc.invoke({ cmd: 'markTask', index: index, done: done });
    },
    render: function (items) {
        return element = picodom.patch(oldNode, (oldNode = UI(items)), element);
    },
};
*/
/*sort { param, ascending }

window.onload = function () { rpc.init(); };
*/