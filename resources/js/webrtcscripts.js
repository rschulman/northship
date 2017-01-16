var signal_socket = new WebSocket("wss://rbs.io:8080");
var pc;

function startCall () {
	pc = new RTCPeerConnection();
	
	pc.onicecandidate = function (evt) {
		signal_socket.send(JSON.stringify({ "candidate": evt.candidate }));
	};

	navigator.getUserMedia({ "audio": true, "video": false }, function (stream) {
		pc.addStream(stream);
		pc.createOffer().then(function(offer) {
			return pc.setLocalDescription(offer)
		})
		.then(function() {
			signal_socket.send(JSON.stringify({ "sdp": pc.localDescription }));
		});
	});
}

window.onload = function () {
	startCall();
}
