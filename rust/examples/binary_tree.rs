use std::env;
use mazes::generator::binary_tree;
use mazes::solver::dijkstras::{find_shortest_path};
use mazes::position::Pos;

fn main() {

    // get width and height from STDIN else default them to 10
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
    println!("binary_tree {}x{}", &height, &width);
    let maze = binary_tree::generate(height, width);
    println!("{}", &maze);

    // find shortest path from northwest corner to southwest corner
    println!("binary_tree {}x{} shortest path", &height, &width);
    let shortest_path = find_shortest_path(&maze, Pos::new(0, 0), Pos::new(height-1, width-1));
    println!("{}\n\n\n", maze.display_path(&shortest_path));
}