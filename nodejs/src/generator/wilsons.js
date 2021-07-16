import {randomInt, sample} from "../random.js";
import Grid from "../Grid.js";

/**
 * generates a random maze using Wilson's algorithm.
 *
 * Like Aldous-Broder, this algorithm depends on the idea of a random walk, but with a twist.
 * It performs what is called a loop-erased random walk, which means that as it goes, if the path
 * it is forming happens to intersect with itself and form a loop, it erases that loop before
 * continuing on.
 *
 * 1. choose a point on the grid and mark it visited.
 * 2. choose any unvisited cell in the grid and perform a loop-erased random walk until you
 *    reach a visited cell.
 * 3. link all the cells in the current random walk to the visited cell
 * 4. repeat step 2 until all cells in the grid have been visited
 *
 * @param height the height (number of rows) to generate
 * @param width the width (number of columns) to generate
 * @returns {Grid} a grid containing the randomly generated maze
 */
function generate(height, width) {
  const grid = new Grid(height, width);

  // initialize unvisitedCells to initially contain all cells in the grid
  const unvisitedCells = [];
  for (const cell of grid) {
    unvisitedCells.push(cell);
  }

  // choose a random cell in unvisited to become "visited" by removing it from the array
  const randIndex = randomInt(unvisitedCells.length);
  const rc = unvisitedCells.splice(randIndex, 1);

  while (unvisitedCells.length > 0) {

    // choose a random unvisited cell to begin the random-walk at
    let currentCell = sample(unvisitedCells);
    let path = [currentCell];

    // while the current cell is part of unvisited cells
    while (unvisitedCells.includes(currentCell)) {
      // choose a random neighbor
      currentCell = sample(currentCell.neighbors());

      const index = path.indexOf(currentCell);
      if (index >= 0) {
        // if we've already visited the cell, (creating a loop), remove the last cell from the path
        path.splice(index + 1, path.length);
      } else {
        // add cell to the path as it has not been visited
        path.push(currentCell);
      }
    }

    // link together all cells in the path
    for (let i = 0; i < path.length - 1; i++ ) {
      path[i].link(path[i + 1]);
      // remove the linked cell from unvisited, marking it visited
      const delIdx = unvisitedCells.indexOf(path[i]);
      unvisitedCells.splice(delIdx, 1);
    }
  }

  return grid;
}

export {generate};