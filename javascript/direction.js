


/**
 * a constant that encodes which walls have been removed (carved) from a cell of a maze.
 * @type {{S: number, E: number, W: number, N: number}}
 */
const Direction = {
    N: 1,
    S: 2,
    E: 4,
    W: 8,
    /**
     * @returns {number} a direction that is the opposite of the passed in direction
     */
    opposite: function(dir) {
        if (dir === Direction.N) { return Direction.S; }
        if (dir === Direction.S) { return Direction.N; }
        if (dir === Direction.W) { return Direction.E; }
        if (dir === Direction.E) { return Direction.W; }
    },
    /**
     * represents a "move" from on cell to another in the East/West direction
     * @param dir - the Direction that is being moved to
     * @returns {number} 1 if dir was a move to the east, -1 if dir was a move to West, 0 if dir was North or South
     */
    dx: function (dir) {
        if (dir === Direction.E) {
            return 1;
        } else if (dir === Direction.W) {
            return -1;
        } else {
            return 0;
        }
    },
    /**
     * represents a "move" from on cell to another in the North/South direction
     * @param dir the Direction that is being moved to
     * @returns {number} 1 if dir was a move to the South, -1 if dir was a move to North, 0 if dir was East or West
     */
    dy: function (dir) {
        if (dir === Direction.N) {
            return -1;
        } else if (dir === Direction.S) {
            return 1;
        } else {
            return 0;
        }
    }
}

module.exports = Direction;