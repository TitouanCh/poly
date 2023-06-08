# Devlogs


## **Journey to making a game with a dedicated server using Godot and Rust**
___
I. Preface

II. Building out the client

III. Working on the server

IV. Conclusion
___

**I. Preface**

*Animation of a message being sent*

*Footage of online games*

Narrator (voiceover): Hey there,

Narrator (voiceover): Odds are, if you're watching this video, you've got access to the internet and you've probably played some kind of online game if not consumed hundreds of hours playing them.

Narrator (voiceover): But, what does it take to make one? And with dedicated servers?

Narrator (voiceover): This is something I've been wondering about lately. Just how far, can one inexperienced person go in building an online game from scratch?

Narrator (voiceover): I'm ghost wave, let's dive in.

**II. Building a client**

Narrator (voiceover): First I'll need to choose a game engine or a graphics library to build the client for my game. To make my life easier I'm going to stick with what I'm most experienced with. The Godot game engine.

*Animation of me building the game*

Narrator (voiceover): Here we go, I've got a simple terrain renderer. A way to move the camera and a simple way to place objects. Right now there isn't really an objective to the game, but this doesn't really matter to me yet. I just want to see if I can get multiple clients linked up and able to place balls together on this map.

Narrator (voiceover): Now that I've got a simple game working it was tempting to jump directly into server code. Alas, before I do that, I need some basic UI for the clients.

*Animation of the different needed UI screens*

Narrator (voiceover): I need a login screen, a screen the list of all the games and a lobby screen. For fun I've also added a global and in game chat window.

**III. Working on the server**

Narrator (voiceover): Now, this first part was easy enought but now I'm getting into the deep end.

Narrator (voiceover): I've never programmed a game server before so my approach should be taken with a grain of salt.

Narrator (voiceover): I'm using Rust with tokio for asynchonous communications. That just means that the server can keep track of multiple tasks in a non blocking way.

Narrator (voiceover): I'm using TCP which makes everything easier as there is no loss of packets. However, it's a lot slower than UDP but this doesn't really matter in the context of the game which is right now just about placing balls.

*Demo*

Narrator (voiceover): Let's start it up and see it in action.

Narrator (voiceover): The global chat is working.

Narrator (voiceover): I can create a game and I automaticly join it.

Narrator (voiceover): My friend sees that the game has been created and can join as wall.

Narrator (voiceover): We can place balls together and... chat!

**IV. Conclusion**

Narrator (voiceover): Creating this little online game was a lot of fun!

Narrator (voiceover): I've also tried running this little demo on a remote server and was successful in connecting to that as well so I guess this is all functional.

Narrator (voiceover): I might continue working on this and try to make a tiny strategy game, thought no promises.

Narrator (voiceover): Thank you for watching to the end. See you...