use mazes::generator::recursive_backtracker;
use std::env;

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

    // going to use recursive backtracker as the starting maze. Any other algorithm that
    // generates a perfect maze would also work
    println!("recursive bactracker unbraided {}x{}", &height, &width);
    let mut maze = recursive_backtracker::generate(height, width);
    println!("{}\n\n", &maze);

    println!("dead ends = {}", maze.dead_ends().len());
    println!("recursive back-tracker BRAIDED {}x{}", &height, &width);
    // remove 80 percent of dead ends
    maze.braid(0.8);
    println!("{}", &maze);
}