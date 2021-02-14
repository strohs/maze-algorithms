const binaryTree = require("./src/generator/binaryTree.js");
const sidewinder = require("./src/generator/sidewinder.js");
const huntKill = require("./src/generator/huntKill.js");
const recursiveBacktracker = require("./src/generator/recursiveBacktracker.js");
const aldousBroder = require("./src/generator/aldousBroder.js");
const wilsons = require("./src/generator/wilsons.js");
const prims = require("./src/generator/prims.js");
const {shortestPathToGoal} = require("./src/solver/dijkstras.js");

const height = 10;
const width = 15;

// generate a maze using binary tree algorithm
console.log(`binary-tree ${height}x${width}`);
const bt_maze = binaryTree.generate(height, width);
console.log(bt_maze.toString(),"\n\n");

// generate a maze using sidewinder algorithm
console.log(`sidewinder ${height}x${width}`);
const sw_maze = sidewinder.generate(height, width);
console.log(sw_maze.toString(), "\n\n");

// generate a maze using hunt and kill algorithm
console.log(`hunt and kill ${height}x${width}`);
const hk_maze = huntKill.generate(height, width);
console.log(hk_maze.toString(), "\n\n");

// generate a maze using recursive-backtracker
console.log(`recursive backtracker ${height}x${width}`);
const rb_maze = recursiveBacktracker.generate(height, width);
console.log(rb_maze.toString(), "\n\n");

// generate a maze using aldous-broder
console.log(`aldous-broder ${height}x${width}`);
const ab_maze = aldousBroder.generate(height, width);
console.log(ab_maze.toString(), "\n\n");

// generate a maze using wilsons
console.log(`wilsons ${height}x${width}`);
const w_maze = wilsons.generate(height, width);
console.log(w_maze.toString(), "\n\n");

// generate a maze using prims
console.log(`prims ${height}x${width}`);
const p_maze = prims.generate(height, width);
console.log(p_maze.toString(), "\n\n");


// generate a braided maze from the prim's maze
p_maze.braid(0.5);
console.log(`prims 50% braided ${height}x${width}`);
console.log(p_maze.toString(), "\n\n");


// find the shortest path between two cell in a maze
// For this example, we will use the prim's maze and find the shortest
// path from the north-west corner to the south-east corner
const startCell = p_maze.getCell(0, 0);
const goalCell = p_maze.getCell(p_maze.rows - 1, p_maze.cols - 1);
const shortestPath = shortestPathToGoal(startCell, goalCell);
// print the maze with the shortest path info included
console.log(`prim's ${height}x${width} shortest path`);
console.log(p_maze.printDistances(shortestPath));