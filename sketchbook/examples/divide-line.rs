// Example 1-5: Vector Magnitude
use nannou::prelude::*;

fn main() {
    nannou::sketch(view).size(800, 800).run();
}

fn view(app: &App, frame: Frame) {
    // Begin drawing
    let draw = app.draw();
    let win = app.window_rect();

    draw.background().color(WHITE);

    let vec_pt1 = vec2(0.0, 0.0);
    let mut vec_pt2 = vec2(app.mouse.x, app.mouse.y);

    // subtract vectors and get the magnitude; order doesn't matter
    let mag = (vec_pt2 - vec_pt1).length();
    
    const segments:usize = 6;

    //implement the Default trait if the element type allows it
    let mut vertices: [Vec2; segments] = Default::default(); //array of len 6, retuns default type for Vec2 (0.0, 0.0)
    // let array: [Vec<u8>; 10] = Default::default();

    for n in 1..segments {

        let l = mag / segments as f32 * n as f32;
        vertices[n] = vec_pt1 + (vec_pt2 - vec_pt1).normalize() * l;

        draw.ellipse()
            .xy(vertices[n])
            .w_h(10.0, 10.0)
            .color(BLACK);
    }

    draw.line().weight(2.0).color(BLACK).points(vec_pt1, vec_pt2);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}