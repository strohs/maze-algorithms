

class Distances:
    """
    Distances is a helper class that is used to hold distance information between a root cell and other cells
    in a Grid.
    When Distances is instantiated, a root cell must be specified to act as the origin point. Then use the put()
    method to add Cells and their distances from the root cell.
    These distances are used by path finding algorithms (like Dijkstra's)
    """

    def __init__(self, root: object):
        self.root = root
        self.cells = {root: 0}
