from typing import Optional

from maze.Distances import Distances


class Cell:
    """
    Cell represents a cell in a two-dimensional Grid. Every cell knows its row and column position in the Grid,
    as well as if it has neighboring cells to the North, South, East or West. Cells also keep track of any "links"
    to those neighboring Cells. A link means that a "passage" has been "carved" between this cell and another cell,
    connecting them in a maze.  Each cell also has a weight, or cost, associated with moving into the cell.
    Two Cells are considered equal if their corresponding row,col indices are equal.
    """

    def __init__(self, row: int, col: int):
        self._row = row
        self._col = col
        self._north: Optional[Cell] = None
        self._south: Optional[Cell] = None
        self._east: Optional[Cell] = None
        self._west: Optional[Cell] = None
        self._links = {}
        self._weight = 1

    @property
    def row(self):
        return self._row

    @property
    def col(self):
        return self._col

    @property
    def north(self):
        return self._north

    @north.setter
    def north(self, value):
        self._north = value

    @property
    def south(self):
        return self._south

    @south.setter
    def south(self, value):
        self._south = value

    @property
    def east(self):
        return self._east

    @east.setter
    def east(self, value):
        self._east = value

    @property
    def west(self):
        return self._west

    @west.setter
    def west(self, value):
        self._west = value

    @property
    def weight(self):
        return self._weight

    @weight.setter
    def weight(self, value):
        self._weight = value

    @property
    def links(self):
        return self._links

    def link(self, other, bidi=True):
        """
         creates a link between this Cell and other. Use this method to "carve a passage" between cells
        :param other: the other Cell to carve a passage to
        :param bidi: bidirectional, if True, than also carve a passage between other and self
        :return:
        """
        self._links[other] = True
        if bidi:
            other.links[self] = True

    def unlink(self, other, bidi=True):
        """
        removes a link between this cell and other. If the link did not exist to begin with then Raises a KeyError
        :param other: the other Cell to unlink from
        :param bidi: bidirectional, if True, then also remove the link from other to self
        :return:
        """
        del self._links[other]
        if bidi:
            del other.links[self]

    def linked_cells(self):
        """
        :return: a list of all keys currently in this Cell's links dictionary
        """
        return list(self._links)

    def is_linked(self, other):
        """
        checks if this Cell has a link to other
        :param other: the other Cell to test
        :return: True if there is a link from this Cell to other, else False
        """
        return other in self._links

    def neighbors(self):
        """
        :return: a list containing all neighbor cells of this cell. Neighbors that do not exist (i.e. None)
        are not returned
        """
        neighbors = []
        if self._north is not None:
            neighbors.append(self._north)
        if self._south is not None:
            neighbors.append(self._south)
        if self._east is not None:
            neighbors.append(self._east)
        if self._west is not None:
            neighbors.append(self._west)
        return neighbors

    def distances(self) -> Distances:
        """
        computes the distances (i.e. the cost of moving into linked cells), using this cell as the root cell.
        :return: a Distances object, with this cell as the root, and with distances (costs) computed for every other
        cell in the grid.
        """
        weights = Distances(self)
        pending = [self]

        while pending:
            # sort pending cells by weight
            pending.sort(key=lambda cell: cell.weight)
            current = pending.pop(0)

            # iterate through the linked neighbors of current and choose the neighbor with the lowest cost
            for neighbor in current.linked_cells():
                # the total weight of moving into a neighboring cell is the total weight
                # of the current path so far, plus the weight of the neighbor
                total_weight = weights.cells[current] + neighbor.weight

                if neighbor not in weights.cells or total_weight < weights.cells[neighbor]:
                    pending.append(neighbor)
                    weights.cells[neighbor] = total_weight

        return weights

    def __str__(self):
        return "({},{}) N:{} S:{} E:{} W:{} links:{}".format(
            self._row,
            self._col,
            self._north is not None,
            self._south is not None,
            self._east is not None,
            self._west is not None,
            len(self._links)
        )

    def __eq__(self, other: object) -> bool:
        """ two cells are equal if their respective row and column values are equal"""
        if not isinstance(other, Cell):
            return NotImplemented
        return self._row == other.row and self._col == other.col

    def __ne__(self, other: object) -> bool:
        if not isinstance(other, Cell):
            return NotImplemented
        return self._row != other.row or self._col != other.col

    def __hash__(self) -> int:
        return self._row.__hash__() + self._col.__hash__()

