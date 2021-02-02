use mazes::generator::sidewinder;
use std::env;
use mazes::solver::dijkstras::{find_shortest_path, display_path};
use mazes::position::Pos;

fn main() {

    // get width and height from STDIN else default them to 10 x 10
    let args: Vec<String> = env::args().collect();
    let (width, height) = match args.len() {
        2 => (args[1].parse::<usize>().unwrap(), 10),
        3 => (
            args[1].parse::<usize>().unwrap(),
            args[2].parse::<usize>().unwrap(),
        ),
        _ => (10, 10),
    };

    // generate the maze
    println!("sidewinder {}x{}", &width, &height);
    let maze = sidewinder::generate(height, width);
    println!("{}", &maze);

    // find shortest path from northwest corner to southwest corner
    println!("sidewinder {}x{} shortest path", &width, &height);
    let shortest_path = find_shortest_path(&maze, Pos::new(0, 0), Pos::new(height-1, 0));
    println!("{}\n\n\n", display_path(&maze, &shortest_path));
}