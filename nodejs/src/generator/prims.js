const {randomInt} = require("../random.js");
const Grid = require("../Grid.js");

/**
 * generates a random maze using Prims algorithm.
 *
 * Prim’s approaches maze generation from a different angle. Rather than working edgewise across the
 * entire graph, it starts at one point, and grows outward from that point.
 * The standard version of the algorithm works something like this:
 *   1. Choose an arbitrary cell from G (the grid), and add it to some (initially empty) set V (toVisit).
 *   2. select a cell (currCell) from V with the lowest weight
 *   3. get all unlinked neighbors of currCell and select the neighbor with the lowest weight (neighbor)
 *   4. if a neighbor was found:
 *   5.   link currentCell to it
 *   6.   add neighbor to V
 *   7. else (backed into a corner of the maze)
 *   8.   remove current from V
 *   9. repeat steps 2 thru 9 until there are no longer cells in V
 *
 * @param height {Number} - the number of rows to generate
 * @param width {Number} - the number of columns to generate
 * @returns {Grid} a grid containing the randomly generated maze
 */
function generate(height, width) {

    const grid = new Grid(height, width);

    // assign random weights between 0 and 100 to all cells in the grid. This is because all the cells in our grid
    // have default weight of one, which tend to generate mazes with a bias. Assigning random weights will help
    // generate a more random maze. If the grid actually has "real" weights assigned, you could skip this step.
    const tempWeights = buildRandomWeights(grid);

    // toVisit stores cells that are to be visited, sorted by cell weight in descending order.
    // Ideally, a min-heap (or priority queue) would be ideal here
    const toVisit = [grid.randomCell()];

    while (toVisit.length > 0) {
        // sort toVisit so that the lowest weighted cells are at the end. Then the cell at the end of toVisit
        // becomes the currentCell that we are visiting
        toVisit.sort((cell1, cell2) =>
            tempWeights[cell2.row][cell2.col] - tempWeights[cell1.row][cell1.col]);
        const currCell = toVisit[toVisit.length - 1];

        // get the unlinkedNeighbors of currCell
        const unlinkedNeighbors = currCell.neighbors().filter(nbr => nbr.links().length === 0);

        if (unlinkedNeighbors.length > 0) {
            // get the lowest weighted unlinked neighbor and link it to currCell
            const lowestNeighbor = unlinkedNeighbors.sort((cell1, cell2) =>
                tempWeights[cell1.row][cell1.col] - tempWeights[cell2.row][cell2.col])[0];
            currCell.link(lowestNeighbor);
            toVisit.push(lowestNeighbor);
        } else {
            // there are no unlinked neighbors, we are in a corner of the maze, so remove currCell from toVisit
            toVisit.pop();
        }
    }

    return grid;
}


// returns a two-dimensional array of random numbers between 1 and 100 that serve as temporary weights for each
// cell of the passed in grid
function buildRandomWeights(grid) {
    const randWeights = [];
    for (let row of grid.rowIterator()) {
        const cells = [];
        for (let cell of row) {
            cells.push(randomInt(101));
        }
        randWeights.push(cells);
    }
    return randWeights;
}

exports.generate = generate;