<div align="center">
  <img width="100" src="img/logo_white.png" alt="ExpiraBot Logo" align="center">
</div>
<h1 align="center" style="margin-top: 20px;"> Deep Duck </h1>


## What?
Deep duck is an engine to evaluate and sugest moves for the duck chess variant.

## Why?
When I found this [game](https://www.chess.com/terms/duck-chess) and played it [online](https://www.chess.com/variants/duck-chess), I quickly discovered that I was pretty bad at it. Then I thought `"Well, I will play against some bots and then come back here"`. To my surprise bots didn't existed yet, so I made my own.

## Is it a good chess engine?
Yeah, it already is much better than me (not that hard), and it is already stronger than the stockfish engine used in [Pychess](https://www.pychess.org/). Here you can see two games played between Duck Chess and Fairy Stockfish.

**SPOILER ALERT:** Deep duck won both!

### As white
<img width="400" src="https://user-images.githubusercontent.com/48336152/214456234-71d418fa-1140-495a-a151-a9f43580fe9a.gif">

[see more](https://www.pychess.org/KjgWhlfk?ply=1)

### As black
<img width="400" src="https://user-images.githubusercontent.com/48336152/214456696-4c4863b2-415f-4383-8e43-d9aacb1ee4ea.gif">

[see more](https://www.pychess.org/rAiY9Qzd?ply=1)

## Nice, how it works? 
This was implemented in Rust using the old fashioned way: 

- Taking inspiration on Deep Blue, its grandfather

- Testing (almost) every move possible

- Repeat this process for you and for your enemy some times

- Then use a hand crafted evaluation to see if the outcome looks like a good position for you or not

If you are interestd in more details, here we go:

- I am using [Negamax](https://www.chessprogramming.org/Negamax) with [alpha-beta](https://www.chessprogramming.org/Alpha-Beta) pruning and a simple move ordering.

- To represent the board I am using the [mailbox](https://www.chessprogramming.org/Mailbox) aproach, were I have an 64 sized array of pieces or empty squares. The other aproach usually involves some [bitwise wizardry](https://www.chessprogramming.org/Bitboards).

- For performance reasons I just test 3 duck positions: the previous position of the moved piece, the position blocking the piece we think the enemy wants to move, the position we think the enemy wants to put the duck. That misses some cases were the duck can block two pieces the same time, but in general seems a very usefull heuristics.

## How can I use this?
In the future you will play it [here](https://andrefpf.github.io/duckmate/), but it is not working yet.

In the meantime you can compile and run the CLI, then put in the FEN of your chess position. The engine will show the position and say the best the best movement it finds.

To help you with the FEN stuff use this [nice editor](https://www.pychess.org/editor/duck) from pychess, then copy and paste the FEN code.
Here is an example of how you can use Deep Duck to evaluate a position for your game.

<a href="https://asciinema.org/a/OFZCXZ1pbIXUTJ2qtm7iRu16v" target="_blank">
<img width=600 src="https://asciinema.org/a/OFZCXZ1pbIXUTJ2qtm7iRu16v.svg" /></a>

## How can I compile it?
You will need Cargo. Then run this in your terminal: 

    cargo run --release

It is important to use the --release, because chess engine is a very time consuming task and every optimization is wellcome.
