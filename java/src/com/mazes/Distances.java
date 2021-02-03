package com.mazes;

import java.util.HashMap;
import java.util.Set;

/**
 * Distances is a helper class that is used to hold distance information between a root cell and other cells
 * in a Grid.
 * When Distances is instantiated, a root cell must be specified to act as the origin point. Then use the put()
 * method to add Cells and their distances from the root cell.
 * These distances are used by path finding algorithms (like Dijkstra's)
 */
public class Distances {

    // the root cell, or origin, that distances will be computed from
    private final Cell root;

    // holds distances values for each Cell in a Grid
    private final HashMap<Cell, Integer> cells;

    public Distances(Cell root) {
        this.root = root;
        this.cells = new HashMap<>();
        this.cells.put(root, 0);
    }

    /**
     *
     * @return the root cell of this Distances object
     */
    public Cell getRoot() {
        return this.root;
    }

    /**
     * get the distance value for the specified cell from this Distance class' internal Map
     *
     * @param cell the Cell to get the distance for
     * @return an integer that indicates how far this cell is from a root cell. If the cell in not in this Distances
     * class then null is returned
     */
    public int get(Cell cell) {
        return this.cells.get(cell);
    }

    /**
     * @param cell
     * @return true if this Distance object contains an entry for the given cell, else false
     */
    public boolean contains(Cell cell) {
        return this.cells.containsKey(cell);
    }

    /**
     * puts the provided cell and distance information into this Distances class internal Map
     *
     * @param cell     the Cell to put
     * @param distance the distance the cell is from the root Cell
     */
    public void put(Cell cell, int distance) {
        this.cells.put(cell, distance);
    }

    /**
     * @return a set view of the Cells contained in this Distances internal map
     */
    public Set<Cell> cells() {
        return this.cells.keySet();
    }



}
