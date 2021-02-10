package com.mazes.generator;

import com.mazes.Cell;
import com.mazes.Grid;

import java.util.Comparator;
import java.util.List;
import java.util.PriorityQueue;
import java.util.Random;
import java.util.stream.Collectors;

/**
 * Prim’s approaches maze generation from a different angle. Rather than working edgewise across the
 * entire graph, it starts at one point, and grows outward from that point.
 * The standard version of the algorithm works something like this:
 *   1. Choose an arbitrary cell from G (the grid), and add it to some (initially empty) set V.
 *   2. select a cell (current) from V with the lowest weight
 *   3. get all unlinked neighbors of current and select the neighbor with the lowest weight (neighbor)
 *   4. if a neighbor was found:
 *   5.   link current to it
 *   6.   add neighbor to V
 *   7. else (backed into a corner of the maze)
 *   8.   remove current from V
 *   9. repeat steps 2 thru 9 until there are no longer cells in V
 */
public class Prims {

    Random random = new Random();

    /**
     * generate a random maze using Prims algorithm
     * @param height
     * @param width
     * @return
     */
    public Grid generate(int height, int width) {

        Grid grid = new Grid(height, width);
        // assign random weights to all cells in the Grid
        for (Cell c: grid) {
            c.setWeight(random.nextInt(100));
        }

        // toVisit will store cells sorted by weight
        PriorityQueue<Cell> toVisit = new PriorityQueue<>(Comparator.comparingInt(Cell::getWeight));
        toVisit.add(grid.randomCell());

        while (!toVisit.isEmpty()) {
            // get the cell with the lowest weight, then get that cells unlinked neighbors
            Cell cell = toVisit.peek();
            List<Cell> neighbors = unlinkedNeighbors(cell);

            if (!neighbors.isEmpty()) {
                // get the lowest weighted neighbor from neighbors and link it to cell
                Cell neighbor = neighbors.stream()
                        .min(Comparator.comparingInt(Cell::getWeight))
                        .get();
                cell.link(neighbor, true);
                toVisit.add(neighbor);
            } else {
                // backed into a corner of the maze, so remove cell from the toVisit
                toVisit.remove(cell);
            }
        }

        return grid;
    }

    /**
     * returns a list of cell's neighbors that are NOT linked to any other cell
     */
    private List<Cell> unlinkedNeighbors(Cell cell) {
        return cell.neighbors().stream()
                .filter(c -> c.links().isEmpty())
                .collect(Collectors.toList());
    }


    // example of generating a random maze of 10 rows and 15 columns
    public static void main(String[] args) {
        Grid maze = new Prims().generate(10, 15);
        System.out.println(maze);
    }
}
