from unittest import TestCase

from Cell import Cell


class TestCell(TestCase):
    def test_link_is_bidirectional_by_default(self):
        cell1 = Cell(1, 1)
        cell2 = Cell(2, 2)
        cell1.link(cell2)

        self.assertTrue(cell1.is_linked(cell2))
        self.assertTrue(cell2.is_linked(cell1))
        self.assertEqual(len(cell1.links), 1)
        self.assertEqual(len(cell2.links), 1)

    def test_cells_can_unlink(self):
        cell1 = Cell(1, 1)
        cell2 = Cell(2, 2)
        cell1.link(cell2)

        self.assertTrue(cell1.is_linked(cell2))
        self.assertTrue(cell2.is_linked(cell1))
        cell1.unlink(cell2)
        self.assertFalse(cell1.is_linked(cell2))
        self.assertFalse(cell2.is_linked(cell1))
        self.assertEqual(len(cell1.links), 0)
        self.assertEqual(len(cell2.links), 0)

    def test_has_north(self):
        cell = Cell(1, 1)
        north = Cell(0, 1)
        cell.north = north
        self.assertEqual(cell.north, north)

    def test_cell_north_is_none(self):
        cell = Cell(1, 1)
        self.assertEqual(cell.north, None)

    def test_neighbors_returns_4_cells(self):
        cell = Cell(1, 1)
        cell.north = Cell(0, 1)
        cell.south = Cell(2, 1)
        cell.east = Cell(1, 2)
        cell.west = Cell(1, 0)
        nbrs = cell.neighbors()
        self.assertEqual(len(nbrs), 4)

    def test_neighbors_returns_3_cells(self):
        cell = Cell(1, 1)
        cell.north = Cell(0, 1)
        cell.south = Cell(2, 1)
        cell.west = Cell(1, 0)
        nbrs = cell.neighbors()
        self.assertEqual(len(nbrs), 3)

    def test_links_returns_2_cells(self):
        cell = Cell(1, 1)
        cell2 = Cell(1, 0)
        cell3 = Cell(1, 2)
        cell.link(cell2)
        cell.link(cell3)
        print(cell.linked_cells())

    def test_distances(self):
        cell = Cell(1, 1)
        distances = cell.distances()
        print(distances)

    def test_loop(self):
        rows = 4
        cols = 5
        grid = [[(j, i) for i in range(cols)] for j in range(rows)]
        print(grid)

    def test_distances_is_2(self):
        """cell2 is 2 'units' away from cell0"""
        cell0 = Cell(1, 0)
        cell1 = Cell(1, 1)
        cell2 = Cell(1, 2)
        cell0.link(cell1)
        cell1.link(cell2)
        dist = cell0.distances()
        self.assertEqual(dist.cells[cell2], 2)
