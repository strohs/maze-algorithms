import {randomInt} from "../random.js";
import Grid from "../Grid.js";

/**
 * generates a random maze using binary tree algorithm
 * Binary Tree is one of the simplest maze generation algorithms:
 * 1. start at a corner of the maze (in this case it will be the North West)
 * 2. iterate through the cells row by row
 * 3. for each cell pick a random East or South wall to remove
 * 4. repeat until all cells have been visited
 * @param height the number of rows to generate
 * @param width the number of columns to generate
 * @returns {Grid} a grid containing the maze
 */
function generate(height, width) {
  const grid = new Grid(height, width);

  for (let cell of grid) {
    const neighbors = [];

    // if a cell has a south neighbor, add it to neighbors
    if (cell.south) {
      neighbors.push(cell.south);
    }

    // if a cell has an east neighbor, add it to neighbors
    if (cell.east) {
      neighbors.push(cell.east);
    }

    // choose a random neighbor from neighbors and link to it
    if (neighbors.length > 0) {
      cell.link(neighbors[randomInt(neighbors.length)]);
    }
  }

  return grid;
}

export {generate};