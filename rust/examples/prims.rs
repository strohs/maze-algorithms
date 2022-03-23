use std::env;
use mazes::generator::prims;
use mazes::solver::dijkstras::{find_shortest_path};
use mazes::maze::grid_maze::GridMaze;


fn main() {

    // get width and height from STDIN else default them to 10 x 15
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
    println!("prims {}x{}", &height, &width);
    let maze = prims::generate(height, width);
    println!("{}", &maze);

    // find shortest path from northwest corner to southeast corner
    println!("prims {}x{} shortest path", &height, &width);
    let se_corner_idx = GridMaze::idx_1d(height - 1, width - 1, width);
    let shortest_path = find_shortest_path(&maze, maze[0], maze[se_corner_idx]);
    println!("{}\n\n\n", maze.display_path(&shortest_path));
}