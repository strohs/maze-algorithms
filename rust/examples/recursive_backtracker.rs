use std::env;
use mazes::generator::recursive_backtracker;
use mazes::solver::dijkstras::{find_shortest_path};
use mazes::maze::grid_maze::GridMaze;
use mazes::solver::distances::overlay_distances;

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
    println!("recursive-backtracker {}x{}", &height, &width);
    let maze = recursive_backtracker::generate(height, width);
    println!("{}", &maze);

    //find shortest path from northwest corner to southeast corner
    println!("recursive-backtracker {}x{} shortest path from NW Corner to SE Corner", &height, &width);
    let sw_corner_idx = GridMaze::idx_1d(height - 1, width - 1, width);
    let shortest_path = find_shortest_path(&maze, maze[0], maze[sw_corner_idx]);
    println!("{}\n\n\n", maze.display_path(&shortest_path));

    let distances = maze.distances(&maze[0]);
    println!("\n\n\n{}", overlay_distances(&maze, &distances));

}