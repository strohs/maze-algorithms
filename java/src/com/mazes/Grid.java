package com.mazes;

import java.util.Arrays;
import java.util.Iterator;
import java.util.NoSuchElementException;
import java.util.Optional;

/**
 * Grid is a wrapper around a two-dimensional "grid" of Cells.
 */
public class Grid implements Iterable<Cell> {

    // total number of rows in this grid
    public final int rows;

    // total number of columns in this grid
    public final int cols;

    // holds the cells of this grid in a two-dimensional array
    Cell [][] grid;

    /**
     * builds a new Grid containing the specified rows and columns.
     * @param rows the number of rows the grid should contain (i.e. its height)
     * @param cols the number of columns the grid should contain (i.e. its width)
     */
    public Grid(int rows, int cols) {
        this.rows = rows;
        this.cols = cols;
        this.grid = buildCells(rows, cols);
    }


    /**
     *
     * @return the dimension of this grid, i.e. rows * cols
     */
    public int size() {
        return this.rows * this.cols;
    }


    /**
     *
     * @return a random cell in this Grid
     */
    public Cell randomCell() {
        int row = Util.rand(this.rows);
        int col = Util.rand(this.cols);
        return this.grid[row][col];
    }


    /**
     *
     * @return an iterator of Cell[], over the rows of this grid
     */
    public Iterator<Cell[]> row_iterator() {
        return Arrays.stream(this.grid).iterator();
    }



    /**
     * builds the cells of this grid and sets the neighbors for each cell
     * @param rows the number of rows in this grid
     * @param cols the number of columns in this grid
     * @return a two dimensional array of Cells, with neighbor cells created for each cell
     */
    private Cell[][] buildCells(int rows, int cols) {
        Cell [][] grid = new Cell[rows][cols];

        // build the Cells of this grid, without any links to neighbors
        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                grid[r][c] = new Cell(r, c);
            }
        }
        // now set the links to neighboring Cells for each cell
        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                setNorthNeighbor(grid, r, c);
                setSouthNeighbor(grid, r, c);
                setEastNeighbor(grid, r, c);
                setWestNeighbor(grid, r, c);
            }
        }

        return grid;
    }

    // set the north neighbor for the cell at index row,col in the grid
    private void setNorthNeighbor(Cell[][] grid, int row, int col) {
        if (row > 0) {
            grid[row][col].setNorth(Optional.of(grid[row - 1][col]));
        }
    }

    // set the south neighbor for the cell at index row,col in the grid
    private void setSouthNeighbor(Cell[][] grid, int row, int col) {
        if (row < this.rows - 1) {
            grid[row][col].setSouth(Optional.of(grid[row + 1][col]));
        }
    }

    // set the eastern neighbor for the cell at index row,col in the grid
    private void setEastNeighbor(Cell[][] grid, int row, int col) {
        if (col < this.cols - 1) {
            grid[row][col].setEast(Optional.of(grid[row][col + 1]));
        }
    }

    // set the western neighbor for the cell at index row,col in the grid
    private void setWestNeighbor(Cell[][] grid, int row, int col) {
        if (col > 0) {
            grid[row][col].setWest(Optional.of(grid[row][col - 1]));
        }
    }

    /**
     * pretty prints the Grid into a String
     * @return a pretty printed version of the Grid
     */
    @Override
    public String toString() {
        StringBuilder sb = new StringBuilder();
        // build the top wall of the grid
        sb.append("+").append("---+".repeat(this.cols)).append("\n");

        Iterator<Cell[]> rowIter = this.row_iterator();
        while (rowIter.hasNext()) {
            StringBuilder top = new StringBuilder("|");
            StringBuilder bottom = new StringBuilder("+");

            Cell[] row = rowIter.next();
            for (Cell cell: row) {
                // determine if an eastern wall should be drawn
                if (cell.east.isPresent() && cell.is_linked(cell.east.get())) {
                    top.append("    "); // no east wall drawn
                }
                else {
                    top.append("   |"); // east wall is drawn
                }

                // determine if south wall should be drawn
                if (cell.south.isPresent() && cell.is_linked(cell.south.get())) {
                    bottom.append("   +");
                } else {
                    bottom.append("---+");
                }
            }
            sb.append(top).append("\n");
            sb.append(bottom).append("\n");
        }

        return sb.toString();
    }

    @Override
    public Iterator<Cell> iterator() {
        return new GridIterator();
    }

    /**
     * An iterator implementation that iterates over the Cells of a Grid in row order
     */
    class GridIterator implements Iterator<Cell> {
        // one-dimensional index into Grid cells array
        int idx = 0;

        @Override
        public boolean hasNext() {
            return idx < Grid.this.size();
        }

        @Override
        public Cell next() {
            if (!hasNext()) {
                throw new NoSuchElementException();
            }
            // convert one-dimensional index into row,col indices
            int cur_row = idx / Grid.this.cols;
            int cur_col = idx % Grid.this.cols;
            idx++;
            return grid[cur_row][cur_col];
        }
    }

}
