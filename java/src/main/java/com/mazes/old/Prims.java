package com.mazes.old;

import java.util.ArrayList;
import java.util.List;
import java.util.Random;

/**
 * Generates a maze using Prim's algorithm
 *
 * Prim’s approaches maze generation from a different angle. Rather than working edgewise across the
 * entire graph, it starts at one point, and grows outward from that point. The standard version
 * of the algorithm works something like this:
 *
 * 1. Choose an arbitrary vertex from G (the graph), and add it to some (initially empty) set V.
 * 2. Choose a **random** edge from G, that connects a vertex in V with another vertex not in V.
 * 3. Add that edge to the minimal spanning tree, and the edge’s other vertex to V.
 * 4. Repeat steps 2 and 3 until V includes every vertex in G.
 */
public class Prims {

    // constant for cells that have already been "carved" out and are now considered IN the maze
    static final int IN = 0x10;
    // constant for cells that are not yet carved out, and considered on the frontier
    static final int FRONTIER = 0x20;

    Maze maze;

    // internal class to hold x,y coordinates of a 2D grid
    private static class Coord {
        int x;
        int y;

        public Coord(int x, int y) {
            this.x = x;
            this.y = y;
        }
    }

    static class Maze {

        int width;
        int height;
        int[][] grid;
        List<Coord> frontier;

        public Maze(int width, int height) {
            this.width = width;
            this.height = height;
            this.grid = newMaze(width, height);
            this.frontier = new ArrayList<>();
        }

        public int [][] newMaze(int width, int height) {
            var grid = new int [width][height];
            for (int y = 0; y < height; y++) {
                var row = new int [width];
                for (int x = 0; x < width; x++) {
                    row[x] = 0;
                }
                grid[y] = row;
            }
            return grid;
        }

        public void display() {
            System.out.println(" _".repeat(width));

            for (int y = 0; y < height; y++) {
                StringBuilder row = new StringBuilder("|");
                for (int x = 0; x < width; x++) {
                    if ((this.grid[y][x] & Direction.S.getVal()) != 0) {
                        row.append(" ");
                    } else {
                        row.append("_");
                    }

                    if ((this.grid[y][x] & Direction.E.getVal()) != 0) {
                        if (((this.grid[y][x] | this.grid[y][x+1]) & Direction.S.getVal()) != 0) {
                            row.append(" ");
                        } else {
                            row.append("_");
                        }
                    } else {
                        row.append("|");
                    }
                }
                System.out.println(row);
            }
        }
    }

    /**
     * adds the cell at specified x,y coordinate to the list of frontier cells
     * @param x
     * @param y
     * @return true if the cells was added to the frontier, false otherwise
     */
    boolean addFrontier(int x, int y) {
        if (x >= 0 && y >= 0 && y < this.maze.grid.length && x < this.maze.grid[y].length && this.maze.grid[y][x] == 0) {
            this.maze.grid[y][x] |= FRONTIER;
            this.maze.frontier.add(new Coord(x, y));
            return true;
        } else {
            return false;
        }
    }

    /**
     * marks the cell at the x,y coordinate as being "IN" the maze, (no longer part of the frontier)
     * @param x
     * @param y
     */
    void mark(int x, int y) {
        this.maze.grid[y][x] |= IN;
        if (x > 0) {
            addFrontier(x-1, y);
        }
        addFrontier(x+1, y);
        if (y > 0) {
            this.addFrontier(x, y-1);
        }
        this.addFrontier(x, y+1);
    }

    /**
     * @return a list of cell coordinates that are neighbors (to the North, South, East, and West) to the Cell
     * located at x,y
     */
    List<Coord> neighbors(int x, int y) {
        List<Coord> ns = new ArrayList<>();

        if (x > 0 && (this.maze.grid[y][x-1] & IN) != 0) {
            ns.add(new Coord(x - 1, y));
        }
        if (x+1 < this.maze.grid[y].length && (this.maze.grid[y][x+1] & IN) != 0) {
            ns.add(new Coord(x + 1, y));
        }
        if (y > 0 && (this.maze.grid[y-1][x] & IN) != 0) {
            ns.add(new Coord(x, y - 1));
        }
        if (y+1 < this.maze.grid.length && (this.maze.grid[y+1][x] & IN) != 0) {
            ns.add(new Coord(x, y + 1));
        }

        return ns;
    }

    /**
     * returns the Direction taken when moving from fx,fy to tx,ty
     * @param fx from x
     * @param fy from y
     * @param tx to x
     * @param ty to y
     */
    static Direction direction(int fx, int fy, int tx, int ty) {
        if (fx < tx) {
            return Direction.E;
        } else if (fx > tx) {
            return Direction.W;
        } else if (fy < ty) {
            return Direction.S;
        } else {    // fy > ty
            return Direction.N;
        }
    }

    /**
     * checks if a cell located at x,y is considered "empty"
     * @param x x position of cell to check
     * @param y y position of cell to check
     * @return true if the cell is empty, false otherwise
     */
    boolean empty(int x, int y) {
        return this.maze.grid[y][x] == 0 || this.maze.grid[y][x] == FRONTIER;
    }

    /**
     * constructs a maze to be generated using Prim's algorithm
     * @param width - the width, or number of columns in the maze
     * @param height - the height, or number of rows in the maze
     */
    public Prims(int width, int height) {
        this.maze = new Maze(width, height);
    }

    /**
     * generate a maze using Prim's algorithm
     */
    public Maze prims() {
        Random rand = new Random();
        // pick a random cell to start at
        int xs = rand.nextInt(this.maze.width);
        int ys = rand.nextInt(this.maze.height);
        mark(xs, ys);

        while (this.maze.frontier.size() != 0) {
            // choose a random element in the frontier and remove it
            int ridx = rand.nextInt(this.maze.frontier.size());
            Coord rCoord = maze.frontier.remove(ridx);
            int x = rCoord.x;
            int y = rCoord.y;

            // choose a random neighbor to the current frontier cell
            List<Coord> ns = neighbors(x,y);
            int nidx = rand.nextInt(ns.size());
            Coord rn = ns.get(nidx);

            // carve a path from the current frontier cell to the random neighbor cell
            Direction dir = Prims.direction(x, y, rn.x, rn.y);
            this.maze.grid[y][x] |= dir.getVal();
            this.maze.grid[rn.y][rn.x] |= Direction.opposite(dir).getVal();

            this.mark(x, y);
        }
        return this.maze;
    }

    public static void main(String[] args) {
        int width = 20;
        int height = 20;
        Prims p = new Prims(width, height);
        Maze m = p.prims();
        m.display();
    }
}
