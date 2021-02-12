/// The **recursive division** algorithm is a "wall adder". This algorithm is particularly
/// fascinating because of its fractal nature: you could theoretically continue the process
/// indefinitely at progressively finer and finer levels of detail.
///
/// It works like this:
///
/// 1. Begin with an empty field.
/// 2. Bisect the field with a wall, either horizontally or vertically. Add a single passage through the wall.
/// 3. Repeat step #2 with the areas on either side of the wall.
/// 4. Continue, recursively, until the maze reaches the desired resolution.

const Direction = {
    SOUTH: 1,
    EAST: 2,
}

const Orientation = {
    HORIZONTAL: 1,
    VERTICAL: 2,
}

/* returns a random integer between 0 (inclusive) and max(exclusive) */
function rand(max) {
    return Math.floor(Math.random() * max );
}


/* choose the next wall orientation based on width,height */
function chooseOrientation(width, height) {
    if (width < height) {
        return Orientation.HORIZONTAL;
    } else if (height < width) {
        return Orientation.VERTICAL;
    } else {
        if (rand(2) === 0) {
            return Orientation.HORIZONTAL;
        } else {
            return Orientation.VERTICAL;
        }
    }
}

/**
 * encapsulates the width, height and a 2D grid (a.k.a. field) of a Maze
 */
class Maze {
    constructor(width, height) {
        this.width = width;
        this.height = height;
        this.grid = this.newMaze(width, height);
    }

    /* returns an empty Maze with the specified width and height. All cells will be set to 0 */
    newMaze(width, height) {
        let grid = [];
        for (let h = 0; h < height; h++) {
            let row = [];
            for (let w = 0; w < width; w++) {
                row.push(0);
            }
            grid.push(row);
        }
        return grid;
    }

    /* pretty prints the maze to the console as ASCII characters */
    display() {
        console.log(" _".repeat(this.width));

        for (let y = 0; y < this.height; y++) {
            let row = "|";
            for (let x = 0; x < this.width; x++) {
                // are we at the bottom row
                const bottom = y + 1 >= this.grid.length;
                // south = true, a bottom wall should be printed
                const south = ((this.grid[y][x] & Direction.SOUTH) !== 0 || bottom);
                //
                const south2 = (x+1 < this.grid[y].length && (this.grid[y][x+1] & Direction.SOUTH) !== 0 || bottom);
                // east = true, a vertical wall should be printed
                const east = ((this.grid[y][x] & Direction.EAST) !== 0 || x+1 >= this.grid[y].length);

                if (south) {
                    row += "_";
                } else {
                    row += " ";
                }

                if (east) {
                    row += "|";
                } else {
                    if (south && south2) {
                        row += "_";
                    } else {
                        row += " ";
                    }
                }
            }
            console.log(row);
        }
    }
}

function divide(maze, x, y, width, height, orientation) {
    if (width < 2 || height < 2) {
        return;
    }

    const horizontal = orientation === Orientation.HORIZONTAL;

    // determine where a wall will be drawn first
    let wx = horizontal ? x : x + rand(width - 2);
    let wy = horizontal ? y + rand(height - 2) : y;

    // determine where to place a passage thru the wall
    const px = horizontal ? wx + rand(width) : wx;
    const py = horizontal ? wy : wy + rand(height);

    // determine direction to draw the wall
    const dx = horizontal ? 1 : 0;
    const dy = horizontal ? 0 : 1;

    // compute wall length
    const length = horizontal ? width : height;

    // what direction is perpendicular to the wall
    const dir = horizontal ? Direction.SOUTH : Direction.EAST;

    for (let i = 0; i < length; i++) {
        if (wx !== px || wy !== py) {
            maze.grid[wy][wx] |= dir;
        }
        wx += dx;
        wy += dy;
    }

    let [nx, ny] = [x, y];
    let w = horizontal ? width : wx - x + 1;
    let h = horizontal ? wy - y + 1 : height;
    divide(maze, nx, ny, w, h, chooseOrientation(w, h));

    nx = horizontal ? x : wx + 1;
    ny = horizontal ? wy + 1 : y;
    w = horizontal ? width : x + width - wx - 1;
    h = horizontal ? y + height - wy - 1 : height;
    divide(maze, nx, ny, w, h, chooseOrientation(w, h));

}

width = process.argv.length > 2 ? parseInt(process.argv[2], 10) : 20
height = process.argv.length > 3 ? parseInt(process.argv[3], 10) : 20
let maze = new Maze(width, height);
divide(maze, 0, 0, maze.width, maze.height, chooseOrientation(maze.width, maze.height));
maze.display();