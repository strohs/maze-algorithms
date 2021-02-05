from typing import Optional

from Distances import Distances


class Cell:
    """
    Cell represents a cell in a two-dimensional Grid. Every cell knows its row and column position in the Grid,
    as well as if it has neighboring cells to the North, South, East or West. Cells also keep track of any "links"
    to those neighboring Cells. A link means that a "passage" has been "carved" between this cell and another cell,
    connecting them in a maze.
    Two Cells are considered equal if their corresponding row,col indices are equal.
    """

    def __init__(self, row: int, col: int):
        self.row = row
        self.col = col
        self.north: Optional[Cell] = None
        self.south: Optional[Cell] = None
        self.east: Optional[Cell] = None
        self.west: Optional[Cell] = None
        self.links = {}

    def link(self, other, bidi=True):
        """
         creates a link between this Cell and other. Use this method to "carve a passage" between cells
        :param other: the other Cell to carve a passage to
        :param bidi: bidirectional, if True, than also carve a passage between other and self
        :return:
        """
        self.links[other] = True
        if bidi:
            other.links[self] = True

    def unlink(self, other, bidi=True):
        """
        removes a link between this cell and other. If the link did not exist to begin with then Raises a KeyError
        :param other: the other Cell to unlink from
        :param bidi: bidirectional, if True, then also remove the link from other to self
        :return:
        """
        del self.links[other]
        if bidi:
            del other.links[self]

    def linked_cells(self):
        """
        :return: a list of all keys currently in this Cell's links dictionary
        """
        return list(self.links)

    def is_linked(self, other):
        """
        checks if this Cell has a link to other
        :param other: the other Cell to test
        :return: True if there is a link from this Cell to other, else False
        """
        return other in self.links

    def neighbors(self):
        """
        :return: a list containing all neighbor cells of this cell. Neighbors that do not exist (i.e. None)
        are not returned
        """
        neighbors = []
        if self.north is not None:
            neighbors.append(self.north)
        if self.south is not None:
            neighbors.append(self.south)
        if self.east is not None:
            neighbors.append(self.east)
        if self.west is not None:
            neighbors.append(self.west)
        return neighbors

    def distances(self) -> Distances:
        """
        computes the distances between this cell and every other cell that is linked to this cell
        :return: a Distances object, with this cell as the root, and with distances computed to every other linked cell
        """
        distances = Distances(self)
        frontier = [self]

        while frontier:
            new_frontier = []

            for cell in frontier:
                for linked_cell in cell.linked_cells():
                    if linked_cell not in distances.cells:
                        distances.cells[linked_cell] = distances.cells[cell] + 1
                        new_frontier.append(linked_cell)

            frontier.clear()
            frontier.extend(new_frontier)

        return distances

    def __str__(self):
        return "({},{}) N:{} S:{} E:{} W:{} links:{}".format(
            self.row,
            self.col,
            self.north is not None,
            self.south is not None,
            self.east is not None,
            self.west is not None,
            len(self.links)
        )

    def __eq__(self, other: object) -> bool:
        """ two cells are equal of their respective row and column values are equal"""
        if not isinstance(other, Cell):
            return NotImplemented
        return self.row == other.row and self.col == other.col

    def __ne__(self, other: object) -> bool:
        if not isinstance(other, Cell):
            return NotImplemented
        return self.row != other.row or self.col != other.col

    def __hash__(self) -> int:
        return self.row.__hash__() + self.col.__hash__()

