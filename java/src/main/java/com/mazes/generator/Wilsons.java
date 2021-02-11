package com.mazes.generator;

import com.mazes.Cell;
import com.mazes.Grid;

import java.util.ArrayList;
import java.util.List;
import java.util.Random;

/**
 * Wilson's Algorithm for generates perfect mazes, (mazes without loops).
 *
 * Like Aldous-Broder, this algorithm depends on the idea of a random walk, but with a twist.
 * It performs what is called a loop-erased random walk, which means that as it goes, if the path
 * it is forming happens to intersect with itself and form a loop, it erases that loop before
 * continuing on.
 *
 * 1. choose a point on the grid and mark it visited.
 * 2. choose any unvisited cell in the grid and perform a loop-erased random walk until you
 *    reach a visited cell.
 * 3. link all the cells in the current random walk to the visited cell
 * 4. repeat step 2 until all cells in the grid have been visited
 */
public class Wilsons {

    Random random = new Random();

    /**
     * generates a random maze using Wilson's algorithm
     * @param height the height (number of rows) to generate
     * @param width the width (number of columns) to generate
     * @return Grid containing the randomly generated maze
     */
    public Grid generate(int height, int width) {
        Grid grid = new Grid(height, width);

        // initialize the unvisited list to contain all cells in the grid
        List<Cell> unvisited = new ArrayList<>();
        for (Cell cell: grid) {
            unvisited.add(cell);
        }

        // choose a random cell in the unvisited list to become "visited"
        unvisited.remove(random.nextInt(unvisited.size()));

        while (!unvisited.isEmpty()) {
            // choose a random unvisited cell to begin the random-walk at
            Cell cell = unvisited.get(random.nextInt(unvisited.size()));
            // add the cell to the random-walk "path"
            List<Cell> path = new ArrayList<>();
            path.add(cell);

            while (unvisited.contains(cell)) {
                cell = sample(cell.neighbors());
                int index = path.indexOf(cell);
                if (index >= 0) {
                    path = path.subList(0, index+1);
                } else {
                    path.add(cell);
                }
            }

            // link together all cells in the path
            for ( int i = 0; i < path.size() - 1; i++ ) {
                path.get(i).link(path.get(i + 1), true);
                // remove the linked cell, marking it visited
                unvisited.remove(path.get(i));
            }
        }

        return grid;
    }

    /**
     * helper function that returns (but does not remove) a random Cell from a list of Cells
     * @param cells a list of cells to sample an element from
     * @return a Cell from the given cells list
     */
    Cell sample(List<Cell> cells) {
        int size = cells.size();
        return cells.get(random.nextInt(size));
    }


    // example of generating a maze of 10 rows and 15 columns
    public static void main (String[] args) {
        Grid maze = new Wilsons().generate(10, 15);
        System.out.println(maze);
    }
}
