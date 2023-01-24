## How it looks like to make this work
```
[andre@fedora duckmate-engine]$ cargo run --release
   Compiling duckmate-engine v0.1.0 (/home/andre/Documentos/duckmate-engine)
    Finished release [optimized] target(s) in 1.07s
     Running `target/release/duckmate-engine`

Your FEN position: rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
8 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ 
7 ♙ ♙ ♙ ♙ ♙ ♙ ♙ ♙ 
6           🐤     
5                 
4                 
3         ♟       
2 ♟ ♟ ♟ ♟   ♟ ♟ ♟ 
1 ♜ ♞ ♝ ♛ ♚ ♝ ♞ ♜ 
  A B C D E F G H
Move: E2 to E3 and duck to F6
Time elapsed: 1.39845902s

Your FEN position: rnbqkbnr/ppp1pppp/8/3p4/8/3*P3/PPPP1PPP/RNBQKBNR w KQkq - 0 1 
8 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ 
7 ♙ ♙ ♙ 🐤 ♙ ♙ ♙ ♙ 
6                 
5       ♙         
4                 
3     ♞   ♟       
2 ♟ ♟ ♟ ♟   ♟ ♟ ♟ 
1 ♜   ♝ ♛ ♚ ♝ ♞ ♜ 
  A B C D E F G H
Move: B1 to C3 and duck to D7
Time elapsed: 4.994489727s

Your FEN position: rnbqkbnr/ppp2ppp/8/3pp3/3*4/2N1P3/PPPP1PPP/R1BQKBNR w KQkq - 0 1
8 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ 
7 ♙ ♙ ♙     ♙ ♙ ♙ 
6     🐤           
5   ♝   ♙ ♙       
4                 
3     ♞   ♟       
2 ♟ ♟ ♟ ♟   ♟ ♟ ♟ 
1 ♜   ♝ ♛ ♚   ♞ ♜ 
  A B C D E F G H
Move: F1 to B5 and duck to C6
Time elapsed: 5.079903387s

Your FEN position: rnbqkbnr/1pp*1ppp/p7/1B1pp3/8/2N1P3/PPPP1PPP/R1BQK1NR w KQkq - 0 1
8 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ 
7   ♙ ♙     ♙ ♙ ♙ 
6 ♙               
5       ♙ ♙       
4 ♝               
3   🐤 ♞   ♟       
2 ♟ ♟ ♟ ♟   ♟ ♟ ♟ 
1 ♜   ♝ ♛ ♚   ♞ ♜ 
  A B C D E F G H
Move: B5 to A4 and duck to B3
Time elapsed: 2.643392928s

Your FEN position: rnbqkbnr/2p2ppp/p7/1p1pp3/B1*5/2N1P3/PPPP1PPP/R1BQK1NR w KQkq - 0 1
8 ♖ ♘ ♗ ♕ ♔ ♗ ♘ ♖ 
7     ♙     ♙ ♙ ♙ 
6 ♙         🐤     
5   ♙   ♙ ♙       
4                 
3   ♝ ♞   ♟       
2 ♟ ♟ ♟ ♟   ♟ ♟ ♟ 
1 ♜   ♝ ♛ ♚   ♞ ♜ 
  A B C D E F G H
Move: A4 to B3 and duck to F6
Time elapsed: 727.597367ms

Your FEN position: rn1qkbnr/2p2ppp/p7/1p1pp3/6b1/1BN1P3/PPPP*PPP/R1BQK1NR w KQkq - 0 1
8 ♖ ♘   ♕ ♔ ♗ ♘ ♖ 
7     ♙     ♙ ♙ ♙ 
6 ♙               
5   ♙   ♙ ♙ 🐤     
4             ♗   
3   ♝ ♞   ♟ ♟     
2 ♟ ♟ ♟ ♟     ♟ ♟ 
1 ♜   ♝ ♛ ♚   ♞ ♜ 
  A B C D E F G H
Move: F2 to F3 and duck to F5
Time elapsed: 1.080275561s

Your FEN position: rn1qkbnr/1*p2ppp/p7/1p2p3/3p2b1/1BN1PP2/PPPP2PP/R1BQK1NR w KQkq - 0 1
8 ♖ ♘   ♕ ♔ ♗ ♘ ♖ 
7     ♙     ♙ ♙ ♙ 
6 ♙               
5   ♙     ♙ 🐤     
4       ♙     ♗   
3   ♝     ♟ ♟     
2 ♟ ♟ ♟ ♟ ♞   ♟ ♟ 
1 ♜   ♝ ♛ ♚   ♞ ♜ 
  A B C D E F G H
Move: C3 to E2 and duck to F5
Time elapsed: 536.7255ms

Your FEN position: rn1qkbnr/2p2ppp/p7/1p2p2b/3p2*1/1B2PP2/PPPPN1PP/R1BQK1NR w KQkq - 0 1
8 ♖ ♘   ♕ ♔ ♗ ♘ ♖ 
7     ♙     ♙ ♙ ♙ 
6 ♙               
5   ♙     ♙     ♗ 
4     🐤 ♟         
3   ♝       ♟     
2 ♟ ♟ ♟ ♟ ♞   ♟ ♟ 
1 ♜   ♝ ♛ ♚   ♞ ♜ 
  A B C D E F G H
Move: E3 to D4 and duck to C4
Time elapsed: 365.719354ms

Your FEN position: rn1qkbnr/2p2ppp/p7/1p5b/3p2*1/1B3P2/PPPPN1PP/R1BQK1NR w KQkq - 0 1
8 ♖ ♘   ♕ ♔ ♗ ♘ ♖ 
7     ♙     ♙ ♙ ♙ 
6 ♙               
5   ♙           ♗ 
4       ♙         
3   ♝ ♟     ♟     
2 ♟ ♟ 🐤 ♟ ♞   ♟ ♟ 
1 ♜   ♝ ♛ ♚   ♞ ♜ 
  A B C D E F G H
Move: C2 to C3 and duck to C2
Time elapsed: 3.167477561s

Your FEN position: rn1qkbnr/2p2ppp/p7/1p5b/8/1BPp1P1*/PP1PN1PP/R1BQK1NR w KQkq - 0 1
8 ♖ ♘   ♕ ♔ ♗ ♘ ♖ 
7     ♙     ♙ ♙ ♙ 
6 ♙           🐤   
5   ♙           ♗ 
4           ♞     
3   ♝ ♟ ♙   ♟     
2 ♟ ♟   ♟     ♟ ♟ 
1 ♜   ♝ ♛ ♚   ♞ ♜ 
  A B C D E F G H
Move: E2 to F4 and duck to G6
Time elapsed: 879.212172ms

Your FEN position: rn1qkbnr/2p2ppp/8/pp5b/5N2/1BPp1P*1/PP1P2PP/R1BQK1NR w KQkq - 0 1
8 ♖ ♘   ♕ ♔ ♗ ♘ ♖ 
7     ♙   🐤 ♙ ♙ ♙ 
6                 
5 ♙ ♙           ♞ 
4                 
3   ♝ ♟ ♙   ♟     
2 ♟ ♟   ♟     ♟ ♟ 
1 ♜   ♝ ♛ ♚   ♞ ♜ 
  A B C D E F G H
Move: F4 to H5 and duck to E7
Time elapsed: 1.42605814s

Your FEN position: rn1qkbnr/2p2p1p/6p1/pp5N/5*2/1BPp1P2/PP1P2PP/R1BQK1NR w KQkq - 0 1
8 ♖ ♘   ♕ ♔ ♗ ♘ ♖ 
7     ♙   🐤 ♙   ♙ 
6             ♙   
5 ♙ ♙             
4                 
3   ♝ ♟ ♙   ♟ ♞   
2 ♟ ♟   ♟     ♟ ♟ 
1 ♜   ♝ ♛ ♚   ♞ ♜ 
  A B C D E F G H
Move: H5 to G3 and duck to E7
Time elapsed: 455.152399ms

Your FEN position: rn1qkbnr/2p2p1p/6p1/1p1*4/p7/1BPp1PN1/PP1P2PP/R1BQK1NR w KQkq - 0 1
8 ♖ ♘   ♕ ♔ ♗ ♘ ♖ 
7     ♙   🐤 ♙   ♙ 
6             ♙   
5   ♙             
4 ♙               
3     ♟ ♙   ♟ ♞   
2 ♟ ♟ ♝ ♟     ♟ ♟ 
1 ♜   ♝ ♛ ♚   ♞ ♜ 
  A B C D E F G H
Move: B3 to C2 and duck to E7
Time elapsed: 560.789519ms

Your FEN position: rn1qkbnr/2p2p1p/6p1/1p1*4/p7/2P2PN1/PPpP2PP/R1BQK1NR w KQkq - 0 1
8 ♖ ♘   ♕ ♔ ♗ ♘ ♖ 
7     ♙   🐤 ♙   ♙ 
6             ♙   
5   ♙             
4 ♙               
3     ♟     ♟ ♞   
2 ♟ ♟ ♛ ♟     ♟ ♟ 
1 ♜   ♝   ♚   ♞ ♜ 
  A B C D E F G H
Move: D1 to C2 and duck to E7
Time elapsed: 1.302568944s

Your FEN position: r2qkbnr/2p2p1p/n5p1/1p6/p2*4/2P2PN1/PPQP2PP/R1B1K1NR w KQkq - 0 1
8 ♖     ♕ ♔ ♗ ♘ ♖ 
7     ♙   🐤 ♙   ♙ 
6 ♘           ♙   
5   ♙             
4 ♙       ♛       
3     ♟     ♟ ♞   
2 ♟ ♟   ♟     ♟ ♟ 
1 ♜   ♝   ♚   ♞ ♜ 
  A B C D E F G H
Move: C2 to E4 and duck to E7
Time elapsed: 3.320511353s

Your FEN position: 1r1qkbnr/2p2p1p/n5p1/1p2*3/p3Q3/2P2PN1/PP1P2PP/R1B1K1NR w KQkq - 0 1
8   ♖   ♕ ♔ ♗ ♘ ♖ 
7     ♙ 🐤   ♙   ♙ 
6 ♘   ♛       ♙   
5   ♙             
4 ♙               
3     ♟     ♟ ♞   
2 ♟ ♟   ♟     ♟ ♟ 
1 ♜   ♝   ♚   ♞ ♜ 
  A B C D E F G H
Move: E4 to C6 and duck to D7
Time elapsed: 5.231026555s

Your FEN position: 1r1q1bnr/2p1kp1p/n*Q3p1/1p6/p7/2P2PN1/PP1P2PP/R1B1K1NR w KQkq - 0 1
8   ♖   ♕   ♗ ♘ ♖ 
7   🐤 ♙   ♔ ♙   ♙ 
6 ♘           ♙   
5   ♛             
4 ♙               
3     ♟     ♟ ♞   
2 ♟ ♟   ♟     ♟ ♟ 
1 ♜   ♝   ♚   ♞ ♜ 
  A B C D E F G H
Move: C6 to B5 and duck to B7
Time elapsed: 2.10312702s

Your FEN position: r2q1bnr/*1p1kp1p/n5p1/1Q6/p7/2P2PN1/PP1P2PP/R1B1K1NR w KQkq - 0 1
8 ♖ 🐤   ♕   ♗ ♘ ♖ 
7   ♛ ♙   ♔ ♙   ♙ 
6 ♘           ♙   
5                 
4 ♙               
3     ♟     ♟ ♞   
2 ♟ ♟   ♟     ♟ ♟ 
1 ♜   ♝   ♚   ♞ ♜ 
  A B C D E F G H
Move: B5 to B7 and duck to B8
Time elapsed: 5.154745654s

Your FEN position: r1*q1bnr/1Qp1kp1p/6p1/2n5/p7/2P2PN1/PP1P2PP/R1B1K1NR w KQkq - 0 1
8 ♛ 🐤   ♕   ♗ ♘ ♖ 
7     ♙   ♔ ♙   ♙ 
6             ♙   
5     ♘           
4 ♙               
3     ♟     ♟ ♞   
2 ♟ ♟   ♟     ♟ ♟ 
1 ♜   ♝   ♚   ♞ ♜ 
  A B C D E F G H
Move: B7 to A8 and duck to B8
Time elapsed: 928.704286ms

Your FEN position: Q2q1bnr/2p1kp1p/6p1/8/p7/2Pn1PN1/PP1P2PP/R1B*K1NR w KQkq - 0 1
8 ♛   🐤 ♕   ♗ ♘ ♖ 
7     ♙   ♔ ♙   ♙ 
6             ♙   
5                 
4 ♙               
3     ♟ ♘   ♟ ♞   
2 ♟ ♟   ♟     ♟ ♟ 
1 ♜   ♝     ♚ ♞ ♜ 
  A B C D E F G H
Move: E1 to F1 and duck to C8
Time elapsed: 422.320258ms

Your FEN position: Q*1q1bnr/2p1kp1p/6p1/8/p7/2P2PN1/PP1P2PP/R1n2KNR w KQkq - 0 1
8       ♕   ♗ ♘ ♖ 
7     ♙ 🐤 ♔ ♙   ♙ 
6             ♙   
5                 
4 ♙       ♛       
3     ♟     ♟ ♞   
2 ♟ ♟   ♟     ♟ ♟ 
1 ♜   ♘     ♚ ♞ ♜ 
  A B C D E F G H
Move: A8 to E4 and duck to D7
Time elapsed: 1.417546727s

Your FEN position: 3q1bnr/2p1kp1p/6p1/4*3/p3Q3/1nP2PN1/PP1P2PP/R4KNR w KQkq - 0 1
8       ♕   ♗ ♘ ♖ 
7     ♙ 🐤 ♔ ♙   ♙ 
6             ♙   
5                 
4 ♙       ♛       
3   ♟ ♟     ♟ ♞   
2   ♟   ♟     ♟ ♟ 
1 ♜         ♚ ♞ ♜ 
  A B C D E F G H
Move: A2 to B3 and duck to D7
Time elapsed: 593.383358ms

Your FEN position: 3q1bnr/2p1kp1p/4*1p1/8/4Q3/1pP2PN1/1P1P2PP/R4KNR w KQkq - 0 1
8       ♕   ♗ ♘ ♖ 
7     ♙ 🐤 ♔ ♙   ♙ 
6             ♙   
5                 
4   ♛             
3   ♙ ♟     ♟ ♞   
2   ♟   ♟     ♟ ♟ 
1 ♜         ♚ ♞ ♜ 
  A B C D E F G H
Move: E4 to B4 and duck to D7
Time elapsed: 5.219630233s

Your FEN position: 3qkbnr/2p2p1p/6p1/8/1Q2*3/1pP2PN1/1P1P2PP/R4KNR w KQkq - 0 1
8       ♕ ♔ ♗ ♘ ♖ 
7     ♙ 🐤   ♙   ♙ 
6             ♙   
5   ♛             
4                 
3   ♙ ♟     ♟ ♞   
2   ♟   ♟     ♟ ♟ 
1 ♜         ♚ ♞ ♜ 
  A B C D E F G H
Move: B4 to B5 and duck to D7
Time elapsed: 2.706416928s

Your FEN position: 3q1bnr/2p1kp1p/6p1/1Q6/1*6/1pP2PN1/1P1P2PP/R4KNR w KQkq - 0 1
8       ♕   ♗ ♘ ♖ 
7     ♙ 🐤 ♔ ♙   ♙ 
6             ♙   
5   ♛             
4                 
3   ♙ ♟     ♟ ♞   
2   ♟   ♟ ♞   ♟ ♟ 
1 ♜         ♚   ♜ 
  A B C D E F G H
Move: G1 to E2 and duck to D7
Time elapsed: 5.154972059s

Your FEN position: 3q2nr/2p1kpbp/*5p1/1Q6/8/1pP2PN1/1P1PN1PP/R4K1R w KQkq - 0 1
8       ♕     ♘ ♖ 
7     ♙ 🐤 ♔ ♙ ♗ ♙ 
6             ♙   
5                 
4                 
3   ♛ ♟     ♟ ♞   
2   ♟   ♟ ♞   ♟ ♟ 
1 ♜         ♚   ♜ 
  A B C D E F G H
Move: B5 to B3 and duck to D7
Time elapsed: 5.952345713s

Your FEN position: 3q2nr/2p1k1bp/6p1/5p2/8/1QP*1PN1/1P1PN1PP/R4K1R w KQkq - 0 1
8       ♕     ♘ ♖ 
7     ♙ 🐤 ♔   ♗ ♙ 
6             ♙   
5       ♛   ♙     
4                 
3     ♟     ♟ ♞   
2   ♟   ♟ ♞   ♟ ♟ 
1 ♜         ♚   ♜ 
  A B C D E F G H
Move: B3 to D5 and duck to D7
Time elapsed: 5.667697407s

Your FEN position: 3q3r/2p1k1bp/3*1np1/3Q1p2/8/2P2PN1/1P1PN1PP/R4K1R w KQkq - 0 1
8       ♕       ♖ 
7     ♙ 🐤 ♔   ♗ ♙ 
6           ♘ ♙   
5     ♛     ♙     
4                 
3     ♟     ♟ ♞   
2   ♟   ♟ ♞   ♟ ♟ 
1 ♜         ♚   ♜ 
  A B C D E F G H
Move: D5 to C5 and duck to D7
Time elapsed: 3.096425611s

Your FEN position: 3qk2r/2p3bp/5np1/2Q2p2/2*5/2P2PN1/1P1PN1PP/R4K1R w KQkq - 0 1
8       ♕ ♔     ♖ 
7     ♙ 🐤     ♗ ♙ 
6     ♛     ♘ ♙   
5           ♙     
4                 
3     ♟     ♟ ♞   
2   ♟   ♟ ♞   ♟ ♟ 
1 ♜         ♚   ♜ 
  A B C D E F G H
Move: C5 to C6 and duck to D7
Time elapsed: 6.365642523s

Your FEN position: 3q3r/2p2kbp/2Q2np1/5p2/2*5/2P2PN1/1P1PN1PP/R4K1R w KQkq - 0 1
8       ♕     🐤 ♖ 
7     ♙     ♔ ♗ ♙ 
6     ♛     ♘ ♙   
5           ♙     
4       ♞         
3     ♟     ♟ ♞   
2   ♟   ♟     ♟ ♟ 
1 ♜         ♚   ♜ 
  A B C D E F G H
Move: E2 to D4 and duck to G8
Time elapsed: 4.257436208s

Your FEN position: 3q3r/2p2kbp/2Q2np1/5*2/3N1p2/2P2PN1/1P1P2PP/R4K1R w KQkq - 0 1
8       ♕       ♖ 
7     ♙     ♔ ♗ ♙ 
6     ♛ 🐤   ♘ ♙   
5                 
4       ♞   ♙     
3     ♟     ♟     
2   ♟   ♟ ♞   ♟ ♟ 
1 ♜         ♚   ♜ 
  A B C D E F G H
Move: G3 to E2 and duck to D6
Time elapsed: 2.021400438s

Your FEN position: 3q1r2/2p*1kbp/2Q2np1/8/3N1p2/2P2P2/1P1PN1PP/R4K1R w KQkq - 0 1
8       ♕   ♖ 🐤   
7     ♙     ♔ ♗ ♙ 
6         ♛ ♘ ♙   
5                 
4       ♞   ♙     
3     ♟     ♟     
2   ♟   ♟ ♞   ♟ ♟ 
1 ♜         ♚   ♜ 
  A B C D E F G H
Move: C6 to E6 and duck to G8
Time elapsed: 4.888636175s

Your FEN position: 3qkr2/2p1*1bp/4Qnp1/8/3N1p2/2P2P2/1P1PN1PP/R4K1R w KQkq - 0 1
8       ♕ ♔ ♖     
7     ♙ 🐤     ♗ ♙ 
6     ♞   ♛ ♘ ♙   
5                 
4           ♙     
3     ♟     ♟     
2   ♟   ♟ ♞   ♟ ♟ 
1 ♜         ♚   ♜ 
  A B C D E F G H
Move: D4 to C6 and duck to D7
Time elapsed: 4.529610296s

Your FEN position: 3qkr1b/2p1*2p/2N1Qnp1/8/5p2/2P2P2/1P1PN1PP/R4K1R w KQkq - 0 1
8 ♜   🐤 ♕ ♔ ♖   ♗ 
7     ♙         ♙ 
6     ♞   ♛ ♘ ♙   
5                 
4           ♙     
3     ♟     ♟     
2   ♟   ♟ ♞   ♟ ♟ 
1           ♚   ♜ 
  A B C D E F G H
Move: A1 to A8 and duck to C8
Time elapsed: 3.505745325s

Your FEN position: R2qkr1b/2p1*3/2N1Qnpp/8/5p2/2P2P2/1P1PN1PP/5K1R w KQkq - 0 1
8 🐤     ♜ ♔ ♖   ♗ 
7     ♙           
6     ♞   ♛ ♘ ♙ ♙ 
5                 
4           ♙     
3     ♟     ♟     
2   ♟   ♟ ♞   ♟ ♟ 
1           ♚   ♜ 
  A B C D E F G H
Move: A8 to D8 and duck to A8
Time elapsed: 1.303437929s

Your FEN position: *2Rkr1b/2p5/2N1Qn1p/6p1/5p2/2P2P2/1P1PN1PP/5K1R w KQkq - 0 1
8       🐤 ♜ ♖   ♗ 
7     ♙           
6     ♞   ♛ ♘   ♙ 
5             ♙   
4           ♙     
3     ♟     ♟     
2   ♟   ♟ ♞   ♟ ♟ 
1           ♚   ♜ 
  A B C D E F G H
Move: D8 to E8 and duck to D8
Time elapsed: 1.114931681s
```