mod generator;
mod solver;
mod grid;
mod grid_cell;
mod position;

use crate::generator::{binary_tree, sidewinder};
use crate::grid::Grid;
use std::env;
use crate::position::Pos;
use crate::solver::distances::display_distances;
use crate::solver::dijkstras::{distances, find_shortest_path, display_shortest_path};

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
    let mut grid = binary_tree::generate(height, width);
    println!("{}", &grid);


    println!("sidewinder {}x{}", &width, &height);
    let grid = sidewinder::generate(height, width);
    println!("{}", &grid);
    // find shortest path
    let start = Pos::new(0, 0);
    let goal = Pos::new(9, 0);
    let shortest_path = find_shortest_path(&grid, start, goal);
    println!("{}", display_shortest_path(&grid, &shortest_path));
}
