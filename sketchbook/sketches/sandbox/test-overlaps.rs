use nannou::prelude::*;
use nannou::image;

fn main() {
    nannou::sketch(view)
        .size(800, 800)
        .loop_mode(LoopMode::loop_once())
        .run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    draw.rect()
        .x_y(0.0, 0.0)
        .w_h(100.0, 100.0)
        .color(BLACK);
    
     draw.rect()
        .x_y(50.0, 50.0)
        .w_h(100.0, 100.0)
        .color(BLACK);

    draw.background().color(WHITE);

    draw.to_frame(app, &frame).unwrap();
}
