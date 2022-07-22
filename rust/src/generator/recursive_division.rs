use rand::{Rng, thread_rng};
use crate::maze::grid_maze::GridMaze;

/// The Recursive Division algorithm is unique among the other algorithms implemented
/// here, for two reasons. First of all, it treats the maze as a fractal, a shape whose
/// component parts are all identical (or nearly so) to the whole. Second, instead
/// of carving passages like the other algorithms have done, this one begins with
/// a wide open space and adds walls until a maze is produced. Algorithms of
/// this nature are called wall adders (as opposed to passage carvers).
/// It works by dividing the grid into two subgrids, adding a wall between them
/// and a single passage linking them. The algorithm is then repeated on each
/// side, recursively, until the passages are the desired size.
pub fn generate(height: usize, width: usize) -> GridMaze {

    let mut maze = GridMaze::new(height, width);

    // initially link each node of the maze to all its neighbors
    for pos in 0..maze.len() {
        let node = maze[pos];
        println!("node:{}", &node.pos());
        for nbr in maze.neighbors(&node) {
            println!("   nbr:{:?}", &nbr.pos());
            maze.link(&node, &nbr, false);
        }
    }

    divide(&mut maze, 0, 0, height, width);

    maze
}

fn divide(maze: &mut GridMaze, row: usize, col: usize, height: usize, width: usize) {
    //println!("r:{row} c:{col} h:{height} w:{width} \n{maze}");
    if height <= 1 || width <= 1 {
        return
    }
    if height > width {
        divide_horizontally(maze, row, col, height, width);
    } else {
        divide_vertically(maze, row, col, height, width);
    }
}

fn divide_horizontally(maze: &mut GridMaze, row: usize, col: usize, height: usize, width: usize) {
    let divide_south_of = thread_rng().gen_range(0, height-1);
    let passage_at = thread_rng().gen_range(0, width);

    for x in 0..width {
        if passage_at == x {
            continue
        }

        if let Some(node) = maze.get2d(row + divide_south_of, col + x) {
            if let Some(south_node) = maze.south(&node) {
                maze.unlink(&node, &south_node);
            }
        }
    }

    divide(maze, row, col, divide_south_of + 1, width);
    divide(maze, row + divide_south_of + 1, col, height - divide_south_of - 1, width);
}

fn divide_vertically(maze: &mut GridMaze, row: usize, col: usize, height: usize, width: usize) {
    let divide_east_of = thread_rng().gen_range(0, width-1);
    let passage_at = thread_rng().gen_range(0, height);

    for y in 0..height {
        if passage_at == y {
            continue
        }

        if let Some(node) = maze.get2d(row + y, col + divide_east_of) {

            if let Some(east_node) = maze.east(&node) {
                maze.unlink(&node, &east_node);
            }
        }
    }

    divide(maze, row, col, height, divide_east_of + 1);
    divide(maze, row, col + divide_east_of + 1, height, width - divide_east_of - 1);
}