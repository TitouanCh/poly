const http = require("http");
const express = require("express");
const socketio = require("socket.io");

const { cp } = require("fs");
const Game = require("./game");

const app = express();

const clientPath = `${__dirname}/../client`;
console.log(`Serving static from ${clientPath}`);

app.use(express.static(clientPath));

const server = http.createServer(app);

const io = socketio(server);

var unknows = [];

var game1 = new Game();

io.on('connection', (sock) => {
    console.log("Detected connection --");
    var temp_id = unknows.length;
    unknows.push(sock);
    sock.emit("logon");

    sock.on("login-attempt", (username) => {
        unknows = unknows.splice(temp_id, 1)
        game1.connectPlayer(username, sock);
    });
});

server.on('error', (err) => {
    console.error('Server error:', err);
});

server.listen(3000, () => {
    console.log('RPS started on 3000');
});