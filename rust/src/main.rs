mod generator;
mod grid;
mod grid_cell;
mod position;

use crate::generator::{binary_tree, sidewinder};
use crate::grid::Grid;
use std::env;

/// use this to configure the maze dimensions and launch the maze generation algorithm(s)
/// To run from the command line:  `cargo run <width> <height>`
/// width and height will default to 20 if not specified
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
    println!("binary tree {} {}", &width, &height);
    let grid = binary_tree::generate(height, width);
    println!("{}", &grid);

    println!("sidewinder {} {}", &width, &height);
    let grid = sidewinder::generate(height, width);
    println!("{}", &grid);

    // // binary tree
    // let maze = binary_tree::generate(width, height);
    // print!("{}", &maze);
    // println!("binary tree {}x{}\n", width, height);
    //
    // // recursive backtracking
    // let maze = recursive_backtracking::recursive_backtracking(width, height);
    // print!("{}", &maze);
    // println!("recursive backtracking {}x{}\n", width, height);
    //
    // // recursive division
    // let maze = recursive_division::recursive_divide(width, height);
    // //print!("{}", &maze);
    // recursive_division::display_maze(&maze);
    // println!("recursive division {}x{}\n", width, height);
    //
    // // prims algorithm
    // let maze = prims::prims(width, height);
    // print!("{}", &maze);
    // println!("prim's {}x{}\n", width, height);
}
