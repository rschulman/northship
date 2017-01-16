var socket = new WebSocket("wss://rbs.io:8080");

socket.onopen = function (event) {
// Discover then send lat, long, and tz.

}

socket.onmessage = function (event) {
// Append the payload to the conversation div.

}

window.onload = function () {

}
