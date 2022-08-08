import Grid from "../src/Grid.js";
import chai from "chai";

let should = chai.should() //actually call the function

describe('Grid', function() {
  describe('constructor', function() {
    it('should build a 3x3 grid', function() {
      let grid = new Grid(3, 3);
      grid.size().should.equal(9);
    });
  })

  describe('northwest neighbors', function() {
    it('grid[0][0] should not have north and west neighbors', function() {
      let grid = new Grid(3, 3);

      should.not.exist(grid.getCell(0, 0).north);
      should.not.exist(grid.getCell(0, 0).west);
      should.exist(grid.getCell(0, 0).east);
      should.exist(grid.getCell(0, 0).south);
    });
  });

  describe('southeast neighbors', function() {
    it('grid[2][2] should not have south and east neighbors', function() {
      let grid = new Grid(3, 3);

      should.not.exist(grid.getCell(2, 2).south);
      should.not.exist(grid.getCell(2, 2).east);
      should.exist(grid.getCell(2, 2).north);
      should.exist(grid.getCell(2, 2).west);
    });
  });

  describe('inner neighbors', function() {
    it('inner grid cells should have all neighbors', function() {
      let grid = new Grid(3, 3);

      should.exist(grid.getCell(1, 1).south);
      should.exist(grid.getCell(1, 1).north);
      should.exist(grid.getCell(1, 1).west);
      should.exist(grid.getCell(1, 1).east);
    });
  });

  describe('grid cell iterator', function() {
    it('should iterate over all grid cells in row order', function() {
      let grid = new Grid(3, 3);

      let curRow = 0;
      let curCol = 0;
      for(let c of grid) {
        if (curCol >= grid.cols) {
          curRow += 1;
          curCol = 0;
        }
        c.row.should.equal(curRow);
        c.col.should.equal(curCol);
        curCol++;
      }
      
    });
  });

  describe('grid row iterator', function() {
    it('should iterate over all grid rows', function() {
      let grid = new Grid(3, 3);

      let curRow = 0;
      for(let row of grid.rowIterator()) {
        row.length.should.equal(3);
        for (let c of row) {
          c.row.should.equal(curRow);
        }
        curRow++;
      }

    });
  });


});