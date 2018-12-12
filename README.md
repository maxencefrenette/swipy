# Swipy

Swipy is a 2048 AI using modern AI techniques.

## Goals

* Implement an open-source 2048 engine that is fast and achieves high scores;
* Demonstrate the capabilities of Rust as a replacement for C/C++ for computationally-intensive tasks in a reinforcement learning context;
* Acquire experience with reinforcement learning

## Features

* Average score (1-ply): ~23,000
* Average score (3-ply): ~60,000

* [x] Expectimax search
* [x] N-tuple network v-function
  * [x] Learn afterstates
* [x] TD(0) learning
* [ ] Multi-stage learning (game phases)

## Previous Work

Swipy is largely based on the following implements and papers about 2048 engines.

* <https://github.com/kaito4213/2048-Game-Player>
* <https://github.com/nneonneo/2048-ai>
  * <https://stackoverflow.com/questions/22342854/what-is-the-optimal-algorithm-for-the-game-2048>
* [Temporal Difference Learning of N-Tuple Networks for the Game 2048](http://www.cs.put.poznan.pl/mszubert/pub/szubert2014cig.pdf)
  * [Slides](https://www.researchgate.net/profile/Wojciech_Jaskowski/publication/265178223_Temporal_Difference_Learning_of_N-Tuple_Networks_for_the_Game_2048_presentation_at_Computational_Intelligence_in_Games_Dortmund_2014/links/54043d1b0cf2c48563b05f5d/Temporal-Difference-Learning-of-N-Tuple-Networks-for-the-Game-2048-presentation-at-Computational-Intelligence-in-Games-Dortmund-2014.pdf)
* [Multi-Stage Temporal Difference Learning for 2048-like Games](https://arxiv.org/abs/1606.07374)
* [Mastering 2048 with Delayed Temporal Coherence Learning, Multi-Stage Weight Promotion, Redundant Encoding and Carousel Shaping](https://arxiv.org/abs/1604.05085)
