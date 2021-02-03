package com.mazes;

import java.util.*;

/**
 * Cell represents a cell in a two-dimensional Grid. Every cell knows its row and column position in the Grid,
 * as well as if it has neighboring cells to the North, South, East or West. Cells also keep track of any "links"
 * to those neighboring Cells. A link means that a "passage" has been "carved" between this cell and another cell,
 * connecting them in a maze.
 * Two Cells are considered equal if their corresponding row,col indices are equal.
 */
@SuppressWarnings("OptionalUsedAsFieldOrParameterType")
public class Cell {

    // this cells row index in a grid
    public final int row;

    // this cells column index in a grid
    public final int col;

    // holds any links, (carved passages) between this cell and another Cell
    HashMap<Cell, Boolean> links;

    // a link to this cells northern neighbor, if one exists
    Optional<Cell> north;

    // a link to this cells southern neighbor, if one exists
    Optional<Cell> south;

    // a link ti this cells eastern neighbor, if one exists
    Optional<Cell> east;

    // a link to this cells western neighbor, if one exists
    Optional<Cell> west;

    /**
     * constructs a new Cell with the specified row,col index. The cells neighbors will be set to
     * Optional.empty() and the Cell's links HashMap will be empty. The cells neighbors must be set individually
     * via their respective setter methods
     * @param row the row index where this cell resides in a grid
     * @param col the col index where this cell resides in a grid
     */
    public Cell(int row, int col) {
        this.row = row;
        this.col = col;
        this.north = Optional.empty();
        this.south = Optional.empty();
        this.east = Optional.empty();
        this.west = Optional.empty();
        this.links = new HashMap<>();
    }

    /**
     * @return an Optional<Cell> containing this cell's northern neighbor
     */
    public Optional<Cell> getNorth() {
        return this.north;
    }

    /**
     * @return an Optional<Cell> containing this cell's southern neighbor
     */
    public Optional<Cell> getSouth() {
        return south;
    }

    /**
     * @return an Optional<Cell> containing this cell's eastern neighbor
     */
    public Optional<Cell> getEast() {
        return east;
    }

    /**
     * @return an Optional<Cell> containing this cell's western neighbor
     */
    public Optional<Cell> getWest() {
        return west;
    }

    void setNorth(Optional<Cell> north) {
        this.north = north;
    }

    void setSouth(Optional<Cell> south) {
        this.south = south;
    }

    void setEast(Optional<Cell> east) {
        this.east = east;
    }

    void setWest(Optional<Cell> west) {
        this.west = west;
    }

    /**
     * creates a link between this Cell and other. Use this to "carve a passage" between cells
     *
     * @param other the other Cell you want to create a link to
     * @param bidi  bi-directional link, if true, then also create a link between other and this Cell
     */
    public void link(Cell other, boolean bidi) {
        this.links.put(other, true);
        if (bidi) {
            other.link(this, false);
        }
    }

    /**
     * removes a link between this cell and other. If the link did not exist to begin with, then nothing happens
     * @param other the other Cell to unlink from
     * @param bidi bi-directonal link, if true, then this method will also attempt to remove the link between other
     *             and this cell
     */
    public void unlink(Cell other, boolean bidi) {
        this.links.remove(other);
        if (bidi) {
            other.unlink(this, false);
        }
    }

    /**
     *
     * @return a Set view over the Cells that are linked to, by this Cell
     */
    public Set<Cell> links() {
        return this.links.keySet();
    }

    /**
     *
     * @param other the other Cell to test for a link
     * @return true if this Cell is links to other, else false
     */
    public boolean is_linked(Cell other) {
        return this.links.containsKey(other);
    }


    /**
     *
     * @return a List containing any neighbors of this Cell that exist
     */
    public List<Cell> neighbors() {
        List<Cell> neighbors = new ArrayList<>();
        this.north.ifPresent(neighbors::add);
        this.south.ifPresent(neighbors::add);
        this.east.ifPresent(neighbors::add);
        this.west.ifPresent(neighbors::add);

        return neighbors;
    }

    @Override
    public String toString() {
        return "Cell{" +
                "row=" + row +
                ", col=" + col +
                '}';
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (o == null || getClass() != o.getClass()) return false;
        Cell cell = (Cell) o;
        return row == cell.row && col == cell.col;
    }

    @Override
    public int hashCode() {
        return Objects.hash(row, col);
    }
}
