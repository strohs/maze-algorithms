const ansi = require('ansi-escape-sequences');
const Direction = require('./direction');

/**
 * The Maze class encapsulates a maze as a 2D array of cells, as well as keeping track of a separate array of cells
 * called the "Frontier".
 * Maze is used by a maze generation algorithm to hold the state of the algorithm, the cells will be either im
 * the "frontier" or "In" the maze. Frontier cells are a list of cells that have not been "carved" out of the maze...
 * yet. "In" cells have been carved out, and are considered part of the maze.
 */
class Maze {

    /**
     * constructs a new Maze with the given width and height
     * @param width
     * @param height
     */
    constructor(width, height) {
        this.width = width;
        this.height = height;
        // a two dimensional array of cells. width = the number of columns, height = number of rows
        this.field = this.newMaze(width, height);
    }


    /**
     * initializes and returns two-dimensional array that will be used to hold the cells of the Maze.
     * @param width number of columns
     * @param height number of rows
     * @returns {[[Number]]} a two-dimensional array with the number of columns equal to width and the number of rows
     * equal to height. All cells will be initialized to 0
     */
    newMaze(width, height) {
        let field = [];
        for (let h = 0; h < height; h++) {
            let row = [];
            for (let w = 0; w < width; w++) {
                row.push(0);
            }
            field.push(row);
        }
        return field;
    }


    /**
     *
     * @param x the x coordinate (column) of the cell you want to check
     * @param y the y coordinate (row) of the cell you want to check
     * @returns {boolean} true if the given x,y coordinate is within the bounds of the maze
     */
    inBounds(x, y) {
        return y >= 0 && y < this.field.length && x >= 0 && x < this.field[y].length
    }


    /**
     * pretty prints the maze to the console as ASCII characters. It uses bitwise ORs and ANDs to determine which
     * walls to draw for each cell
     */
    display() {
        console.log(" _".repeat(this.width));

        for (let h = 0; h < this.height; h++) {
            let row = "|";
            for (let w = 0; w < this.width; w++) {
                (this.field[h][w] & Direction.S) !== 0 ? row += " " : row += "_";

                if ((this.field[h][w] & Direction.E) !== 0) {
                    (((this.field[h][w] | this.field[h][w + 1]) & Direction.S) !== 0) ? row += " " : row += "_";
                } else {
                    row += "|";
                }
            }
            console.log(row);
        }
    }



    /**
     * @returns {[]} array of cell indices {x;, y:} that are neighbors of the cell at position x,y
     */
    neighbors(x, y) {
        const n = [];

        if (x > 0) {
            n.push( { x: x-1, y: y } );
        }
        if (x+1 < this.field[y].length) {
            n.push( { x: x+1, y: y });
        }
        if (y > 0) {
            n.push( { x: x, y: y-1 } );
        }
        if (y+1 < this.field.length) {
            n.push( { x: x, y: y+1 } );
        }

        return n;
    }

    /**
     * determines which wall of a cell to remove or "carve out" when moving from fx,fy to tx,ty
     * @param fx from x position
     * @param fy from y position
     * @param tx to x position
     * @param ty to y position
     * @returns {number} a number (Direction) that indicates which wall was "carved" away when carving a path from
     * fx,fy  to  tx,ty
     */
    static removeWall(fx, fy, tx, ty) {
        if (fx < tx) {
            return Direction.E;
        } else if (fx > tx) {
            return Direction.W;
        } else if (fy < ty) {
            return Direction.S;
        } else {    // fy > ty
            return Direction.N;
        }
    }

    /**
     *
     * @param x
     * @param y
     * @returns {boolean} true if the given x,y coordinate is empty, or doesn't have any walls removed
     */
    isEmpty(x, y) {
        return this.field[y][x] === 0
    }

}


module.exports = Maze;

