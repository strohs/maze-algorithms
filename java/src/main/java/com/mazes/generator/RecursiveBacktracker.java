package com.mazes.generator;

import com.mazes.Cell;
import com.mazes.Grid;

import java.util.*;
import java.util.stream.Collectors;

/**
 * Recursive Backtracker for generating perfect mazes, (mazes without loops).
 *
 * Here’s the mile-high view of recursive backtracker:
 * 1. Choose a random starting point in the grid.
 * 2. Randomly choose a random neighbor that has not been visited and link to it. This neighbor
 *    becomes the new current cell.
 * 3. If all neighbor cells have been visited, back up to the last cell that has unvisited
 *    neighbors and repeat.
 * 4. The algorithm ends when the process has backed all the way up to the starting point.
 *
 * In essence, this carves passages using a depth-first search, that backtracks once it carves itself
 * into a corner. Also, like hunt-and-kill, recursive-backtracker also produces mazes that are full
 * of long and meandering passages.
 */
public class RecursiveBacktracker {

    Random random = new Random();

    /**
     * generates a random maze using recursive backtracker algorithm
     * @param height the number of rows in the maze
     * @param width the number of columns in the maze
     * @return Grid containing the cells of the maze
     */
    public Grid generate(int height, int width) {

        Grid grid = new Grid(height, width);

        // stack holds the cells to visit
        Deque<Cell> stack = new ArrayDeque<>();
        // push a random start position onto the stack
        stack.push(grid.randomCell());

        while (!stack.isEmpty()) {

            Cell current = stack.getFirst();

            // get all linked neighbors of current cell that are not linked to other cells
            List<Cell> neighbors = current.neighbors()
                    .stream()
                    .filter(cell -> cell.links().isEmpty() )
                    .collect(Collectors.toList());

            if (neighbors.isEmpty()) {
                // we are in a corner, so backtrack by popping the current cell off of the stack
                stack.pop();
            } else {
                // choose a random neighbor, link to it, and make it the next current by pushing it on the stack
                Cell neighbor = neighbors.get(random.nextInt(neighbors.size()));
                current.link(neighbor, true);
                stack.push(neighbor);
            }
        }
        return grid;
    }
}
