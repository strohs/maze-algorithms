const {sample} = require("../random.js");
const Grid = require("../Grid.js");


/**
 * generates a maze using Aldoud-Broder algorithm.
 *
 * The idea behind it is as follows:
 * 1. Start anywhere in the grid you want, and choose a random neighbor.
 * 2. Move to that neighbor, and if it has not previously been visited, link it to the prior cell.
 * 3. Repeat until every cell has been visited.
 *
 * @param height the number of rows to generate
 * @param width the number of columns to generate
 * @returns {Grid} a grid containing the generated maze
 */
function generate(height, width) {

  const grid = new Grid(height, width);
  // start at a random grid cell
  let currentCell = grid.randomCell();
  let unvisitedCount = grid.size() - 1;

  while (unvisitedCount > 0) {
    // pick a random neighbor
    const neighbor = sample(currentCell.neighbors());

    // if the neighbor is not linked to any other cells, then link currentCell to that neighbor
    if (neighbor.links().length === 0) {
      currentCell.link(neighbor);
      unvisitedCount--;
    }
    // neighbor becomes the current cell
    currentCell = neighbor;
  }

  return grid;
}

exports.generate = generate;