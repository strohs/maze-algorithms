package com.mazes;

import com.mazes.generator.*;
import com.mazes.solver.Dijkstras;

public class Examples {


    public static void main(String[] args) {
        int height = 10;
        int width = 15;

        // try to parse the height and width from STDIN, if provided
        if (args.length > 1) {
            height = Integer.parseInt(args[1]);
        }

        if (args.length > 2) {
            width = Integer.parseInt(args[2]);
        }

        /// Maze Generation Algorithms
        Grid abMaze = new AldousBroder().generate(height, width);
        System.out.println("Aldous Broder " + height + "x" + width);
        System.out.println(abMaze + "\n\n");

        Grid btMaze = new BinaryTree().generate(height, width);
        System.out.println("Binary Tree " + height + "x" + width);
        System.out.println(btMaze + "\n\n");

        Grid hkMaze = new HuntKill().generate(height, width);
        System.out.println("Hunt and Kill " + height + "x" + width);
        System.out.println(hkMaze + "\n\n");

        Grid pMaze = new Prims().generate(height, width);
        System.out.println("Prims " + height + "x" + width);
        System.out.println(pMaze + "\n\n");

        Grid rbMaze = new RecursiveBacktracker().generate(height, width);
        System.out.println("Recursive Backtracker " + height + "x" + width);
        System.out.println(rbMaze + "\n\n");

        Grid swMaze = new Sidewinder().generate(height, width);
        System.out.println("Sidewinder " + height + "x" + width);
        System.out.println(swMaze + "\n\n");

        Grid wMaze = new Wilsons().generate(height, width);
        System.out.println("Wilsons " + height + "x" + width);
        System.out.println(wMaze + "\n\n");

        // remove 50% of dead-ends, which will create loops in the maze, a.k.a a "braided" maze
        wMaze.braid(0.5f);
        System.out.println("Wilsons Braided (50% dead-ends removed) " + height + "x" + width);
        System.out.println(wMaze + "\n\n");

        // find the shortest path from north-west corner to south-east corner
        Cell nwCell = wMaze.get(0, 0);
        Cell seCell = wMaze.get(wMaze.rows - 1, wMaze.cols - 1);
        Distances shortestPath = Dijkstras.pathToGoal(nwCell, seCell);
        System.out.println("Shortest path from north-west corner to south-east corner");
        System.out.println(wMaze.printDistances(shortestPath) + "\n\n");
    }
}
