package com.mazes;

import org.junit.jupiter.api.Test;

import java.util.Optional;

import static org.junit.jupiter.api.Assertions.*;

class CellTest {

    @Test
    void link_can_link_cells_bidirectionally() {
        Cell cell1 = new Cell(0,0);
        Cell cell2 = new Cell(0, 1);

        cell1.link(cell2, true);
        assertTrue(cell1.links.containsKey(cell2));
        assertTrue(cell2.links.containsKey(cell1));
    }

    @Test
    void equal() {
        Cell cell1 = new Cell(0,1);
        Cell cell2 = new Cell(0, 1);

        assertEquals(cell1, cell2);
    }

    @Test
    void not_equal() {
        Cell cell1 = new Cell(0,1);
        Cell cell2 = new Cell(1, 1);

        assertNotEquals(cell1, cell2);
    }


    @Test
    void should_unlink_cells_that_have_an_existing_link() {
        Cell cell1 = new Cell(0,0);
        Cell cell2 = new Cell(0, 1);

        cell1.link(cell2, true);
        assertTrue(cell1.links.containsKey(cell2));
        assertTrue(cell2.links.containsKey(cell1));
        cell1.unlink(cell2, true);
        assertFalse(cell1.links.containsKey(cell2));
        assertFalse(cell2.links.containsKey(cell1));
    }
}