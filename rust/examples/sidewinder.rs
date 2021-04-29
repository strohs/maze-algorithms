use mazes::generator::sidewinder;
use std::env;
use mazes::solver::dijkstras::{find_shortest_path};
use mazes::maze::grid_maze::GridMaze;

fn main() {

    // get width and height from STDIN else default them to 10 x 10
    let args: Vec<String> = env::args().collect();
    let (height, width) = match args.len() {
        2 => (args[1].parse::<usize>().unwrap(), 10),
        3 => (
            args[1].parse::<usize>().unwrap(),
            args[2].parse::<usize>().unwrap(),
        ),
        _ => (10, 15),
    };

    // generate the maze
    println!("sidewinder {}x{}", &height, &width);
    let maze = sidewinder::generate(height, width);
    println!("{}", &maze);

    // find shortest path from northwest corner to southwest corner
    println!("sidewinder {}x{} shortest path from NW Corner to SW Corner", &height, &width);
    let sw_corner_idx = GridMaze::idx_1d(height - 1, 0, width);
    let shortest_path = find_shortest_path(&maze, maze[0], maze[sw_corner_idx]);
    println!("{}\n\n\n", maze.display_path(&shortest_path));
}