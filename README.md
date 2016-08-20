# Akari player and solver

See [here](https://en.wikipedia.org/wiki/Light_Up_&#40puzzle&#41) for the rules of the game.

The solver is written in Rust, and can be built using Cargo.

The player is written in Elm, with the puzzle generation and server written in Python. You will
need Flask to run the server.

You will also need a SAT solver to run the puzzle solver. The puzzle solver was tested using
[Glucose](http://www.labri.fr/perso/lsimon/glucose/).

In order to play, build the solver using Cargo. Then, edit the configuration variables in
[puzzle_generator.py](player/puzzle_generator.py) to point to the puzzle solver binary and the
Glucose binary, and run the server by executing app.py.
