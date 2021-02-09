import maze.generator.RecursiveBacktracker
import maze.generator.BinaryTree
import maze.generator.Sidewinder
import maze.generator.AldousBroder
import maze.generator.HuntKill
import maze.generator.Wilsons
import maze.generator.Prims
from maze.solver.Dijkstras import path_to_goal

# height and width of the generated mazes
height = 10
width = 15

# Maze Generation Algorithms

# generate a random maze using BinaryTree algorithm
bt_maze = maze.generator.BinaryTree.generate(height, width)
print(f"binary tree {height}x{width}")
print(f"{bt_maze}\n\n")

# generate a random maze using Sidewinder algorithm
sw_maze = maze.generator.Sidewinder.generate(height, width)
print(f"sidewinder {height}x{width}")
print(f"{sw_maze}\n\n")

# generate a random maze using Aldous-Broder algorithm
ab_maze = maze.generator.AldousBroder.generate(height, width)
print(f"Aldous-Broder {height}x{width}")
print(f"{ab_maze}\n\n")

# generate a random maze using Hunt and Kill algorithm
hk_maze = maze.generator.HuntKill.generate(height, width)
print(f"Hunt and Kill {height}x{width}")
print(f"{hk_maze}\n\n")

# generate a random maze using Prims algorithm
pr_maze = maze.generator.Prims.generate(height, width)
print(f"prims {height}x{width}")
print(f"{pr_maze}\n\n")

# generate a random maze using Wilson's algorithm
w_maze = maze.generator.Wilsons.generate(height, width)
print(f"Wilsons {height}x{width}")
print(f"{w_maze}\n\n")

# generate a braided maze, from the Wilson's maze, by removing 50% of the dead-ends
w_maze.braid(0.5)
print(f"Braided maze (from Wilson's) with 50% dead-ends removed {height}x{width}")
print(f"{w_maze}\n\n")

# generate a random maze using RecursiveBacktracking algorithm
rb_maze = maze.generator.RecursiveBacktracker.generate(height, width)
print(f"recursive backtracker {height}x{width}")
print(f"{rb_maze}\n\n")

# Finding the shortest path from a starting cell to a goal cell

start = rb_maze.get(0, 0)                               # north-west corner
goal = rb_maze.get(rb_maze.rows - 1, rb_maze.cols - 1)  # south-east corner
shortest_path = path_to_goal(start, goal)
print(f"shortest path in recursive backtracker {height}x{width}")
print(rb_maze.print_grid(shortest_path))
