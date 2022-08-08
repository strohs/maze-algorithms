import Cell from "../src/Cell.js";
import chai from "chai";

let should = chai.should(); //actually call the function


describe('Cell', function() {
    describe('north()', function() {
        it('should return a Cell to the north if cell has north neighbor', function() {
            let cell = new Cell(1, 1);
            let northCell = new Cell(0, 1);
            cell.north = northCell;

            cell.north.should.equal(northCell);
        });
    });

    describe('new cells have no neighbors set', function() {
        it('should return null for all neighbors when a new cell is created', function() {
            let cell = new Cell(1, 1);

            should.not.exist(cell.south);
            should.not.exist(cell.north);
            should.not.exist(cell.east);
            should.not.exist(cell.west);
        });
    });

    describe('link()', function() {
        it('should uni-directional link two cells', function() {
            let cell = new Cell(1, 1);
            let cell2 = new Cell(1, 2);

            cell.link(cell2, false);

            cell.isLinked(cell2).should.be.true;
            cell2.isLinked(cell).should.be.false;
        });
    });

    describe('link()', function() {
        it('should bidirectional link two cells', function() {
            let cell = new Cell(1, 1);
            let cell2 = new Cell(1, 2);

            cell.link(cell2);

            cell.isLinked(cell2).should.be.true;
            cell2.isLinked(cell).should.be.true;
        });
    });

    describe('unlink()', function() {
        it('should bidirectional unlink two cells', function() {
            let cell = new Cell(1, 1);
            let cell2 = new Cell(1, 2);

            cell.link(cell2);

            cell.isLinked(cell2).should.be.true;
            cell2.isLinked(cell).should.be.true;
            cell.unlink(cell2)
            cell.isLinked(cell2).should.be.false;
            cell2.isLinked(cell).should.be.false;
        });
    });

    describe('links()', function() {
        it('should iterate over two cells', function() {
            let cell = new Cell(1, 1);
            let cell2 = new Cell(1, 2);
            let cell3 = new Cell(2, 1);

            cell.link(cell2);
            cell.link(cell3);
            let cellCount = 0;
            for (let link of cell.links()) {
                cellCount++;
            }
            cellCount.should.equal(2);
        });
    });

    describe('neighbors()', function() {
        it('should have three neighbors', function() {
            let cell = new Cell(1, 1);
            let cell2 = new Cell(1, 2);
            let cell3 = new Cell(2, 1);
            cell.north = new Cell(0, 1);
            cell.south = cell3;
            cell.east = cell2;

            cell.neighbors().length.should.equal(3);
        });
    });


});