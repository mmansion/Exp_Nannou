use nannou::prelude::*;
use nannou::image;

fn main() {
    nannou::sketch(view)
        .size(400, 400)
        .loop_mode(LoopMode::loop_once())
        .run();
}

fn view(app: &App, frame: Frame) {

    let draw = app.draw();

    draw.background().color(BLACK);

    let steps = 30;
    for step in 0..steps {
        let degree = (360/steps * step) as f32; 
        let hsv = Hsv::new(degree, 1.0, 0.5);
        let x = (degree.to_radians()).sin() * 100.0;
        let y = (degree.to_radians()).cos() * 100.0;
        draw.ellipse()
            .x_y(x, y)
            .w_h(30.0, 30.0)
            .color(hsv);
    }

    // draw.rect()
    //     .x_y(-100.0, 100.0)
    //     .w_h(100.0, 100.0)
    //     .color(hsv_col_nannou_norm);

    // draw.rect()
    //     .x_y(100.0, 100.0)
    //     .w_h(100.0, 100.0)
    //     .color(hsv_col_palette_deg);

    // draw.rect()
    //     .x_y(-100.0, -100.0)
    //     .w_h(100.0, 100.0)
    //     .color(hsv_col_nannou_deg);

    // draw.rect()
    //     .x_y(100.0, -100.0)
    //     .w_h(100.0, 100.0)
    //     .color(hsv_col_palette_norm);

    draw.to_frame(app, &frame).unwrap();
}
