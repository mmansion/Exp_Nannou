use nannou::prelude::*;

fn main() {
    nannou::sketch(view)
        .size(800, 800)
        .loop_mode(LoopMode::loop_once())
        .run();
    // app.set_loop_mode(LoopMode::loop_once());
}

fn view(app: &App, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();

    draw.background().color(WHITE);

    let radius = 200.0;
    let count = 10000;
    let pos = pt2(0.0, 0.0);

    for i in 0..count {
        let angle = random::<f32>() * PI * 2.0;

        let r = random::<f32>().sqrt(); //adding sqrt to distribute

        let x = radius * angle.cos() * r + pos.x;
        let y = radius * angle.sin() * r + pos.y;

        draw.ellipse().x_y(x, y).w_h(3.0, 3.0).color(BLACK);
    }

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}
