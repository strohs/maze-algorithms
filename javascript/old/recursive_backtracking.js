/// Recursive Backtracking
/// Here’s the mile-high view of **recursive backtracking**:
///
/// 1. Choose a starting point in the field.
/// 2. Randomly choose a wall at that point and carve a passage through to the adjacent cell, but
///    only if the adjacent cell has not been visited yet. This becomes the new current cell.
/// 3. If all adjacent cells have been visited, back up to the last cell that has uncarved walls and repeat.
/// 4. The algorithm ends when the process has backed all the way up to the starting point.
///
/// cx,cy is the current cell being visited

const ansi = require('ansi-escape-sequences');
const Direction = require("./direction");
const Maze = require("./Maze");


/**
 * @returns a random integer between min(inclusive) and max(exclusive)
 */
function getRndInteger(min, max) {
    return Math.floor(Math.random() * (max - min) ) + min;
}

/**
 * shuffles the elements of an array in place
 * @param arr - the array to shuffle
 * @returns {*} the passed in array with its elements shuffled
 */
function shuffle(arr) {
    let rand, temp, i;

    for (i = arr.length - 1; i > 0; i -= 1) {
        rand = getRndInteger(0, i+1);
        temp = arr[rand];//swap i and the zero-indexed number
        arr[rand] = arr[i];
        arr[i] = temp;
    }
    return arr;
}



// prints the maze as ASCII characters to the console
function displayMaze(maze) {
    const width = maze.width;
    const height = maze.height;

    console.log(" _".repeat(width));

    for (let h=0; h < height; h++) {
        let row = "|";
        for (let w=0; w < width; w++) {
            (maze.field[h][w] & Direction.S) !== 0 ? row += " " : row += "_";

            if ((maze.field[h][w] & Direction.E) !== 0) {
                (((maze.field[h][w] | maze.field[h][w+1]) & Direction.S) !== 0) ? row += " " : row += "_";
            } else {
                row += "|";
            }
        }
        console.log(row);
    }
}



/**
 * performs recursive backtracking on the passed in "empty" maze
 * @param cx - cell x position to begin the algorithm at, 0 is the leftmost column of maze
 * @param cy - cell y position to begin the algorithm at, 0 is the topmost row of the maze
 * @param maze - the empty maze to use for the algorithm
 */
function recurse(cx, cy, maze) {
    // choose a random direction
    let directions = [Direction.N, Direction.S, Direction.E, Direction.W];
    shuffle(directions);

    for (const dir of directions) {
        const nx = cx + Direction.dx(dir);
        const ny = cy + Direction.dy(dir);

        if (ny >= 0 && ny < maze.field.length && nx >= 0 && nx < maze.field[ny].length && maze.field[ny][nx] === 0) {
            maze.field[cy][cx] |= dir;
            maze.field[ny][nx] |= Direction.opposite(dir);

            recurse(nx, ny, maze);
        }
    }
}


width = process.argv.length > 2 ? parseInt(process.argv[2], 10) : 20
height = process.argv.length > 3 ? parseInt(process.argv[3], 10) : 20
const maze = new Maze(width, height);
// perform recursive backtracking
recurse(0, 0, maze);
displayMaze(maze);

