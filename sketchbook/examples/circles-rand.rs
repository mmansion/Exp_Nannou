use nannou::prelude::*;

fn main() {
    nannou::sketch(view)
        .size(800, 800)
        .loop_mode(LoopMode::loop_once())
        .run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    let radius = 400.0;
    let count = 10000;
    let pos = pt2(0.0, 0.0);

    for i in 0..count {
        let angle = random::<f32>() * PI * 2.0;

        let r = random::<f32>();
        let x = pos.x + r * radius * angle.cos();
        let y = pos.y + r * radius * angle.sin();

        draw.ellipse().x_y(x, y).w_h(3.0, 3.0).color(BLACK);
    }

    draw.to_frame(app, &frame).unwrap();
}
