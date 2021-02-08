from unittest import TestCase

from maze.Grid import Grid


class TestGrid(TestCase):

    def tests_row_iterator(self):
        grid = Grid(4, 4)
        row_count = 0
        for row in grid.row_iterator():
            row_count += 1
        self.assertEqual(row_count, 4)

    def tests_cell_iterator(self):
        grid = Grid(4, 3)
        cell_count = 0
        for cell in grid.cell_iterator():
            cell_count += 1
        self.assertEqual(cell_count, 12)

    def test_str_(self):
        grid = Grid(4, 3)
        print(grid)
