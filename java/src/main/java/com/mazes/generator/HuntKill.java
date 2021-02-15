package com.mazes.generator;

import com.mazes.Cell;
import com.mazes.Grid;

import java.util.List;
import java.util.Random;
import java.util.stream.Collectors;

/**
 * Hunt and Kill maze generation algorithm.
 *
 * Hunt-and-kill is similar to Aldous-Broder but slightly different. It generates perfect mazes, (mazes without loops),
 * but Aldous-Broder allows you to step into any cell (even previously visited ones), while hunt-and-kill requires
 * you to walk on only unvisited cells. If you walk into a corner, you begin the "hunt" mode, which is where
 * you start from the top of the maze, look for the first cell that is a neighbor of the cells in
 * the current walk you are performing, and then link into the walk. Then repeat the random walk
 * until all cells are visited.
 *
 * Hunt-and-Kill is known to produce mazes with longer winding and meandering corridors than
 * other algorithms. That is to say, hunt-and-kill produces mazes with fewer dead ends.
 */
public class HuntKill {

    Random random = new Random();

    /**
     * generates a random maze using Hunt and Kill algorithm
     * @param height number of rows to generate
     * @param width number of columns to generate
     * @return a Grid representing the random maze that was generated
     */
    public Grid generate(int height, int width) {
        Grid grid = new Grid(height, width);

        // the current cell being visited during a random-walk
        Cell current = grid.randomCell();

        while (current != null) {
            // get all linked neighbors of current cell that are not linked to other cells
            List<Cell> unvisitedNeighbors = current.neighbors()
                    .stream()
                    .filter(cell -> cell.links().isEmpty() )
                    .collect(Collectors.toList());

            // this is the random walk portion. If the current has unvisited neighbors, we will link
            // the current to a random, unvisited neighbor, and then make that random neighbor the current
            if (!unvisitedNeighbors.isEmpty()) {
                Cell neighbor = sample(unvisitedNeighbors);
                current.link(neighbor, true);
                current = neighbor;
            } else {
                current = null;

                // this is where the hunt phase begins. Starting from the top row of the grid, begin looking
                // for the first cell that is unvisited AND that has visited neighbors
                for (Cell cell: grid) {
                    // get a list of any visited neighbors of the current cell
                    List<Cell> visitedNeighbors = cell.neighbors().stream()
                            .filter(c -> !c.links().isEmpty())
                            .collect(Collectors.toList());

                    // if a cell is unvisited BUT one of its neighbors is, then link cell to a
                    // random neighbor and set current cell to the cell being visited
                    if (cell.links().isEmpty() && !visitedNeighbors.isEmpty()) {
                        current = cell;
                        Cell randNeighbor = sample(visitedNeighbors);
                        current.link(randNeighbor, true);
                        break;
                    }
                }
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
}
