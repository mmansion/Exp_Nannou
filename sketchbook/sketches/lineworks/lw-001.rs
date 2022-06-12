// simple line perspective using stroke

use nannou::prelude::*;
use nannou_touchosc::TouchOscClient;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    touchosc: TouchOscClient,
}

fn model(app: &App) -> Model {
    app.new_window().size(600, 600).view(view).build().unwrap();

    // EXAMPLE: Initializing the client.
    // To initialize a TouchOscClient you instantiate it as "mutable"
    // and pass the OSC port where messages will be received.
    let mut touchosc = TouchOscClient::new(6555);

    touchosc.verbose();

    touchosc.add_fader("/spacing", 12.0, 35.0, 12.0);
    // touchosc.add_fader("/y_off", 0.0, -app.window_rect().h()/2.0, 1.0);

    // EXAMPLE: Adding client inputs.
    // Any type of TouchOSC controller inputs can be added to the TouchOscClient instance.
    // Inputs are initialized by calling their respective add_ method, and passing initialization values.
    // See the README documentaiton for a breakdown of the init values used for each type of TouchOSC controller.
    // touchosc.add_button("/show_points", true);
    // touchosc.add_radio("/invert", 2, 0);
    // touchosc.add_grid("/grid", 2, 3.0, 24.0, 10.0);
    // touchosc.add_encoder("/rotate", 0.0, PI * 2.0, 0.0);
    // touchosc.add_radial("/offset", 0.0, 10.0, 0.0);
    // touchosc.add_fader("/color_r", 0.0, 1.0, 1.0);
    // touchosc.add_fader("/color_g", 0.0, 1.0, 0.0);
    // touchosc.add_fader("/color_b", 0.0, 1.0, 1.0);
    // touchosc.add_xy("/scale", 0.1, 3.0, 1.0);
    // touchosc.add_fader("/stroke_width", 1.0, 10.0, 2.0);
    // touchosc.add_fader("/vertices", 3.0, 8.0, 3.0);
    // touchosc.add_radar("/scale_rotate", (0.1, 10.0, 1.0), (0.0, PI * 2.0, PI / 4.0));

    Model { touchosc }
}

fn update(app: &App, m: &mut Model, _update: Update) {
    // EXAMPLE: Updating values.
    // To receive OSC values from the TouchOSC controller, run the update function.
    // If messages available, they'll be routed to the associated TouchOSC client input and saved.
    // Note that values do persist after the application is terminated.
    m.touchosc.update();
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);
    draw.background().color(rgb(0.99, 0.99, 0.99));

    let win_w = app.window_rect().w();
    let win_h = app.window_rect().h();

    let s = m.touchosc.fader("/spacing");
    let n = 10;

    let c = rgb(0.1, 0.1, 0.1);

    for l in 0..n {
        let y = s * l as f32;
        let w = (n - l) as f32 * (1.0 / s * 9.0);

        let draw = draw.translate(pt3(0.0, -win_h / 2.0, 0.0));

        draw.line()
            .color(c)
            .stroke_weight(w)
            .points(pt2(-win_w / 2.0, y), pt2(win_w / 2.0, y));

        if l == n - 1 {
            let rect_w = 200.0 - s * 2.0;
            let rect_h = 250.0 - s * 2.0;
            
            let rect_x = 0.0;
            let rect_y = y + rect_h / 2.0;

            let pts = [
                pt2(-rect_w / 2.0, rect_h / 2.0),
                pt2(rect_w / 2.0, rect_h / 2.0),
                pt2(rect_w / 2.0, -rect_h / 2.0),
                pt2(-rect_w / 2.0, -rect_h / 2.0),
                pt2(-rect_w / 2.0, rect_h / 2.0),
            ];

            draw.polyline()
                .xy(pt2(rect_x, rect_y))
                .stroke_weight(1.0 + w)
                .color(c)
                .points_closed(pts);

            // draw.rect()
            //     .xy(pt2(rect_x, rect_y))
            //     .w_h(rect_w, rect_h)
            //     .stroke_weight(1.0 + w)
            //     .color(WHITE);


            draw.line()
                .color(c)
                .stroke_weight(0.5 + w)
                .points(pt2(-win_w / 2.0, 0.0), pt2(rect_x-rect_w/2.0, y));

            draw.line()
                .color(c)
                .stroke_weight(0.5 + w)
                .points(pt2(win_w / 2.0, 0.0), pt2(rect_x+rect_w/2.0, y));

        }
    }

    
    //--------------------------------------------------------

    // let draw = draw.rotate(PI/3.0);
    // let draw = draw.rotate(grid_rotate).translate(pt3(x_off, y_off, 0.0));

    // for c in 1..cols as i32 {
    //     for r in 1..rows as i32 {
    //         let n = rows * cols;
    //         let f = (c * r) as f32 / n;
    //         let w = (rotate).sin() * (rotate + f * PI * 2.0).cos();
    //         let rotation = rotate + (w * offset);
    //         let x = x_space * c as f32;
    //         let y = y_space * r as f32;

    //         let radius = 20.0;
    //         let points = (0..=360).step_by(360 / vertices).map(|i| {
    //             let radian = deg_to_rad(i as f32);
    //             let x = radian.sin() * radius;
    //             let y = radian.cos() * radius;
    //             pt2(x * x_scale, y * y_scale)
    //         });
    //         draw.translate(pt3(x, y, 0.0))
    //             .rotate(rotation)
    //             .scale(scale * stroke_width * 0.1 + 1.0)
    //             .polygon()
    //             .stroke_weight(stroke_width)
    //             .stroke_color(stroke_color)
    //             .color(fill_color)
    //             .points(points);

    //         if grid_points {
    //             draw.ellipse().color(GRAY).x_y(x, y).radius(10.0);
    //         }
    //     }
    // }

    draw.to_frame(app, &frame).unwrap();
}
