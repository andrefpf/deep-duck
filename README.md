<div align="center">
  <img width="100" src="img/logo_white.png" alt="ExpiraBot Logo" align="center">
</div>
<h1 align="center" style="margin-top: 20px;"> Deep Duck </h1>


## What?
Deep duck is an engine to evaluate and sugest moves for the duck chess variant.

## What is duck chess?
Duck Chess is a chess variant invented by Dr. Tim Paulden in 2016. The game follows most regular chess rules but adds a dynamic component to the mix: a rubber duck that both players can move.
You can learn more about it [here](https://www.chess.com/terms/duck-chess) and play some duckchess in [chess.com](https://www.chess.com/variants/duck-chess) or in [pychess](https://www.pychess.org/).

## Is it a good chess engine?
Yeah, I think. It is much better than me, and it won the best engine I found online. [Here](https://www.pychess.org/4fKk39R2?ply=72) you can see a game Deep Duck played against Pychess strongest engine!

To be fair, pychess is using fairy stockfish and the duck evaluation isn't fully finished yet. But common, Deep Duck also isn't finished.

## Wait, is this a conversation with yourself?
[Maybe](https://pt.wikipedia.org/wiki/Esquizofrenia). Next question.

## Nice, how it works? 
This was implemented in Rust using the old fashioned way: 

- Taking inspiration on Deep Blue, its grandfather

- Testing (almost) every move possible

- Repeat this process for you and for your enemy some times

- Then use a hand crafted evaluation to see if the outcome looks like a good position for you or not

If you are interestd in more details, here we go:

- I am using [Negamax](https://www.chessprogramming.org/Negamax) with [alpha-beta](https://www.chessprogramming.org/Alpha-Beta) pruning.

- To represent the board I am using the [mailbox](https://www.chessprogramming.org/Mailbox) aproach, were I have an 64 sized array of pieces or empty squares. The other aproach usually involves some [bitwise wizardry](https://www.chessprogramming.org/Bitboards).

- For performance reasons I just test 3 duck positions: the previous position of the moved piece, the position in front of the piece we think the enemy wants to move, the position we think the enemy wants to put the duck. That misses some cases were the duck can block two pieces the same time, but in general seems a very usefull heuristics.

## How can I use this?
In the future you will play it [here](https://andrefpf.github.io/duckmate/), but it is not working yet.

In the meantime you can compile and run the CLI, then put in the FEN of your chess position. The engine will show the position and say the best the best movement it finds.

To help you with the FEN stuff use this [nice editor](https://www.pychess.org/editor/duck) from pychess, then copy and paste the FEN code.

    Your FEN position: 8/3*4/8/8/8/4K3/8/7k w - - 0 1 q
    8                 
    7                 
    6                 
    5                 
    4                 
    3                 
    2           ♚   🐤 
    1               ♔ 
      A B C D E F G H
    Move: E3 to F2 and duck to H2
    Time elapsed: 31.667347ms

## How can I compile it?
You will need Cargo. Then run this in your terminal: 

    cargo run --release

It is important to use the --release, because chess engine is a very time consuming task and every optimization is wellcome.
