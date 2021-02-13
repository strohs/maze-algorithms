const {randomInt, randomBoolean} = require("../random.js");
const Grid = require("../Grid.js");


/**
 * generates a random maze using the sidewinder algorithm.
 *
 * Sidewinder is similar to binary tree but does have some differences.
 * In a nutshell, it goes like this:
 * 1. Work through the grid row-wise, starting with the cell at 0,0. Initialize the “run” set to be empty.
 * 2. Add the current cell to the “run” set.
 * 3. For the current cell, randomly decide whether to carve east or not.
 * 4. If a passage was carved, make the new cell the current cell and repeat steps 2-4.
 * 5. If a passage was not carved, choose any one of the cells in the run set and carve a
 * passage north. Then empty the run set, set the next cell in the row to be the current
 * cell, and repeat steps 2-5.
 * 6. Continue until all rows have been processed.
 *
 * @param height number of rows in the generated maze
 * @param width number of columns in the generated maze
 * @returns a Grid containing the random maze
 */
function generate(height, width) {

  /**
   * returns true if a current run of Cells should be closed out. A run should be closed out if
   * we are at the eastern most cell of a row, OR if we are not on the northernmost row and a random
   * true value is generated
   */
  function shouldCloseOut(cell) {
    return (!cell.east || (cell.north && randomBoolean()));
  }

  const grid = new Grid(height, width);

  for (const cell of grid) {
    // holds the current "run" of cells we are collecting
    let runs = [cell];

    // if a run should be closed out, then choose a random cell from the run, and link to that cells
    // northern neighbor
    if (shouldCloseOut(cell) && runs.length > 0) {

      const randCell = runs[randomInt(runs.length)];

      if (randCell.north) {
        randCell.north.link(randCell);
      }
      // clear the runs after we link to a cell
      runs = [];

    } else if (cell.east) {
      // else just link to the east cell
      cell.east.link(cell);
    }
  }

  return grid;
}

exports.generate = generate;