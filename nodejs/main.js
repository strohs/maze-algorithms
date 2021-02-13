const binaryTree = require("./src/generator/binaryTree.js");
const sidewinder = require("./src/generator/sidewinder.js");

// generate a 10x15 maze using binary tree algorithm
console.log(`binary-tree ${10}x${15}`)
const bt_maze = binaryTree.generate(10, 15);
console.log(bt_maze.toString(),"\n\n");

// generate a 10x15 maze using sidewinder algorithm
console.log(`sidewinder ${10}x${15}`)
const sw_maze = binaryTree.generate(10, 15);
console.log(sw_maze.toString());