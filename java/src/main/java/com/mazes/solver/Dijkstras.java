package com.mazes.solver;


import com.mazes.Cell;
import com.mazes.Distances;
import com.mazes.Grid;
import com.mazes.generator.RecusrsiveBacktracker;

public class Dijkstras {


    /**
     * finds the shortest path in a `maze` using Dijkstra's Algorithm. The path will begin at `start`
     * and finish at `goal`.
     *
     * @param start the starting Cell of a maze to begin at
     * @param goal  the starting Cell of a maze to end at
     * @return a Distances object containing Cells on the shortest path
     */
    public static Distances pathToGoal(Cell start, Cell goal) {
        // compute the distances from start Cell to all other cells in the maze.
        Distances mazeDistances = start.distances();

        // start at the goal Cell and work backwards towards the start
        Cell current = goal;
        Distances curPath = new Distances(current);

        // put the goal cell on the current path, this is where the search begins
        curPath.put(current, mazeDistances.get(current));

        // the search is done once the current cell reaches the start cell
        while (current != start) {
            for (Cell neighbor : current.links()) {
                // if the neighbor's distance is less than the current cell's distance, insert
                // the neighbor cell into curPath, and make that neighbor the current cell
                if (mazeDistances.get(neighbor) < mazeDistances.get(current)) {
                    curPath.put(neighbor, mazeDistances.get(neighbor));
                    current = neighbor;
                    break;
                }
            }
        }
        return curPath;
    }


    // example of using dijkstras to find shortest path in a maze
    public static void main(String[] args) {
        Grid maze = new RecusrsiveBacktracker().generate(10, 15);

        // set start to NorthWest corner of the maze
        Cell start = maze.get(0, 0);
        // set goal to SouthEast corner of the maze
        Cell goal = maze.get(maze.rows - 1, maze.cols - 1);
        // find the shortest path
        Distances shortestPath = Dijkstras.pathToGoal(start, goal);

        // print the grid to STDOUT and overlay the shortest path information on top of it
        System.out.println(maze.printDistances(shortestPath));
    }
}
