package com.mazes.generator;

import com.mazes.Grid;

/**
 * Generates a braided maze from an algorithm that generates a perfect maze.
 * A braided maze, is a maze that contains loops. They are created from a perfect maze, but with some amount
 * of dead-ends removed.
 * In this example, Recursive-Backtracker will be used to generate the source maze, but any of the other algorithms
 * could be used.
 */
public class BraidedMaze {

    /**
     * generates a braided maze
     * @param height the number of rows to generate
     * @param width the number of columns to generate
     * @param p a float that indicates how many dead ends to remove in the maze when creating the braided maze.
     *          p = 1.0 would mean remove 100% of the dead-ends, while p=0.5 would remove 50%
     * @return a braided maze creating from a perfect maze using Recursive Backtracker
     */
    public Grid generate(int height, int width, float p) {
        Grid maze = new RecursiveBacktracker().generate(height, width);
        maze.braid(p);
        return maze;
    }
}
