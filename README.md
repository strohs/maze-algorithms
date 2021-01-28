# Maze Generation Algorithms

Examples of various [maze generation algorithms](https://en.wikipedia.org/wiki/Maze_generation_algorithm) written
in Rust, Java and JavaScript (NodeJS).  All these algorithms were inspired by the book
[Mazes for Programmers](https://pragprog.com/titles/jbmaze/mazes-for-programmers/)
They can be run from the command line and will output the final maze to the terminal using ASCII characters and
ANSI escape sequences.

The implemented algorithms include:
- [Prims's Algorithm](http://weblog.jamisbuck.org/2011/1/10/maze-generation-prim-s-algorithm)
- [Recursive Backtracking](https://weblog.jamisbuck.org/2010/12/27/maze-generation-recursive-backtracking)
- [Recursive Division](http://weblog.jamisbuck.org/2011/1/12/maze-generation-recursive-division-algorithm)


### Prerequisites
- Java 14 if you want to run the Java algorithms
- Python 3 (at least 3.8) for the python algorithms
- NodeJs (at least version 11, but older versions may work) for the javascript algorithms
- Rust (at least version 1.4, but older versions may work) and cargo


### Running
- Java
    - located in the `java-mazes` directory. Each algorithm is in its own Java class with a main method,
      so just run the class directly
- JavaScript (NodeJs)
    - located in the `javascript-mazes` directory, run each algorithm individually using node, i.e.:
        - `node prims`
        - `node recursive_backtracking`
        - `node recursive_division`
- Python
    - located in the `python-mazes` directory, you can run each individual algorithm via the python command i.e:
        - `python prims`
        - `python recursive_backtracking`
        - `python recursive_division`
- Rust
    - located in the `rust-mazes` directory, the algorithms are packaged as a cargo library project.
      There is a `main.rs` file that will start all the algorithms and print the results to the console.
      The easiest way to run is via `cargo run`