# Devlogs


## **Building a strategy combat engine**

___
I. Introduction and inspiration for the project

II. Tools for the 2d prototype

III. Features

IV. Going foward
___

If you've played any amount of PC games, you're probably already familiar with the behemoth that are RTS games. From Starcraft to ... if you've spent any amount of time on the internet you've certainly stumbled upon footage of one of them.

A common problem for the genre is that these types of of game are hard to first start out in. And have a massive entry curve. This could explain why these types of game have fallen out of favor in the past decade being overshadowed by quote unquote simpler games: MOBAs and FPS.

This fact has inspired me to start this small, purely theoretical for now, project:
What would it take to make a simpler and easy to pickup game in the RTS genre.

I've built a small prototype of an RTS in the godot game engine, of
course I won't have the time to design a full game from the ground up in a single video, so this prototype is small in scope, still, let's try to tackle the most important issues that make RTS so hard to pickup and play.

I. APM, having no time to make many any decisions.
One big barrier to entry of RTS games is just how fast you need to play most effeciently, how you need to micro eve...

A possible way to solve 

___

There are many games out there that allow you to control massive armies and let you battle hundred if not thousands of soldiers toghether.

Mostly RTSs and strategy games.

What does it take to make a game like those, what does it take to build a combat engine.

Now first, what exactly do I mean by a combat engine?

I am talking about a program that takes units and soldier data and simulates the outcome of a battle.

Sorta off like how a physics engine takes objects' mass and position and predicts where they will end up next. 

Before we start programming this engine, let me first start by defining what I mean by some of the terms I am using

In my engine, individual soldiers are represented as dots, I will agree that it's not the prettiest but graphics aren't the focus here.

I will call units groups of dots/soldiers.

Let's first start by implementing basic movement for our soldiers.

...

As you can see, movement of the soldiers is a rigid and stuttery, let's fix that by slowing them once they get close to their target destination using linear interpolation.

Much better!

On a sidenote, as you can see, I've made it so soldiers first rotate before moving, to me this looks more realistic but of course I'm open to feedback in the comments.

Next, it would be nice if soldiers didn't phase throught each other. This is why I then added unit collision. This will be more useful once I add a way for units to charge and react to impacts;

And finally, I've added basic melee combat. Once units are close enought, the closest soldiers will attack. I've made it so that the soldiers attack in wave so as to create a sort of frontline that gets progressively messier over time.

There is still a tiny bug at the end where the soldiers don't collide but other than that this system works fine for now

Perpective, right now this is all just a very basic and simple system. I'm using GDscript for everything but the plan is to move to a more performant language so as to be able to simulate many units.

I was thinking of Rust and keeping the Godot frontend for display.

Let me know your thoughts... And thank you for watching.

I. Unit movement

a) linear

b) lerp

c) turning then moving

II. Unit selection and teams

III. Melee Combat

a) 

Sources: https://www.youtube.com/watch?v=MzaLrGCDit0