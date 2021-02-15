package com.mazes.generator;

import com.mazes.Cell;
import com.mazes.Grid;

import java.util.ArrayList;
import java.util.List;
import java.util.Random;

/**
 *
 * Binary Tree algorithm is one of the simplest maze generation algorithms:
 * 1. start at a corner of the maze (in this case it will be the North West)
 * 2. iterate through the cells row by row
 * 3. for each cell pick a random East or South wall to remove
 * 4. repeat until all cells have been visited
 */
public class BinaryTree {
    
    /**
     * generates a random maze using binary tree algorithm
     * @param height number of rows in the generated maze
     * @param width number of columns in the generated maze
     * @return a Grid containing the random maze
     */
    public Grid generate(int height, int width) {
        Random random = new Random();
        Grid grid = new Grid(height, width);

        for (Cell cell: grid) {
            List<Cell> neighbors = new ArrayList<>();


            // if a cell has a south neighbor add it to neighbors
            cell.getSouth().ifPresent(neighbors::add);

            // if a cell has an east neighbor, add it to neighbors
            cell.getEast().ifPresent(neighbors::add);

            // choose a random neighbor from neighbors and create a link to it
            if (!neighbors.isEmpty()) {
                int randIdx = random.nextInt(neighbors.size());
                cell.link(neighbors.get(randIdx), true);
            }

        }

        return grid;
    }
}
