const binaryTree = require("./src/generator/binaryTree.js");

// generate a 10x15 maze using binary tree algorithm
const maze = binaryTree.generate(10, 15);
console.log(maze.toString());