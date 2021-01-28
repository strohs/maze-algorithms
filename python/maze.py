from enum import Enum


class Direction(Enum):
    """A North, South, East, West direction that indicates which walls of a cell have been carved out"""
    N = 1
    S = 2
    E = 4
    W = 8

    @classmethod
    def dx(cls, d) -> int:
        """represents a "move" from one cell to another in the 'x' (East/West direction)"""
        if d == cls.E:
            return 1
        elif d == cls.W:
            return -1
        else:
            return 0

    @classmethod
    def dy(cls, d) -> int:
        """represents a "move" from one cell to another in the 'y' (North/South) direction"""
        if d == cls.N:
            return -1
        elif d == cls.S:
            return 1
        else:
            return 0

    @classmethod
    def opposite(cls, d):
        """returns the direction that is opposite to the passed in direction 'd' """
        if d == cls.N:
            return cls.S
        if d == cls.S:
            return cls.N
        if d == cls.W:
            return cls.E
        if d == cls.E:
            return cls.W


class Maze:
    """
    The Maze class encapsulates a maze as a 2D array of cells (called the field).
    A Cell is an integer value from Direction enum.  It that indicates which walls of a cell have been removed, or
    "carved out".
    For example a cell = 1 indicates the North wall has been removed. A value of 5 indicates the North and East
    wall have been removed
    """

    def __init__(self, width: int, height: int) -> None:
        self.width = width
        self.height = height
        self.field = Maze.empty_maze(width, height)

    @classmethod
    def empty_maze(cls, width, height):
        field = [[0 for x in range(width)] for y in range(height)]
        return field

    def display(self):
        """display the cells (field) of this maze to standard out using ASCII characters"""
        print(" _" * self.width)
        for y in range(self.height):
            row = "|"
            for x in range(self.width):
                row += " " if self.field[y][x] & Direction.S.value != 0 else "_"
                if self.field[y][x] & Direction.E.value != 0:
                    row += " " if (((self.field[y][x] | self.field[y][x+1]) & Direction.S.value) != 0) else "_"
                else:
                    row += "|";
            print(row)

    def in_bounds(self, x: int, y: int) -> bool:
        """returns true if the x,y index is within the bounds of the maze"""
        return len(self.field) > y >= 0 and 0 <= x < len(self.field[y])

    def neighbors(self, x, y):
        """
        returns a list of 2-tuples containing the x,y indices of cells adjacent to the cell at the given position
        x,y and that are marked as IN the maze
        """
        ns = []
        if x > 0 and self.field[y][x - 1]:
            ns.append((x - 1, y))
        if x + 1 < len(self.field[y]) and self.field[y][x+1]:
            ns.append((x + 1, y))
        if y > 0 and self.field[y - 1][x]:
            ns.append((x, y - 1))
        if y + 1 < len(self.field) and self.field[y+1][x]:
            ns.append((x, y + 1))

        return ns

    @classmethod
    def remove_wall(cls, fx, fy, tx, ty) -> Direction:
        """
        determines which wall of a cell to remove or "carve out" when moving from fx,fy to tx,ty
        :param fx: x position of cell being carved from
        :param fy: y position of cell being carved from
        :param tx: x position of cell being carved to
        :param ty: y position of cell being carved to
        :return: the Direction of the wall that should be removed (i.e. carved away)
        """
        if fx < tx:
            return Direction.E
        elif fx > tx:
            return Direction.W
        elif fy < ty:
            return Direction.S
        else:
            # fy > ty
            return Direction.N


