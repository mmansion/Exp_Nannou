/*
* name
* description of app
* mikhail mansion YYYY
*/

use nannou::geom::Point2;
use nannou::geom::*;
use nannou::prelude::*;
use nannou::Draw;
use std::ops::Range;
use std::time::Duration;

use library::grid2::Grid2;

//--------------------------------------------------------
static CAPTURE: bool = true; // capture to image sequence
static WIDTH: f32 = 800.0;
static HEIGHT: f32 = 800.0;

//--------------------------------------------------------
fn main() {
    nannou::app(model).update(update).run();
}

//--------------------------------------------------------
struct Model {
    this_capture_frame: i32,
    last_capture_frame: i32,
    last_calc: Duration,
    grid: Grid2,
}

//--------------------------------------------------------
fn model(app: &App) -> Model {
    let rect = Rect::from_w_h(WIDTH, HEIGHT);

    // app.set_loop_mode(LoopMode::loop_once());
    // app.set_loop_mode(LoopMode::rate_fps(0.1));

    app.set_loop_mode(LoopMode::NTimes {
        // two frames are necessary for capture_frame to work properly
        number_of_updates: 2,
    });

    app.new_window().size(800, 800).view(view).build().unwrap();

    let w = rect.w();
    let h = rect.h();

    let mut last_calc = Duration::from_millis(0);

    //--------------------------------------------------------
    let mut grid = Grid2::new(10.0, 10.0, 10, 10, &rect);
    let scale_factor = 100.0;

    for i in 0..grid.points.len() {
        let resolution = 100.0;
        let inc = i as f32;
        let vec = vec2(
            (inc.to_radians()).cos() * scale_factor,
            (inc.to_radians()).sin() * scale_factor,
        );

        grid.set_angle(i, vec);
    }

    //--------------------------------------------------------
    let mut this_capture_frame = 0;
    let mut last_capture_frame = 0;

    //--------------------------------------------------------

    Model {
        this_capture_frame,
        last_capture_frame,
        last_calc,
        grid,
    }
}

fn update(app: &App, m: &mut Model, _update: Update) {
    // ref:
    //https://doc.rust-lang.org/nightly/core/time/struct.Duration.html
    //let millis = Duration::from_millis(100).as_millis();
    let since_last_calc = _update.since_start.as_millis() - m.last_calc.as_millis();
    if since_last_calc > 10 {
        //time interval
        m.last_calc = _update.since_start;
    }

    if m.this_capture_frame != m.last_capture_frame {
        m.last_capture_frame = m.this_capture_frame;
    }

    if CAPTURE {
        m.this_capture_frame += 1;
    }
}

fn view(app: &App, m: &Model, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();
    let win = app.window_rect();
    let time = app.time;

    //--------------------------------------------------------
    // background

    let bg = rgba(1.0, 1.0, 1.0, 1.0);

    if app.elapsed_frames() == 1 {
        draw.background().color(rgba(0.0, 0.0, 0.0, 0.9));
    } else {
        draw.rect()
            .x_y(0.0, 0.0)
            .w_h(win.w() * 2.0, win.w() * 2.0)
            .color(bg);
    }

    //--------------------------------------------------------

    m.grid.draw(&draw);

    for i in 0..m.grid.points.len() {
        let rotation = m.grid.angles[i];
        let position = m.grid.points[i];

        //let d = draw.rotate( rotation.angle()  );
        let _draw = draw.translate(pt3(position.x, position.y, 0.0));

        _draw.line().points(pt2(0.0, 0.0), rotation);

        _draw
            .rect()
            .w_h(10.0, 10.0)
            .xy(position)
            .stroke_weight(10.0)
            .color(BLACK);
    }

    //--------------------------------------------------------
    // draw frame

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();

    //--------------------------------------------------------
    // capture frame

    if m.this_capture_frame != m.last_capture_frame {
        let directory = "captures/".to_string();
        let app_name = app.exe_name().unwrap().to_string();
        let extension = ".png".to_string();
        let frame_num = format!("{:05}", m.this_capture_frame);

        let path = format!("{}{}{}", directory, frame_num, extension);
        app.main_window().capture_frame(path);
    }
}
