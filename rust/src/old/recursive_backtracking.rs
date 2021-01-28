use crate::direction::Direction;
use crate::maze::Maze;
use rand::prelude::*;

/// Here’s the mile-high view of **recursive backtracking**:
///
/// 1. Choose a starting point in the field.
/// 2. Randomly choose a wall at that point and carve a passage through to the adjacent cell, but
///    only if the adjacent cell has not been visited yet. This becomes the new current cell.
/// 3. If all adjacent cells have been visited, back up to the last cell that has uncarved walls and repeat.
/// 4. The algorithm ends when the process has backed all the way up to the starting point.
///
/// cx,cy is the current cell being visited
fn carve_passage(cx: usize, cy: usize, maze: &mut Maze) {
    // choose a random direction
    let mut directions = [Direction::N, Direction::S, Direction::E, Direction::W];
    directions.shuffle(&mut thread_rng());

    for direction in directions.iter() {
        let nx = (cx as i8 + direction.dx()) as usize;
        let ny = (cy as i8 + direction.dy()) as usize;

        // new cell must be within the bounds of the grid and must not already be visited
        if maze.in_bounds(nx, ny) && maze[[ny, nx]] == 0 {
            // "carve" a passage from the current cell to the next cell
            maze[[cy, cx]] |= *direction as u8;
            maze[[ny, nx]] |= direction.opposite() as u8;

            carve_passage(nx, ny, maze);
        }
    }
}

pub fn recursive_backtracking(width: usize, height: usize) -> Maze {
    let mut maze = Maze::new(width, height);
    carve_passage(0, 0, &mut maze);
    maze
}
