package com.mazes;

/**
 * Direction encodes a North, South, East, West direction as an integer value. These values are stored in the cells
 * of a maze to indicate which walls have been removed ("carved") away
 */
public enum Direction {
    N(1),
    S(2),
    E(4),
    W(8);

    private final int val;

    Direction(int val) {
        this.val = val;
    }

    int getVal() { return val; }

    static int dx(Direction dir) {
        return switch (dir) {
            case E: yield 1;
            case W: yield -1;
            default: yield 0;
        };
    }

    static int dy(Direction dir) {
        return switch (dir) {
            case N: yield -1;
            case S: yield 1;
            default: yield 0;
        };
    }

    /**
     *
     * @param dir
     * @return the opposite direction of dir
     */
    static Direction opposite(Direction dir) {
        return switch (dir) {
            case N: yield S;
            case S: yield N;
            case E: yield W;
            case W: yield E;
        };
    }
}