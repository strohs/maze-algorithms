package com.mazes.generator;

import com.mazes.Cell;
import com.mazes.Grid;

import java.util.List;
import java.util.Random;

/**
 * Aldous-Broder maze generation. Aldous-Broder generates perfect mazes, (mazes without loops), using "random-walks".
 * This avoids creating mazes with biases (like Binary Tree) and instead produces mazes with lots of winding passages.
 *
 * The idea behind it is as follows:
 * 1. Start anywhere in the grid you want, and choose a random neighbor.
 * 2. Move to that neighbor, and if it has not previously been visited, link it to the prior cell.
 * 3. Repeat until every cell has been visited.
 */
public class AldousBroder {

    Random random = new Random();

    public Grid generate(int height, int width) {
        Grid grid = new Grid(height, width);

        Cell current = grid.randomCell();
        int unvisitedCount = grid.size() - 1;

        while (unvisitedCount > 0) {
            // pick a random neighbor of the current cell
            List<Cell> neighbors = current.neighbors();
            Cell neighbor = neighbors.get(random.nextInt(neighbors.size()));

            if (neighbor.links().isEmpty()) {
                // link to the neighbor
                current.link(neighbor, true);
                unvisitedCount--;
            }
            // neighbor becomes the current cell
            current = neighbor;
        }

        return grid;
    }
}
