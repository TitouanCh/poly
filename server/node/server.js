const net = require('net');

let server = net.createServer((socket) => {
	socket.on('data', (data) => {
		console.log("Received: " + data);
	});
	console.log("Accepted connection.");
	socket.write("Hello from the server!\n");
}).listen(3000, () => console.log("Listening on 3000."));