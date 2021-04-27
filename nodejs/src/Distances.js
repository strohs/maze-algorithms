/**
 * Distances is a helper class that holds distance information between a "root" cell and other "linked" cells.
 * It manages an internal Map that maps cell objects to number values, where the number is that cells
 * distance from the root cell.
 */
class Distances {

    /**
     * constructs a new Distances object with rootCell as the "root"
     * @param rootCell
     */
    constructor(rootCell) {
        this._root = rootCell;
        this._cells = new Map();
        this._cells.set(this._root, 0);
        this._rootCell = rootCell;
    }

    /**
     *
     * @returns {Cell} the root cell of this Distances object
     */
    get root() {
        return this._root;
    }

    /**
     * gets the distance for specified cell
     * @param cell
     * @returns {Number | undefined} the distance value for the specified cell. If the cell is not contained in this
     * distances object, undefined is returned
     */
    distanceOf(cell) {
        return this._cells.get(cell);
    }

    /**
     * checks if this Distances object has an entry for cell
     * @param cell {Cell} the cell to search for
     * @returns {boolean} true if the specified cell has an entry in this distances object, else false
     */
    contains(cell) {
        return this._cells.has(cell);
    }

    /**
     * add the specified cell and distance to this distance object
     * @param cell
     * @param distance
     */
    put(cell, distance) {
        this._cells.set(cell, distance);
    }

    /**
     *
     * @returns {IterableIterator<any>} an iterator over the cells contained in this Distances object
     */
    cells() {
        return this._cells.keys();
    }
}

module.exports = Distances;