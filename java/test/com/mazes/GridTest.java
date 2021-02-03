package com.mazes;

import org.junit.jupiter.api.Test;

import java.util.Iterator;

import static org.junit.jupiter.api.Assertions.*;

class GridTest {

    @Test
    void size() {
        Grid g = new Grid(4, 5);
        assertEquals(g.size(), 20);
    }

    @Test
    void row_iterator() {
    }

    @Test
    void iterator_should_iterate_over_20_elements() {
        Grid g = new Grid(4, 5);
        int iterCount = 0;
        for (Cell c: g) {
            iterCount++;
        }
        assertEquals(iterCount, 20);
    }

    @Test
    void iterator_should_iterate_in_row_order() {
        Grid g = new Grid(4, 5);
        int curRow = 0;
        int curCol = 0;
        for (Cell c: g) {
            if (curCol >= 5) {
                curRow++;
                curCol = 0;
            }
            assertEquals(c.row, curRow);
            assertEquals(c.col, curCol);
            curCol++;
        }
    }
}