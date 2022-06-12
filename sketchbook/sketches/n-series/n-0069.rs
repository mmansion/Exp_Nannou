/*
* daily sketching
* blob shape
*
* mikhail mansion Jul 24 2021
*/

use nannou::geom::Point2;
use nannou::geom::*;
use nannou::prelude::*;
use nannou::Draw;
use std::ops::Range;
use std::time::Duration;

// use library::grid;

// mod colors;
// mod quadtree;
// use crate::colors::Palette;
// use crate::grid::
// use library;
// pub mod library;

// // module tree
// use crate::lib::grid::Grid as Grid;

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
}

//--------------------------------------------------------
fn model(app: &App) -> Model {
    let rect = Rect::from_w_h(WIDTH, HEIGHT);

    // app.set_loop_mode(LoopMode::loop_once());
    // app.set_loop_mode(LoopMode::rate_fps(0.1));

    app.new_window().size(800, 800).view(view).build().unwrap();

    let mut last_calc = Duration::from_millis(0);

    //--------------------------------------------------------
    let mut this_capture_frame = 0;
    let mut last_capture_frame = 0;

    //--------------------------------------------------------

    Model {
        this_capture_frame,
        last_capture_frame,
        last_calc,
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

    let bg = rgba(1.0, 1.0, 1.0, 0.001);

    if app.elapsed_frames() == 10 {
        draw.background().color(rgba(1.0, 1.0, 1.0, 1.0));
    } else {
        draw.rect()
            .x_y(0.0, 0.0)
            .w_h(win.w() * 2.0, win.w() * 2.0)
            .color(bg);
    }

    //--------------------------------------------------------
    let r = 200.0; //radius value

    let angles = 360;
    // initialize array with 360 Vec2 vars
    // let mut points = [vec2(0.0,0.0); angles];

    // create a vec array type
    // initialize each array position with a value (vec2)
    // let a = [0; 30], initializes zero to the first 30 elements
    let mut points = vec![vec2(0.0, 0.0); angles];

    let offset_size = 50.0;

    let draw = draw.rotate(app.time * 0.1);
    let draw = draw.scale(map_range(time.sin() * 0.2, -1.0, 1.0, 0.01, 3.0));

    for i in (0..angles).step_by(20) {
        // let i = i as f32;
        let a = (i as f32).to_radians();
        // println!("{}", a.sin() * 100.0);
        //let offset = map_range((app.elapsed_frames() as f32 * 0.05).sin() * 50.0, -50.0, 50.0, -offset_size, offset_size);

        // let offset_r = r + offset;
        let offset_r = r;

        let x = a.cos() * offset_r;
        let y = a.sin() * offset_r;

        // points[i]
        points[i] = vec2(x, y);

        draw.rotate(app.time * 0.2)
            .ellipse()
            .x_y(x, y)
            .radius(map_range(abs(time.sin() * 0.2), 0.0, 1.0, 1.0, 4.0))
            .hsva(
                map_range(abs(time.sin() * 0.1), 0.4, 0.9, 0.1, 0.75),
                1.0,
                1.0,
                0.4,
            );
        // ...
    }

    // draw
    // .rotate(app.time)
    // .polyline()
    // .stroke_weight(1.0)
    // .caps_round()
    // .color(WHITE)
    // .points(points)
    // ;
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
