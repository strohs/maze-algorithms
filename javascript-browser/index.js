// Make an instance of two and place it on the page.
var elem = document.getElementById('draw-shapes');
var params = { width: 285, height: 200 };
var two = new Two(params).appendTo(elem);

// two has convenience methods to create shapes.
var circle = two.makeCircle(90, 100, 50);
var rect = two.makeRectangle(213, 100, 100, 100);

// The object returned has many stylable properties:
circle.fill = '#FF8000';
circle.stroke = 'orangered'; // Accepts all valid css color
circle.linewidth = 5;

rect.fill = 'rgb(0, 200, 255)';
rect.opacity = 0.75;
rect.noStroke();

// Don't forget to tell two to render everything
// to the screen
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