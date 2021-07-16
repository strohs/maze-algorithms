// Make an instance of two and place it on the page.
const elem = document.getElementById('draw-shapes');
const params = { width: 1000, height: 1000 };
const two = new Two(params).appendTo(elem);

// two has convenience methods to create shapes.
const circle = two.makeCircle(90, 100, 50);
const rect = two.makeRectangle(213, 100, 100, 100);

// angles expressed in radians
// 1 radian = 57.296 degrees
// - radians draw CCW and + radians draw CW
// 45 deg = Math.PI / 4
// 90 deg = Math.PI / 2
// 135 deg = 3 * Math.PI / 4
// 180 deg = Math.PI
//const arc = two.makeArcSegment(400, 100, 20, 30, 0, Math.PI * 3 / 4);



// The object returned has many styleable properties:
circle.fill = '#FF8000';
circle.stroke = 'orangered'; // Accepts all valid css color
circle.linewidth = 5;

rect.fill = 'rgb(0, 200, 255)';
rect.opacity = 0.75;
rect.noStroke();

// arc.stroke = `grey`;
// arc.fill = '#FF1111'
// arc.linewidth = 2;

// // draw inner wall in red
// const iw = two.makeArcSegment(400, 100, 20, 20, 0, Math.PI * 3 / 4);
// iw.stroke = '#fa23e4';
// iw.linewidth = 2;
//
// // draw outer wall in yellow
// const ow = two.makeArcSegment(400, 100, 30, 30, 0, Math.PI * 3 / 4);
// ow.stroke = '#6c00b4';
// ow.linewidth = 2;
//
// // draw the ccw wall
// const ccw = two.makeArcSegment(400, 100, 20, 30, 0, 0);
// ccw.stroke = 'rgb(0,255,59)';
// ccw.linewidth = 2;
//
// // draw the cw wall
// const cw = two.makeArcSegment(400, 100, 20, 30, Math.PI * 3 / 4, Math.PI * 3 / 4);
// cw.stroke = '#ffeb00';
// cw.linewidth = 2;

// Don't forget to tell two to render everything
// to the screen
//two.update();


function drawEmptyPolarGrid(rows, cols, ringHeight, ox, oy) {
    const cell_count = rows * cols;
    const theta = 2 * Math.PI / cols;

    for (let r = 0; r < rows; r++) {
        for (let c = 0; c < cols; c++) {

            const inner_radius = r * ringHeight;
            const outer_radius = (r + 1) * ringHeight;
            const theta_ccw = c * theta;
            const theta_cw = (c + 1) * theta;
            const arc = two.makeArcSegment(ox, oy, inner_radius, outer_radius, theta_ccw, theta_cw);
            arc.stroke = '#000000';
            arc.linewidth = 2;
        }
    }
}

drawEmptyPolarGrid(8, 24, 25, 500, 500);
two.update();



// var elem = document.getElementById('draw-group');
// var two = new Two({ width: 285, height: 200 }).appendTo(elem);
//
// var circle = two.makeCircle(-70, 0, 50);
// var rect = two.makeRectangle(70, 0, 100, 100);
// circle.fill = '#FF8000';
// circle.stroke = 'orangered';
// rect.fill = 'rgba(0, 200, 255, 0.75)';
// rect.stroke = '#1C75BC';
//
// // Groups can take an array of shapes and/or groups.
// var group = two.makeGroup(circle, rect);
//
// // And have translation, rotation, scale like all shapes.
// group.translation.set(two.width / 2, two.height / 2);
// group.rotation = Math.PI;
// group.scale = 0.75;
//
// // You can also set the same properties a shape have.
// group.linewidth = 7;
//
// two.update();





// var elem = document.getElementById('draw-animation');
// var two = new Two({ type: Two.Types.canvas, width: 285, height: 200 }).appendTo(elem);
//
// var circle = two.makeCircle(-70, 0, 50);
// var rect = two.makeRectangle(70, 0, 100, 100);
// circle.fill = '#FF8000';
// rect.fill = 'rgba(0, 200, 255, 0.75)';
//
// var group = two.makeGroup(circle, rect);
// group.translation.set(two.width / 2, two.height / 2);
// group.scale = 0;
// group.noStroke();
//
// // Bind a function to scale and rotate the group
// // to the animation loop.
// two.bind('update', function(frameCount) {
//     // This code is called everytime two.update() is called.
//     // Effectively 60 times per second.
//     if (group.scale > 0.9999) {
//         group.scale = group.rotation = 0;
//     }
//     var t = (1 - group.scale) * 0.125;
//     group.scale += t;
//     group.rotation += t * 4 * Math.PI;
// }).play();  // Finally, start the animation loop