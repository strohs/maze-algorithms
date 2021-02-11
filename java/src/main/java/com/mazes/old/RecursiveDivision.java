package com.mazes.old;

import com.mazes.Util;

/**
 * Direction encodes a South, East, direction as an integer value.
 */
enum Dir {
    S(1),
    E(2);

    private final int val;

    Dir(int val) {
        this.val = val;
    }

    int getVal() { return val; }
}

enum Orientation {
    HORIZONTAL(1),
    VERTICAL(2);

    private final int val;

    Orientation(int val) {
        this.val = val;
    }

    int getVal() { return val; }
}

/**
 * Maze encapsulates the width, height, and a two-dimensional grid of cells
 */
class Maze {

    int width;
    int height;
    int[][] grid;

    public Maze(int width, int height) {
        this.width = width;
        this.height = height;
        this.grid = newMaze(width, height);
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

    public void displayMaze() {
        System.out.println(" _".repeat(width));

        for (int y = 0; y < height; y++) {
            StringBuilder row = new StringBuilder("|");
            for (int x = 0; x < width; x++) {
                var bottom = y + 1 >= this.grid.length;
                var south = ((this.grid[y][x] & Dir.S.getVal()) != 0 || bottom);
                var south2 = (x+1 < this.grid[y].length && (this.grid[y][x+1] & Dir.S.getVal()) != 0 || bottom);
                var east = ((this.grid[y][x] & Dir.E.getVal()) != 0 || x+1 >= this.grid[y].length);

                if (south) {
                    row.append("_");
                } else {
                    row.append(" ");
                }

                if (east) {
                    row.append("|");
                } else {
                    if (south && south2) {
                        row.append("_");
                    } else {
                        row.append(" ");
                    }
                }
            }
            System.out.println(row);
        }
    }
}

/**
 * The **recursive division** algorithm is a "wall adder". This algorithm is particularly
 * fascinating because of its fractal nature: you could theoretically continue the process
 * indefinitely at progressively finer and finer levels of detail.
 *
 * It works like this:
 *
 * 1. Begin with an empty field.
 * 2. Bisect the field with a wall, either horizontally or vertically. Add a single passage through the wall.
 * 3. Repeat step #2 with the areas on either side of the wall.
 * 4. Continue, recursively, until the maze reaches the desired resolution.
 */
public class RecursiveDivision {

    static Orientation chooseOrientation(int width, int height) {
        if (width < height) {
            return Orientation.HORIZONTAL;
        } else if (height < width) {
            return Orientation.VERTICAL;
        } else {
            if ( Util.rand(2) == 0) {
                return Orientation.HORIZONTAL;
            } else {
                return Orientation.VERTICAL;
            }
        }
    }

    // this is the recursive division algorithm
    static void divide(Maze maze, int x, int y, int width, int height, Orientation orientation) {
        if (width < 2 || height < 2) {
            return;
        }

        var horizontal = orientation == Orientation.HORIZONTAL;

        // determine where a wall will be drawn first
        var wx = horizontal ? x : x + Util.rand(width - 2);
        var wy = horizontal ? y + Util.rand(height - 2) : y;

        // determine where to place a passage thru the wall
        var px = horizontal ? wx + Util.rand(width) : wx;
        var py = horizontal ? wy : wy + Util.rand(height);

        // determine direction to draw the wall
        var dx = horizontal ? 1 : 0;
        var dy = horizontal ? 0 : 1;

        // compute wall length
        var length = horizontal ? width : height;

        // what direction is perpendicular to the wall
        var dir = horizontal ? Dir.S : Dir.E;

        for (int i = 0; i < length; i++) {
            if (wx != px || wy != py) {
                maze.grid[wy][wx] |= dir.getVal();
            }
            wx += dx;
            wy += dy;
        }

        var nx = x;
        var ny = y;
        var w = horizontal ? width : wx - x + 1;
        var h = horizontal ? wy - y + 1 : height;
        divide(maze, nx, ny, w, h, chooseOrientation(w, h));

        nx = horizontal ? x : wx + 1;
        ny = horizontal ? wy + 1 : y;
        w = horizontal ? width : x + width - wx - 1;
        h = horizontal ? y + height - wy - 1 : height;
        divide(maze, nx, ny, w, h, chooseOrientation(w, h));
    }

    public static void main(String[] args) {
        var maze = new Maze(15, 15);
        RecursiveDivision.divide(maze, 0, 0, maze.width, maze.height, chooseOrientation(maze.width, maze.height));
        maze.displayMaze();
    }
}
