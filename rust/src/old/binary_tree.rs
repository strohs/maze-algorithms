use crate::maze::Maze;
use crate::direction::Direction;
use rand::seq::SliceRandom;


//// Binary Tree algorithm is one of the simplest maze generation algorithms:
////
//// 1. start at a corner of the maze (in this case it will be the North West)
//// 2. iterate through the cells row by row
//// 3.     for each cell pick a random East or South wall to remove
//// 4.     repeat until all cells have been visited



/// returns a maze generated using the binary tree algorithm
pub fn generate(width: usize, height: usize) -> Maze {
    let east_most_cell = |x| x == width - 1;
    let south_most_cell = |y| y == height - 1;
    let es_direction = [Direction::E, Direction::S];

    let mut maze = Maze::new(width, height);

    for y in 0..maze.height {
        for x in 0..maze.width {
            match (x, y) {
                (x, y) if east_most_cell(x) && south_most_cell(y) => (),
                (x, y) if east_most_cell(x) => maze[[y,x]] |= Direction::S as u8,
                (x, y) if south_most_cell(y) => maze[[y,x]] |= Direction::E as u8,
                _ => maze[[y,x]] |= *es_direction.choose(&mut rand::thread_rng()).unwrap() as u8
            }
        }
    }

    maze
}
