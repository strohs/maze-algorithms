import {sample} from "../random.js";
import Grid from "../Grid.js";

/**
 * generates a random maze using Hunt and Kill algorithm.
 *
 * Hunt-and-kill is similar to Aldous-Broder but slightly different. It generates perfect mazes,
 * but Aldous-Broder allows you to step into any cell (even previously visited ones), while hunt-and-kill requires
 * you to walk on only unvisited cells. If you walk into a corner, you begin the "hunt" mode, which is where
 * you start from the top of the maze, look for the first cell that is a neighbor of the cells in
 * the current walk you are performing, and then link into the walk. Then repeat the random walk
 * until all cells are visited.
 *
 * Hunt-and-Kill is known to produce mazes with longer winding and meandering corridors than
 * other algorithms. That is to say, hunt-and-kill produces mazes with fewer dead ends.
 *
 * @param height number of rows to generate
 * @param width number of columns to generate
 * @returns a Grid representing the random maze that was generated
 */
function generate(height, width) {
  const grid = new Grid(height, width);

  let currentCell = grid.randomCell();

  while (currentCell) {
    // get all linked neighbors of current cell that are not linked to other cells
    const unvisitedNeighbors = currentCell.neighbors().filter(nbr => nbr.links().length  === 0);

    if (unvisitedNeighbors.length > 0) {

      // this is the random walk portion. If the current cell has unvisited neighbors, we will link
      // the current cell to a random unvisited neighbor, and then make that random neighbor the current cell
      const neighborCell = sample(unvisitedNeighbors);
      currentCell.link(neighborCell);
      currentCell = neighborCell;

    } else {

      currentCell = null;

      // begin the hunt phase. Starting from the top row of the grid, begin looking
      // for the first cell that is unvisited AND that has visited neighbors
      for (const cell of grid) {
        const visitedNeighbors = cell.neighbors().filter(c => c.links().length > 0);

        // if a cell is unvisited BUT one of its neighbors is, then link cell to a
        // random neighbor and set current cell to the cell being visited
        if (cell.links().length === 0 && visitedNeighbors.length > 0) {
          currentCell = cell;
          const randNeighbor = sample(visitedNeighbors);
          currentCell.link(randNeighbor);
          break;
        }
      }
    }
  }

  return grid;
}

export {generate};