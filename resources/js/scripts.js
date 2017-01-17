var socket = new WebSocket("wss://rbs.io:8080");

socket.onopen = function (event) {
    // Discover then send lat, long, and tz.
    var d = new Date();
    var tz = d.getTimeZoneOffset();
    navigator.geolocation.getCurrentPosition().then(function(position){
        var greeting = {
            payload: "Hey Northship."
            lat: position.coords.latitude,
            long: position.coords.longitude,
            tz: tz
        };
    });

}

socket.onmessage = function (event) {
// Append the payload to the conversation div.

}

window.onload = function () {

}
