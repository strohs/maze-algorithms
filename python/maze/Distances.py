
class Distances:
    """
    Distances is a helper class that is used to hold distance information between a "root" cell and other cells
    in a Grid.
    The distance information is used by path finding algorithms (like Dijkstra's)
    """
    def __init__(self, root: object):
        self._root = root
        self._cells = {root: 0}

    @property
    def root(self):
        return self._root

    @property
    def cells(self):
        return self._cells

