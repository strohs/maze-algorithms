const {sample} = require("../random.js");
const Grid = require("../Grid.js");

/**
 * generates a random maze using recursive backtracker algorithm.
 *
 * Here’s the mile-high view of recursive backtracker:
 * 1. Choose a random starting point in the grid.
 * 2. Randomly choose a random neighbor that has not been visited and link to it. This neighbor
 *    becomes the new current cell.
 * 3. If all neighbor cells have been visited, back up to the last cell that has unvisited
 *    neighbors and repeat.
 * 4. The algorithm ends when the process has backed all the way up to the starting point.
 *
 * In essence, this carves passages using a depth-first search, that backtracks once it carves itself
 * into a corner. Also, like hunt-and-kill, recursive-backtracker also produces mazes that are full
 * of long and meandering passages.
 *
 * @param height the number of rows in the maze
 * @param width the number of columns in the maze
 * @returns Grid containing the cells of the maze
 */
function generate(height, width) {

  const grid = new Grid(height, width);
  // stack holds the cells to be visited
  const stack =[grid.randomCell()];

  while (stack.length > 0) {

    // get the top element of the stack, but don't remove it... yet
    const currentCell = stack[stack.length - 1];

    // get all linked neighbors of current cell that are not linked to other cells
    const neighbors = currentCell.neighbors().filter(cell => cell.links().length === 0);

    if (neighbors.length === 0) {
      stack.pop();
    } else {
      // choose a random neighbor, link to it, and make it the next current by pushing it on the stack
      const neighbor = sample(neighbors);
      currentCell.link(neighbor);
      stack.push(neighbor);
    }
  }

  return grid;
}

exports.generate = generate;