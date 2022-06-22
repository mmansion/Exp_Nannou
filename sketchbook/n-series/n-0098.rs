use nannou::prelude::*;

//--------------------------------------------------------
static FILENAME: &str = "n-0098";
static CAPTURE: bool = true;
static FRAME: bool = true; //hide window chrome when set to false

static SIZE: u32 = 800;

fn main() {
    nannou::app(model).run();
}

struct Model {
    window_id: WindowId,
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(SIZE, SIZE)
        .decorations(FRAME) //creates a borderless window
        .view(view)
        .build()
        .unwrap();

    app.set_loop_mode(LoopMode::loop_once());

    Model { window_id }
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();

    draw.background().color(WHITE);

    let draw = draw.rotate(PI / 2.0);

    let count = 20000;
    let point_size = 2.0;
    let rect_width = win.w() / 2.0;

    for i in 0..count {
        let rrand = rect_width * random::<f32>() * random::<f32>();
        let y = map_range(i as f32, 0.0, count as f32, win.h() / 2.0, -win.h() / 2.0);
        let x = rrand;
        draw.rotate(PI)
            .ellipse()
            .x_y(x, y)
            .w_h(point_size, point_size)
            .color(BLACK);
        // for ii in 0..3 {
        //     let y_off = 50.0 * ii as f32;
        // }
    }

    draw.rect()
        .x_y(0.0, 0.0)
        .w_h(rect_width, rect_width * 1.5)
        .color(WHITE)
        .stroke_weight(1.0)
        .stroke_color(rgba(0.0, 0.0, 0.0, 0.4));

    draw.rect()
        .x_y(0.0, 0.0)
        .w_h(rect_width * 0.5, rect_width)
        .color(WHITE)
        .stroke_weight(1.0)
        .stroke_color(rgba(0.0, 0.0, 0.0, 1.0));

    for i in 0..count / 2 {
        let rrand = (rect_width * 0.25) * random::<f32>() * random::<f32>();
        let x = -rect_width / 4.0 + rrand;
        let y = map_range(
            i as f32,
            0.0,
            (count / 2) as f32,
            -rect_width / 2.0,
            rect_width / 2.0,
        );
        draw.ellipse()
            .x_y(x, y)
            .w_h(point_size / 2.0, point_size / 2.0)
            .color(BLACK);
        // for ii in 0..3 {
        //     let y_off = 50.0 * ii as f32;
        // }
    }

    //--------------------------------------------------------
    if CAPTURE {
        let directory = "captures/".to_string();
        let app_name = app.exe_name().unwrap().to_string();
        let extension = ".png".to_string();

        let path = format!("{}{}{}", directory, FILENAME, extension);
        println!("Capturing {}", path);
        app.main_window().capture_frame(path);
    }

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}
