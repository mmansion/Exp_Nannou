//ref https://guide.nannou.cc/tutorials/basics/drawing-2d-shapes.html
use nannou::prelude::*;

fn main() {

    nannou::sketch(view);
}

fn view(app: &App, frame: &Frame) {
    
    // prepare to draw
    let draw = app.draw();

    // clear the background to purple
    draw.background().color(BLACK);

    // draw a red ellipse with default size and position
    draw.ellipse().color(RED).x_y(-300.0, 100.0);

    // draw an orange rectangle
    draw.rect().color(ORANGE)
               .w(200.0)
               .h(100.0)
               .x_y(400.0, 0.0);


    let point1 = pt2(-100.0, -20.0);
    let point2 = pt2(-10.0, -30.0);
    let point3 = pt2(-15.0, 40.0);
    let point4 = pt2(-80.0, 35.0);

    draw.quad()
        .color(STEELBLUE)
        .w(300.0)
        .h(200.0)
        .points(point1, point2, point3, point4);

    // draw a wavey line
    let points = (0..50).map(|i| {
        let x = (i as f32 - 25.0);          //subtract 25 to center the sine wave
        let point = pt2(x, x.sin()) * 20.0; //scale sine wave by 20.0
        (point, STEELBLUE)
        });
      draw.polyline()
          .weight(3.0)
          .colored_points(points);
      
    // write to the window frame
    draw.to_frame(app, &frame).unwrap();
}