mod generator;
mod grid;
mod grid_cell;
mod position;
mod solver;

use crate::generator::{aldous_broder, binary_tree, sidewinder, wilsons};
use crate::position::Pos;
use crate::solver::dijkstras::{display_path, find_shortest_path};
use std::env;

/// use this to configure the maze dimensions and launch the maze generation algorithm(s)
/// To run from the command line:  `cargo run <width> <height>`
/// width and height will default to 10 if not specified
fn main() {
    let args: Vec<String> = env::args().collect();
    let (width, height) = match args.len() {
        2 => (args[1].parse::<usize>().unwrap(), 20),
        3 => (
            args[1].parse::<usize>().unwrap(),
            args[2].parse::<usize>().unwrap(),
        ),
        _ => (10, 10),
    };

    // generate a maze using binary tree algorithm
    println!("binary tree {}x{}", &width, &height);
    let grid = binary_tree::generate(height, width);
    println!("{}", &grid);
    // find shortest path from northwest corner to southwest corner
    println!("binary tree {}x{} shortest path", &width, &height);
    let shortest_path = find_shortest_path(&grid, Pos::new(0, 0), Pos::new(height-1, 0));
    println!("{}\n\n\n", display_path(&grid, &shortest_path));

    // sidewinder
    println!("sidewinder {}x{}", &width, &height);
    let grid = sidewinder::generate(height, width);
    println!("{}", &grid);
    // find shortest path from northwest corner to southwest corner
    println!("sidewinder {}x{} shortest path", &width, &height);
    let shortest_path = find_shortest_path(&grid, Pos::new(0, 0), Pos::new(height-1, 0));
    println!("{}\n\n\n", display_path(&grid, &shortest_path));

    // aldous-broder
    println!("aldous-broder {}x{}", &width, &height);
    let grid = aldous_broder::generate(height, width);
    println!("{}", &grid);
    // find shortest path from northwest corner to southwest corner
    println!("aldous-broder {}x{} shortest path", &width, &height);
    let shortest_path = find_shortest_path(&grid, Pos::new(0, 0), Pos::new(height-1, 0));
    println!("{}\n\n\n", display_path(&grid, &shortest_path));

    // wilsons
    println!("wilsons {}x{}", &width, &height);
    let grid = wilsons::generate(height, width);
    println!("{}", &grid);
    // find shortest path from northwest corner to southwest corner
    // println!("aldous-broder {}x{} shortest path", &width, &height);
    // let shortest_path = find_shortest_path(&grid, Pos::new(0, 0), Pos::new(height-1, 0));
    // println!("{}\n\n\n", display_path(&grid, &shortest_path));
}
