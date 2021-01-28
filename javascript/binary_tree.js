const Maze = require("./Maze");
const Direction = require('./direction');

// Binary Tree algorithm is one of the simplest maze generation algorithms:
//
// 1. start at a corner of the maze (in this case it will be the North West)
// 2. iterate through the cells row by row
// 3.     for each cell pick a random East or South wall to remove
// 4.     repeat until all cells have been visited

/* returns a random integer between 0 (inclusive) and max(exclusive) */
function rand(max) {
    return Math.floor(Math.random() * max );
}


function generate(width, height) {
    const east_most_cell = (x) => x === width - 1;
    const south_most_cell = (y) => y === height - 1;

    let maze = new Maze(width, height);
    const es_directions = [Direction.E, Direction.S];

    for (let y = 0; y < maze.height; y++) {
        for (let x = 0; x < maze.width; x++) {
            if (east_most_cell(x) && south_most_cell(y)) {
                continue;
            } else if (east_most_cell(x)) {
                maze.field[y][x] |= Direction.S;
            } else if (south_most_cell(y)) {
                maze.field[y][x] |= Direction.E;
            } else {
                maze.field[y][x] |= es_directions[rand(2)];
            }
        }
    }
    return maze;
}

width = process.argv.length > 2 ? parseInt(process.argv[2], 10) : 20
height = process.argv.length > 3 ? parseInt(process.argv[3], 10) : 20
maze = generate(width, height);
maze.display();
