# Maze Generation Algorithms

Examples of various [maze generation algorithms](https://en.wikipedia.org/wiki/Maze_generation_algorithm) written
in Rust, Java, JavaScript (NodeJS) and Python.  All these algorithms were inspired by the book
[Mazes for Programmers](https://pragprog.com/titles/jbmaze/mazes-for-programmers/).

The code is intended to be run from the command line and will pretty print the generated maze to the terminal.

The implemented algorithms include:
- [Binary Tree Algorithm](http://weblog.jamisbuck.org/2011/2/1/maze-generation-binary-tree-algorithm)
- [Prims's Algorithm](http://weblog.jamisbuck.org/2011/1/10/maze-generation-prim-s-algorithm)
- [Recursive Backtracking](https://weblog.jamisbuck.org/2010/12/27/maze-generation-recursive-backtracking)
- [Wilson's Algorithm](https://en.wikipedia.org/wiki/Maze_generation_algorithm)
- [Aldous-Broder Algorithm](https://en.wikipedia.org/wiki/Maze_generation_algorithm)
- [Hunt and Kill Algorithm](https://weblog.jamisbuck.org/2011/1/24/maze-generation-hunt-and-kill-algorithm)
- [Sidewinder Algorithm](http://weblog.jamisbuck.org/2011/2/3/maze-generation-sidewinder-algorithm)

And [Dijkstra's Algorithm](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm) to find the shortest path between
two points in a maze.


### Prerequisites
- Java 14 if you want to run the Java algorithms (although any Java version >= Java 8 should work)
- Python 3 (at least 3.8) for the python algorithms
- NodeJs (at least version 11, but older versions may work) for the javascript algorithms
- Rust (at least version 1.4, but older versions may work) and cargo


### Running
- Java
    - all java code is located in the `java` directory. Each algorithm is in its own Java class with a main method,
      so just run the class directly

- JavaScript (NodeJs)
    - all javascript code is located in the `nodejs` directory, run the main.js file:  `node main.js` to kick off
    an example script that will generate a 10x15 maze using each maze generation algorithm.
    
- Python
    - all Python code is located in the `python` directory. 
    - Run the `example.py` file via the python 3 interpreter: `python example.py`. This will generate 10x15 mazes
    using the various algorithms
      
- Rust
    - located in the `rust` directory. Example files have been provided for each maze algorithm in the `examples`
    directory. Use the `cargo run --example`, command to see an ... "example" of each maze type:
    - i.e.  `cargo run --example wilsons 20 30` to generate a maze of 20 columns and 30 rows using Wilson's algorithm