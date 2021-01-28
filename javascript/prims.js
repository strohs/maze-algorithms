const ansi = require('ansi-escape-sequences');
const Maze = require("./Maze");
const Direction = require('./direction');

// bit-mask used to test if a cell of the maze is "IN" the maze (i.e. has been carved out)
const IN = 0x10;        // 0001 0000

// bit-mask that tests if a cell of the maze is in the current Frontier of the maze
const FRONTIER = 0x20;  // 0010 0000


/**
 * returns a random integer between min(inclusive) and max(exclusive)
 */
function getRndInteger(min, max) {
    return Math.floor(Math.random() * (max - min) ) + min;
}


// displays one iteration (a.k.a step) of the algorithm to console.log
function display_step(maze) {
    const empty = (cell) => cell === 0 || cell === FRONTIER

    // move to upper left of terminal
    console.log(ansi.cursor.position(1,1));

    console.log(" _".repeat(maze.width));

    for (let y = 0; y < maze.height; y++) {
        let line = "|";
        for (let x = 0; x < maze.width; x++) {
            const cell = maze.field[y][x];
            if (cell === FRONTIER) {
                line += ansi.styles('bg-red');
            }
            if (empty(cell) && y + 1 < maze.field.length && empty(maze.field[y+1][x])) {
                line += " ";
            } else {
                if ((cell & Direction.S) !== 0) {
                    line += " ";
                } else {
                    line += "_";
                }
            }
            if (cell === FRONTIER) {
                line += ansi.style.reset;
            }

            const row = maze.field[y];
            if (empty(cell) && x+1 < row.length && empty(row[x+1])) {
                if (y+1 < maze.field.length && (empty(maze.field[y+1][x]) || empty(maze.field[y+1][x+1]))) {
                    line += " ";
                } else {
                    line += "_";
                }
            } else if ((cell & Direction.E) !== 0) {
                const c = ((cell | row[x+1]) & Direction.S) !== 0 ? " " : "_";
                line += c;
            } else {
                line += "|";
            }
        }
        console.log(line);
    }
}

/**
 * adds the specified cell to the maze's list of frontier cells.
 * @param maze
 * @param frontier
 * @param x the x position (column) of the cell to add
 * @param y the y position (row) of the cell to add
 * @returns {boolean} true if the cell was added to the frontier, false if it was not added (because
 * it is already in the frontier or the given x,y coordinates are out of bounds of the Maze)
 */
function addFrontier(maze, frontier, x, y) {
    if (maze.inBounds(x, y) && maze.isEmpty(x, y)) {
        maze.field[y][x] |= FRONTIER;
        frontier.push({x, y});
        return true;
    } else {
        return false;
    }
}

/**
 * marks the cell at the given x,y position as being in the maze. Also adds all neighboring cells to the frontier
 * (if those cells are not already in the frontier)
 * @param maze
 * @param frontier
 * @param x
 * @param y
 */
function markIn(maze, frontier, x, y) {
    maze.field[y][x] |= IN;
    if (x > 0) {
        addFrontier(maze, frontier,x-1, y);
    }
    addFrontier(maze, frontier,x+1, y);
    if (y > 0) {
        addFrontier(maze, frontier, x, y-1);
    }
    addFrontier(maze, frontier, x, y+1);
}


/**
 * Generates a maze using Prim's algorithm
 * Prim’s approaches maze generation from a different angle. Rather than working edgewise across the
 * entire graph, it starts at one point, and grows outward from that point. The standard version
 * of the algorithm works something like this:
 *
 * 1. Choose an arbitrary vertex from G (the graph), and add it to some (initially empty) set V.
 * 2. Choose a **random** edge from G, that connects a vertex in V with another vertex not in V.
 * 3. Add that edge to the minimal spanning tree, and the edge’s other vertex to V.
 * 4. Repeat steps 2 and 3 until V includes every vertex in G.
 *
 * @param width - width of the maze to generate
 * @param height - height of the maze to generate
 * @returns {Maze} - a completed Maze
 */
function prims(width, height) {
    let maze = new Maze(width, height);
    // an array of Javascript Objects containing the x (column) and y (row) coordinates of cells in the frontier
    let frontier = []; // [ { x: y: } ]

    // mark a random cell in the maze to begin at
    const xs = getRndInteger(0, width);
    const ys = getRndInteger(0, height);
    markIn(maze, frontier, xs, ys);

    while (frontier.length !== 0) {

        // choose a random element in the frontier and remove it
        const ridx = getRndInteger(0, frontier.length);
        let {x: fx, y: fy} = frontier.splice(ridx, 1)[0];

        // choose a random neighbor to the current frontier cell, that is currently marked as IN the maze
        const rand_neighbors = maze.neighbors(fx, fy).filter(xy => (maze.field[xy.y][xy.x] & IN) !== 0);
        let {x: rnx, y: rny} = rand_neighbors[getRndInteger(0, rand_neighbors.length)];

        // carve a path from the current frontier cell to the random neighbor cell
        const dir = Maze.removeWall(fx, fy, rnx, rny);
        maze.field[fy][fx] |= dir;
        maze.field[rny][rnx] |= Direction.opposite(dir);

        markIn(maze, frontier, fx, fy);
    }

    return maze;
}

/**
 *
 * @param ms milli-seconds to sleep for
 * @returns {Promise<unknown>} a promise that will sleep for at least ms amount of time
 */
function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

/**
 * starts prims algorithm but instead of performing the entire algorithm, this function paused to  re-draw each
 * step of the algorithm to the terminal, creating a kind of animated effect. In addition, the current frontier
 * cells are drawn in red
 * @param width - width of the maze to generate
 * @param height - height of the maze to generate
 * @param pauseMs - the amount of time in milliseconds to pause each iteration of the algorithm
 * @returns {Promise<Maze>} the Maze that was generated
 */
async function animatedPrims(width, height, pauseMs = 50) {
    // clear the terminal
    console.log(ansi.erase.display(2));

    let maze = new Maze(width, height);
    let frontier = []; // [ { x: y: } ]

    // mark a random cell in the maze to begin at
    const xs = getRndInteger(0, width);
    const ys = getRndInteger(0, height);
    markIn(maze, frontier, xs, ys);

    while (frontier.length !== 0) {

        // choose a random element in the frontier and remove it
        let ridx = getRndInteger(0, frontier.length);
        let {x: fx, y: fy} = frontier.splice(ridx, 1)[0];

        // choose a random neighbor to the current frontier cell
        const rand_neighbors = maze.neighbors(fx, fy).filter(xy => (maze.field[xy.y][xy.x] & IN) !== 0);
        let {x: rnx, y: rny} = rand_neighbors[ getRndInteger(0, rand_neighbors.length) ];

        // carve a path from the current frontier cell to the random neighbor cell
        const dir = Maze.removeWall(fx, fy, rnx, rny);
        maze.field[fy][fx] |= dir;
        maze.field[rny][rnx] |= Direction.opposite(dir);

        markIn(maze, frontier, fx, fy);
        display_step(maze);
        await sleep(pauseMs);
    }

    return maze;
}

width = process.argv.length > 2 ? parseInt(process.argv[2], 10) : 20
height = process.argv.length > 3 ? parseInt(process.argv[3], 10) : 20
pauseMs = process.argv.length > 4 ? parseInt(process.argv[4], 10) : 10
animatedPrims(width, height);

