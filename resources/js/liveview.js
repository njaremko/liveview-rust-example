import morphdom from "morphdom";

window.morphdom = morphdom;

let conn = null;

let content = document.getElementById('content');
// let parser = new DOMParser();

function connect() {
    disconnect();
    let wsUri = (window.location.protocol === 'https:' && 'wss://' || 'ws://') + window.location.host + '/ws/';
    conn = new WebSocket(wsUri);
    console.log('Connecting...');
    conn.onopen = function () {
        console.log('Connected.');
    };
    conn.onmessage = function (e) {
        let new_content = document.createElement('div');
        new_content.setAttribute('id', 'content');
        // let body = parser.parseFromString(, 'text/html').firstChild;
        // new_content.appendChild(body);
        new_content.innerHTML = e.data;
        morphdom(content, new_content);
    };
    conn.onclose = function () {
        console.log('Disconnected.');
        conn = null;
    };
}

function disconnect() {
    if (conn != null) {
        log('Disconnecting...');
        conn.close();
        conn = null;
    }
}

function send_event(event) {
    conn.send(event);
}

connect();

let clickElems = document.querySelectorAll('[rust-click]');
for (let i = 0; i < clickElems.length; i++) {
    clickElems[i].addEventListener('click', function(e) {
        e.preventDefault();
        console.log(e);
        console.log(clickElems[i].getAttribute('rust-click'));
        // console.log(conn);
        send_event(clickElems[i].getAttribute('rust-click'));
    });
}