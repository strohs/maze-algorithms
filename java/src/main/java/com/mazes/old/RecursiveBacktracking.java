package com.mazes.old;

import java.util.Arrays;
import java.util.Collections;
import java.util.List;

/**
 * Encapsulates the recursive backtracking maze generation algorithm.
 * The "grid" property holds a two-dimensional grid of integers, where each integer value encodes what wall have been
 * removed:
 *  0 = all four walls are present
 *  1 = the North wall is removed
 *  2 = the South wall is removed
 *  4 = the East wall is removed
 *  8 = the West wall is removed
 *  Adding any combination of these values together will indicate that two, up to four walls are removed
 *  i.e. a value of 12 means that the East and West walls are removed
 */
public class RecursiveBacktracking {

    // width of the maze
    int width;
    // height of the maze
    int height;
    // grid holds the cells of the maze
    int[][] field;


    public RecursiveBacktracking(int width, int height) {
        this.width = width;
        this.height = height;
        this.field = newMaze(width, height);
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

    /**
     * prints the maze to standard out
     */
    public void displayMaze() {
        System.out.println(" _".repeat(width));

        for (int h = 0; h < height; h++) {
            StringBuilder row = new StringBuilder("|");
            for (int w = 0; w < width; w++) {
                if ((field[h][w] & Direction.S.getVal()) != 0) {
                    row.append(" ");
                } else {
                    row.append("_");
                }

                if ((field[h][w] & Direction.E.getVal()) != 0) {
                    if ((((field[h][w] | field[h][w + 1]) & Direction.S.getVal()) != 0)) {
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

    /**
     * carvePassage is the meat of the recursive backtracking algorithm:
     *
     * 1. Choose a starting point in the field.
     * 2. Randomly choose a wall at that point and carve a passage through to the adjacent cell, but
     *    only if the adjacent cell has not been visited yet. This becomes the new current cell.
     * 3. If all adjacent cells have been visited, back up to the last cell that has uncarved walls and repeat.
     * 4. The algorithm ends when the process has backed all the way up to the starting point.
     *
     * cx,cy is the row/col index of the current cell being visited
     */
    void carvePassage(int cx, int cy, int[][] grid) {
        List<Direction> dirs = Arrays.asList(Direction.N, Direction.S, Direction.E, Direction.W);
        Collections.shuffle(dirs);

        for (Direction direction: dirs) {
            var nx = cx + Direction.dx(direction);
            var ny = cy + Direction.dy(direction);

            if (ny >= 0 && ny < grid.length && nx >= 0 && nx < grid[ny].length && grid[ny][nx] == 0) {
                grid[cy][cx] |= direction.getVal();
                grid[ny][nx] |= Direction.opposite(direction).getVal();

                carvePassage(nx, ny, grid);
            }
        }
    }


    public static void main(String[] args) {
        RecursiveBacktracking rb = new RecursiveBacktracking(20,20);
        rb.carvePassage(0, 0, rb.field);
        rb.displayMaze();

    }
}

