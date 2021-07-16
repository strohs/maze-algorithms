import Cell from "./Cell.js";
import {randomInt, shuffle} from "./random.js";

export default class Grid {

  /**
   * constructs a Grid containing the specified amount of rows and columns
   * @param rows - the number of rows in this grid
   * @param cols - the number of columns in this grid
   */
  constructor (rows, cols) {
    this._rows = rows;
    this._cols = cols;
    this._grid = this._buildGridCells(rows, cols);
  }

  /**
   * builds the cells of a grid, ensuring that each cell's neighbors are set correctly
   * @param rows the number of rows to build
   * @param cols the number of columns to build
   * @returns {Cell[]} a two-dimensional array of Cells
   * @private
   */
  _buildGridCells(rows, cols) {
    let grid = new Array(rows);

    // build cells without neighbors set
    for (let r = 0; r < rows; r++) {
      let row = new Array(cols);
      for (let c = 0; c < cols; c++) {
        row[c] = new Cell(r, c);
      }
      grid[r] = row;
    }
    // now set the neighbors for each cell
    for (let r = 0; r < rows; r++) {
      for (let c = 0; c < cols; c++) {
        // has north neighbor
        if (r > 0) {
          grid[r][c].north = grid[r-1][c];
        }
        // has south neighbor
        if (r < rows - 1) {
          grid[r][c].south = grid[r + 1][c];
        }
        // has east neighbor
        if (c < cols - 1) {
          grid[r][c].east = grid[r][c + 1];
        }
        // has west neighbor
        if (c > 0) {
          grid[r][c].west = grid[r][c - 1];
        }
      }
    }
    return grid;
  }

  /**
   * returns the number of rows in this grid
   * @returns {number}
   */
  get rows() {
    return this._rows;
  }

  /**
   * the number of columns in this grid
   * @returns {number}
   */
  get cols() {
    return this._cols;
  }

  /**
   * returns the dimensions of this grid. i.e. rows * cols
   * @returns {number}
   */
  size() {
    return this._rows * this._cols;
  }

  /**
   * returns, (but does not remove) the Cell at the specified row,column index
   * @param rowIndex
   * @param colIndex
   * @returns {Cell}
   */
  getCell(rowIndex, colIndex) {
    return this._grid[rowIndex][colIndex];
  }

  /**
   * returns (but does not remove) a random cell in this Grid
   * @returns {Cell}
   */
  randomCell() {
    const rowIdx = randomInt(this._rows);
    const colIdx = randomInt(this._cols);
    return this._grid[rowIdx][colIdx];
  }

  /**
   * returns an array containing all dead end cells in the grid. Dead-ends are cells that only
   * have one link into/out of them
   * @returns {FlatArray<Cell[], 1>[]}
   */
  deadEnds() {
    return this._grid.flat().filter(cell => cell.links().length === 1);
  }

  braid(p) {
    const deadEnds = this.deadEnds();
    shuffle(deadEnds);

    for(const cell of deadEnds) {
      if (cell.links().length === 1 && Math.random() <= p) {
        // get neighbors that are not linked to the current position
        const neighbors = cell.neighbors().filter(neighbor => !cell.isLinked(neighbor));

        // try to find neighbors that are also dead-ends, if possible
        let best = neighbors.filter(neighbor => neighbor.links().length === 1);

        // if no best neighbors were found, just use neighbors
        if (best.length === 0) {
          best = neighbors;
        }

        // choose a random neighbor and link to it, which will create a loop in the maze
        const randNeighbor = best[randomInt(best.length)];
        cell.link(randNeighbor);
      }
    }
  }

  /**
   * pretty prints this grid to a String
   * @returns {string}
   */
  toString() {
    // build the top most border
    let out = "+" + "----+".repeat(this.cols) + "\n";

    for(const row of this.rowIterator()) {
      let top = "|";
      let bottom = "+";

      // iterate each cell in the row and only check if east wall or south wall should be printed
      for(const cell of row) {
        if (cell.east && cell.isLinked(cell.east)) {
          top += "     ";
        } else {
          top += "    |";
        }

        if (cell.south && cell.isLinked(cell.south)) {
          bottom += "    +";
        } else {
          bottom += "----+";
        }
      }
      out += top + "\n";
      out += bottom + "\n";
    }
    return out;
  }

  /**
   * pretty prints this Grid, but also prints the distance value for each cell of this grid
   * @param distances {Distances} a distances object for cells in this grid.
   * @returns {string} a string representation of this grid, with the distance information from Distances printed
   * into the cells.
   */
  printDistances(distances) {
    // build the top most border
    let out = "+" + "----+".repeat(this.cols) + "\n";

    for(const row of this.rowIterator()) {
      let top = "|";
      let bottom = "+";

      // iterate each cell in the row and only check if east wall or south wall should be printed
      for(const cell of row) {
        // if the current cell is contained in the Distance object, print its distance info in the cell
        // as a hexa-decimal number, else print 2 spaces
        let body = distances.contains(cell) ? distances.distanceOf(cell).toString(16).padStart(2, "0") : "  ";
        if (cell.east && cell.isLinked(cell.east)) {
          top += " " + body + "  ";
        } else {
          top += " " + body + " |";
        }

        if (cell.south && cell.isLinked(cell.south)) {
          bottom += "    +";
        } else {
          bottom += "----+";
        }
      }
      out += top + "\n";
      out += bottom + "\n";
    }
    return out;
  }

  /**
   * returns an iterator over the rows of this grid
   * @returns {IterableIterator<Cell>}
   */
  rowIterator() {
    return this._grid.values();
  }

  // iterates over grid cells in row order
  [Symbol.iterator]() {
    let index = 0;
    const grid = this._grid;
    const cols = this._cols;
    const maxSize = this.size();

    return {
      next() {
        let rowIndex = Math.floor(index / cols);
        let colIndex = index % cols;
        if (index < maxSize) {
          index++;
          return {
            value: grid[rowIndex][colIndex],
            done: false,
          }
        } else {
          return {
            value: undefined,
            done: true,
          }
        }
      }
    }
  }
}
