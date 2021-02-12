const Distances = require("./Distances");

/**
 * Cell is the main object used in building mazes.
 * Cell's know their row and column position in a Grid. They know if they have a neighboring cell (or not) to the
 * North, South, East, and West. Most importantly, Cells keep track of any "links" to those neighbors.
 * A link means that a passageway has been carved between this cell and a neighboring cell. Once multiple Cells are
 * linked together they will form a maze
 *
 */
class Cell {

    /**
     * constructs a new Cell at the specified row and column index. All neighbors are set to null, the cell's links
     * are empty and the Cell's weight is set to 1.
     * @param row {Integer} this Cell's row index in a Grid
     * @param col {Integer} this Cell's column index in a Grid
     */
    constructor(row, col) {
        this._row = row;
        this._col = col;
        this._links = new Map();
        this._north = null;
        this._south = null;
        this._east = null;
        this._west = null;
        this._weight = 1;
    }


    get row () {
        return this._row;
    }

    get col () {
        return this._col;
    }

    /**
     * create a link between this cell and other.
     * @param other {Cell} the other Cell you want to create a link to
     * @param bidi {boolean} bi-directional link, if true, then also create a link between other and this Cell
     */
    link(other, bidi=true) {
        this._links.set(other, true);
        if (bidi) {
            other.link(this, false);
        }
    }

    /**
     * removes a link between this cell and other. If the link did not exist to begin with, then nothing happens
     * @param other {Cell} the other Cell to unlink from
     * @param bidi {boolean} bi-directonal link, if true, then this method will also attempt to remove the link between other
     *             and this cell
     */
    unlink(other, bidi=true) {
        this._links.delete(other);
        if (bidi) {
            other.unlink(this, false);
        }
    }

    /**
     * returns an Array containing all the cells that this cell links to
     */
    links() {
        return [...this._links.keys()];
    }

    /**
     * is this Cell linked to other cell?
     * @param otherCell
     * @returns {boolean} true if this Cell is linked to otherCell
     */
    isLinked(otherCell) {
        return this._links.has(otherCell);
    }

    /**
     * @returns {*[]} an array containing the Cells that are neighbors of this Cell. Neighbors that are null,
     * are NOT returned
     */
    neighbors() {
        let neighbors = [this._north, this._south, this._east, this._west];
        return neighbors.filter(neighbor => neighbor)
    }

    /**
     * computes the distances (i.e. the cost of moving into linked cells), using this cell
     * as the root cell.
     * @returns {Distances} a Distances object, with this cell as the root, and with distances (weights) computed for
     * every other cell in the grid.
     */
    distances() {
        // weights holds the current weights for each cell in the grid
        const weights = new Distances(this);

        // pending holds the cells of the grid that need to be visited
        const pending = [this];

        while (pending.length > 0) {
            // sort by cell weight, descending order
            let current = pending.sort((cell1, cell2) => cell2.weight - cell1.weight).pop();

            // iterate through the linked neighbors of current and find the neighbor with the lowest cost
            for(const neighbor of current.links()) {
                // the total weight of moving into a neighboring cell is the total weight
                // of the current path so far, plus the weight of the neighbor
                const totalWeight = weights.distanceOf(current) + neighbor.weight;

                if (!weights.contains(neighbor) || totalWeight < weights.distanceOf(neighbor)) {
                    pending.push(neighbor);
                    weights.put(neighbor, totalWeight);
                }
            }
        }
        return weights;
    }

    get north() {
        return this._north;
    }

    /**
     * sets this Cell's north neighbor to the passed in cell
     * @param cell {Cell} a cell to set as this cell's neighbor. If this cell does not have a neighbor then set
     * this to null
     */
    set north(cell) {
        this._north = cell;
    }

    get south() {
        return this._south;
    }

    /**
     * sets this Cell's south neighbor to the passed in cell
     * @param cell {Cell} a cell to set as this cell's neighbor. If this cell does not have a neighbor then set
     * this to null
     */
    set south(cell) {
        this._south = cell;
    }

    get east() {
        return this._east;
    }

    /**
     * sets this Cell's east neighbor to the passed in cell
     * @param cell {Cell} a cell to set as this cell's neighbor. If this cell does not have a neighbor then set
     * this to null
     */
    set east(cell) {
        this._east = cell;
    }

    get west() {
        return this._west;
    }

    /**
     * sets this Cell's west neighbor to the passed in cell
     * @param cell {Cell} a cell to set as this cell's neighbor. If this cell does not have a neighbor then set
     * this to null
     */
    set west(cell) {
        this._west = cell;
    }

    get weight() {
        return this._weight;
    }

    /**
     * set this Cell's weight to value
     * @param value {number}
     */
    set weight(value) {
        this._weight = value;
    }

    toString() {
        return `Cell{ +
            "row=${this._row}
            ", col=${this._col}
            ", linkCount=${this._links.size}
            '}`;
    }

    // /**
    //  * two Cells are equal if their respective row,column values are equal
    //  * @param cell {Cell}
    //  * @returns {boolean}
    //  */
    // equals(cell) {
    //     return this._row === cell.row && this._col === cell.col;
    // }
}

module.exports = Cell;