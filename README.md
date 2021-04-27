# Maze Generation Algorithms

Examples of various [maze generation algorithms](https://en.wikipedia.org/wiki/Maze_generation_algorithm) written
in Rust, Java, JavaScript (NodeJS) and Python.  All these algorithms were inspired by the book
[Mazes for Programmers](https://pragprog.com/titles/jbmaze/mazes-for-programmers/). They will each generate 
a random, perfect maze, which can then be printed to the terminal as ASCII characters.



The implemented algorithms include:
- [Binary Tree Algorithm](http://weblog.jamisbuck.org/2011/2/1/maze-generation-binary-tree-algorithm)
- [Prims's Algorithm](http://weblog.jamisbuck.org/2011/1/10/maze-generation-prim-s-algorithm)
- [Recursive Backtracking](https://weblog.jamisbuck.org/2010/12/27/maze-generation-recursive-backtracking)
- [Wilson's Algorithm](https://en.wikipedia.org/wiki/Maze_generation_algorithm)
- [Aldous-Broder Algorithm](https://en.wikipedia.org/wiki/Maze_generation_algorithm)
- [Hunt and Kill Algorithm](https://weblog.jamisbuck.org/2011/1/24/maze-generation-hunt-and-kill-algorithm)
- [Sidewinder Algorithm](http://weblog.jamisbuck.org/2011/2/3/maze-generation-sidewinder-algorithm)

Also provided is an example of using [Dijkstra's Algorithm](https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm), to
find the shortest path between two cells in a maze.


### Prerequisites
- Java 8 (or higher) if you want to run the Java examples
- Python 3 (at least 3.8) for the python examples
- NodeJs (at least version 11, but older versions may work) for the javascript examples
- Rust (at least version 1.4, but older versions may work) and Cargo for the Rust examples


### Running
- Java
    - the java code is located in the `java` directory. The `com.mazes.Examples` class contains examples of how to
      run each algorithm, plus how to find the shortest path through a maze.

- JavaScript (NodeJs)
    - all javascript code is located in the `nodejs` directory, run the main.js file:  `node main.js` to kick off
    an example script that will generate a 10x15 maze using each maze generation algorithm.
    
- Python
    - all Python code is located in the `python` directory. 
    - Run the `example.py` file via your python 3 interpreter: `python example.py <height> <width>`. height and width
      are optional integer values that will set the height and width of the generated mazes. If not passed, they 
      default to 10 x 15.
      
- Rust
    - located in the `rust` directory. Example files have been provided for each maze algorithm in the `examples`
    directory. Use the `cargo run --example`, command to see an ... "example" of each maze type:
    - i.e.  `cargo run --example wilsons 20 30` to generate a maze of 20 columns and 30 rows using Wilson's algorithm