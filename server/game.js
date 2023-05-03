class Game {
    constructor() {
        this._players = {};
        this._turn = 1;
    }

    connectPlayer(username, sock) {
        if (this._players[username]) {
            console.log(`${username} has already joined previously.`);
            this._players[username].sock = sock;
        } else {
            this._players[username] = {
                name : username,
                sock : sock
            };
            console.log(`Created dataslot for ${username}.`)
        }

        console.log("Connected players:")
        for (const [key, player] of Object.entries(this._players)) {
            console.log(player.name);
        }

        /* Connect signals
        sock.on("build", (what, slot) => {
            this.build(username, sock, what, slot);
        });
        */

        // sock.emit("sendCity", this._players[username].state);
    }

    /*
    build(username, sock, what, slot) {
        this._players[username].state.slots[slot].b = what;
        sock.emit("sendCity", this._players[username].state);
    }
    */
}

module.exports = Game;