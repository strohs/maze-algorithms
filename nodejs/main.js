import {generate as binaryTree} from "./src/generator/binaryTree.js";
import {generate as sidewinder} from "./src/generator/sidewinder.js";
import {generate as huntKill} from "./src/generator/huntKill.js";
import {generate as recursiveBacktracker} from "./src/generator/recursiveBacktracker.js";
import {generate as aldousBroder} from "./src/generator/aldousBroder.js";
import {generate as wilsons} from "./src/generator/wilsons.js";
import {generate as prims} from "./src/generator/prims.js";
import {shortestPathToGoal} from "./src/solver/dijkstras.js";

const height = 10;
const width = 15;

// generate a maze using binary tree algorithm
// ...
console.log(`binary-tree ${width}x${height}`);
const bt_maze = binaryTree(height, width);
console.log(bt_maze.toString(),"\n\n");

// generate a maze using sidewinder algorithm
console.log(`sidewinder ${width}x${height}`);
const sw_maze = sidewinder(height, width);
console.log(sw_maze.toString(), "\n\n");

// generate a maze using hunt and kill algorithm
console.log(`hunt and kill ${width}x${height}`);
const hk_maze = huntKill(height, width);
console.log(hk_maze.toString(), "\n\n");

// generate a maze using recursive-backtracker
console.log(`recursive backtracker ${width}x${height}`);
const rb_maze = recursiveBacktracker(height, width);
console.log(rb_maze.toString(), "\n\n");

// generate a maze using aldous-broder
console.log(`aldous-broder ${width}x${height}`);
const ab_maze = aldousBroder(height, width);
console.log(ab_maze.toString(), "\n\n");

// generate a maze using wilsons
console.log(`wilsons ${width}x${height}`);
const w_maze = wilsons(height, width);
console.log(w_maze.toString(), "\n\n");

// generate a maze using prims
console.log(`prims ${width}x${height}`);
const p_maze = prims(height, width);
console.log(p_maze.toString(), "\n\n");


// generate a braided maze from the prim's maze
p_maze.braid(0.5);
console.log(`prims 50% braided ${width}x${height}`);
console.log(p_maze.toString(), "\n\n");


// find the shortest path between two cell in a maze
// For this example, we will use the prim's maze and find the shortest
// path from the north-west corner to the south-east corner
const startCell = p_maze.getCell(0, 0);
const goalCell = p_maze.getCell(p_maze.rows - 1, p_maze.cols - 1);
const shortestPath = shortestPathToGoal(startCell, goalCell);
// print the maze with the shortest path info included
console.log(`prim's ${width}x${height} shortest path`);
console.log(p_maze.printDistances(shortestPath));