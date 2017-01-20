var socket = new WebSocket("wss://rbs.io:8080");

socket.onopen = function (event) {
    // Discover then send lat, long, and tz.
    var d = new Date();
    var tz = d.getTimezoneOffset();
    navigator.geolocation.getCurrentPosition(function(position){
        var greeting = {
            payload: "Hey Northship.",
            lat: position.coords.latitude,
            long: position.coords.longitude,
            tz: tz
        };
        socket.send(JSON.stringify(greeting));
    });

}

socket.onmessage = function (event) {
// Append the payload to the conversation div.
    console.log(event.data);
    var message = JSON.parse(event.data);
    document.getElementById("conversation").append(message.payload);
}

window.onload = function () {

}
