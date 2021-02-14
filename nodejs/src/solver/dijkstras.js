
const Distances = require("../Distances.js");

/**
 * finds the shortest path in a maze using Dijkstra's algorithm,  beginning at `startCell`
 * and finishing at `goalCell`.
 *
 * @param startCell {Cell} the starting cell of a maze to begin at
 * @param goalCell {Cell} the goal (or end) cell that you want to reach
 * @returns {Distances} a `Distances` object that contains the cells on the shortest path
 */
function shortestPathToGoal(startCell, goalCell) {
    // generate distances from startCell to goalCell
    const mazeDistances = startCell.distances();

    // starting at goalCell, work backwards towards startCell.
    // curPath will hold all cells on the shortest path
    let curCell = goalCell;
    const curPath = new Distances(curCell);

    // put curCell on the curPath, this is where the path 'starts'
    curPath.put(curCell, mazeDistances.distanceOf(curCell));

    // keep going until we reach the startCell
    while (curCell !== startCell) {

        // get all cells linked to curCell
        for (let linkedCell of curCell.links()) {

            // if the linkedCell's distance is less than the current cell's distance, insert
            // the linkedCell into curPath, and make that linkedCell the current cell
            if (mazeDistances.distanceOf(linkedCell) < mazeDistances.distanceOf(curCell)) {
                curPath.put(linkedCell, mazeDistances.distanceOf(linkedCell));
                curCell = linkedCell;
                break;
            }
        }
    }

    return curPath;
}

exports.shortestPathToGoal = shortestPathToGoal;