package com.mazes.generator;

import com.mazes.Grid;
import com.mazes.Cell;

import java.util.ArrayList;
import java.util.List;
import java.util.Random;

/**
 * Sidewinder is similar to binary tree but does have some differences.
 *
 * In a nutshell, it goes like this:
 *
 * 1. Work through the grid row-wise, starting with the cell at 0,0. Initialize the “run” set to be empty.
 * 2. Add the current cell to the “run” set.
 * 3. For the current cell, randomly decide whether to carve east or not.
 * 4. If a passage was carved, make the new cell the current cell and repeat steps 2-4.
 * 5. If a passage was not carved, choose any one of the cells in the run set and carve a
 *    passage north. Then empty the run set, set the next cell in the row to be the current
 *    cell, and repeat steps 2-5.
 * 6. Continue until all rows have been processed.
 */
public class Sidewinder {

    // random number generator
    Random rd = new Random();

    /**
     * generates a random maze using the sidewinder algorithm
     * @param height number of rows in the generated maze
     * @param width number of columns in the generated maze
     * @return a Grid containing the random maze
     */
    public Grid generate(int height, int width) {
        Random rd = new Random();
        Grid grid = new Grid(height, width);

        for (Cell cell: grid) {
            List<Cell> runs = new ArrayList<>();
            runs.add(cell);

            // if a run should be closed out, then choose a random cell from the run, and link to that cells
            // northern neighbor
            if (shouldCloseOut(cell) && !runs.isEmpty()) {
                Cell randCell = runs.get(rd.nextInt(runs.size()));

                randCell.getNorth().ifPresent(northCell -> northCell.link(randCell, true));

                runs.clear();
            } else {
                // just carve a passage (link) from the current cell to it's eastern neighbor
                cell.getEast().ifPresent(eastCell -> eastCell.link(cell, true));
            }
        }

        return grid;
    }


    /**
     * returns true if a current run of Cells should be closed out. A run should be closed out if
     * we are at the eastern most cell of a row, OR if we are not on the northernmost row and a random
     * true value is generated
     */
    private boolean shouldCloseOut(Cell cell) {
        return (cell.getEast().isEmpty() || (cell.getNorth().isPresent() && rd.nextBoolean()));
    }


    // example of generating a random maze of 10 rows and 15 columns
    public static void main(String[] args) {
        Grid maze = new Sidewinder().generate(10, 15);
        System.out.println(maze);
    }
}
