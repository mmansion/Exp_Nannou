use nannou::prelude::*;

fn main() {

    nannou::sketch(view);
}

fn view(app: &App, frame: &Frame) {
    
    // prepare to draw
    let draw = app.draw();

    // clear the background to purple
    draw.background().color(PLUM);

    // draw a blue ellipse with default size and position
    draw.ellipse().color(STEELBLUE);

    // write to the window frame
    draw.to_frame(app, &frame).unwrap();
}