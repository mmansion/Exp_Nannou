use nannou::prelude::*;

fn perpendicular_cw(v: Vec2) -> Vec2 {
    vec2(v.y, -v.x)
}
fn perpendicular_ccw(v: Vec2) -> Vec2 {
    vec2(-v.y, v.x)
}

fn main() {
    nannou::sketch(view).size(800, 800).run();
}

fn view(app: &App, frame: Frame) {
    // Begin drawing
    let draw = app.draw();
    let win = app.window_rect();

    draw.background().color(WHITE);

    let orig = vec2(0.0, 0.0);
    let mut mouse = vec2(app.mouse.x, app.mouse.y);

    // subtract vectors and get the magnitude; order doesn't matter
    let mag = (mouse - orig).length();
    
    const segments:usize = 6;
    let mut vertices: [Vec2; segments+1] = Default::default();
    // let array: [Vec<u8>; 10] = Default::default();

    draw.line().weight(2.0).color(BLACK).points(orig, mouse);

    for n in 0..segments+1 {

        let l = mag / segments as f32 * n as f32;
        vertices[n] = orig + (mouse - orig).normalize() * l;

        draw.ellipse()
            .xy(vertices[n])
            .w_h(10.0, 10.0)
            .color(BLACK);

        let start = vertices[n] - orig;
        let perp_cw_vec = perpendicular_cw(start);
        let perp_ccw_vec = perpendicular_ccw(start);
        let draw2 = draw.translate(vec3(vertices[n].x, vertices[n].y, 0.0));
        draw2.line().weight(2.0).color(BLACK).points(orig, perp_cw_vec);
        draw2.line().weight(2.0).color(BLACK).points(orig, perp_ccw_vec);
    }


    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

