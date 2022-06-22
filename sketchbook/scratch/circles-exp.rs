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

    let y_off = 20.0;

    for i in 0..count {

        let angle = map_range(i as f32, 0.0, count as f32, 0.0, PI);

        let r = 1.0 - random::<f32>() * random::<f32>() * random::<f32>() * random::<f32>();

        let x = radius * 1.5 * angle.cos() * r + pos.x;
        let y = radius * 1.5 * angle.sin() * r + pos.y + y_off;

        draw.ellipse().x_y(x, y).w_h(2.0, 2.0).color(BLACK);
    }

    for i in 0..count {

        let angle = map_range(i as f32, 0.0, count as f32, 0.0, PI);

        let r = 1.0 - random::<f32>() * random::<f32>() * random::<f32>() * random::<f32>();

        let x = radius * angle.cos() * r + pos.x;
        let y = radius * angle.sin() * r + pos.y - y_off;

        draw.ellipse().x_y(x, y).w_h(2.0, 2.0).color(BLACK);
    }

     for i in 0..count {

        let angle = map_range(i as f32, 0.0, count as f32, PI, PI*2.0);

        let r = 1.0 - random::<f32>() * random::<f32>() * random::<f32>() * random::<f32>();

        let x = radius * angle.cos() * r + pos.x;
        let y = radius * angle.sin() * r + pos.y - y_off;

        draw.ellipse().x_y(x, y).w_h(2.0, 2.0).color(BLACK);
    }
    let rw = radius * 1.5;

    let rect_points = [
        pt2(-rw, y_off),
        pt2(-rw, -rw),
        pt2(rw, -rw),
        pt2(rw, y_off),
    ];

    draw
    .polyline()
    .stroke_weight(1.0)
    .color(BLACK)
    .points(rect_points)
    ;

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}
