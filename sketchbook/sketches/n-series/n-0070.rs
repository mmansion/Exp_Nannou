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
use library::grid::Grid;

// // module tree
//use crate::grid::Grid as Grid;

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

    grid: Grid,
    dir: f32,
    scale: f32,
    color: Hsva,
}

//--------------------------------------------------------
fn model(app: &App) -> Model {
    let rect = Rect::from_w_h(WIDTH, HEIGHT);

    // app.set_loop_mode(LoopMode::loop_once());
    // app.set_loop_mode(LoopMode::rate_fps(0.1));

    app.new_window().size(800, 800).view(view).build().unwrap();

    let mut last_calc = Duration::from_millis(0);

    let grid = Grid::new(10.0, 10.0, 10, 10, &rect);

    //--------------------------------------------------------
    let mut this_capture_frame = 0;
    let mut last_capture_frame = 0;

    let dir = 1.0;
    let scale = 0.0;
    let color = hsva(1.0, 1.0, 1.0, 1.0);

    //--------------------------------------------------------

    Model {
        this_capture_frame,
        last_capture_frame,
        last_calc,
        grid,
        dir,
        scale,
        color,
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

    //--------------------------------------------------------

    if m.scale > 1.5 {
        m.dir = -1.0;
    }

    if m.scale < 0.0 {
        m.dir = 1.0;
    }

    let speed = 0.03;
    m.scale = app.time * speed * m.dir;

    m.color = hsva(
        map_range((app.time * 0.1), -1.0, 1.0, 0.0, 1.0),
        1.0,
        1.0,
        1.0,
    );
}

fn view(app: &App, m: &Model, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();
    let win = app.window_rect();
    let time = app.time;

    //--------------------------------------------------------
    // background

    let bg = rgba(0.0, 0.0, 0.0, 0.001);

    if app.elapsed_frames() == 10 {
        // draw.background().color(rgba(1.0, 1.0, 1.0, 1.0));
        draw.background().color(rgba(0.0, 0.0, 0.0, 1.0));
    } else {
        draw.rect()
            .x_y(0.0, 0.0)
            .w_h(win.w() * 2.0, win.w() * 2.0)
            .color(bg);
    }

    //--------------------------------------------------------
    //m.grid.draw(&draw);

    //--------------------------------------------------------
    let r = 200.0; //radius value

    let angles = 360;
    let mut points = vec![vec2(0.0, 0.0); angles];
    let offset_size = 20.0;
    let draw = draw.rotate(app.time * 0.1).scale(m.scale);

    for i in (0..angles).step_by(30) {
        let a = (i as f32).to_radians();
        let offset = map_range(
            (app.elapsed_frames() as f32 * 0.05).sin(),
            -1.0,
            1.0,
            -offset_size,
            offset_size,
        );

        let offset_r = r + offset;

        let x = a.cos() * offset_r;
        let y = a.sin() * offset_r;

        // points[i]
        points[i] = vec2(x, y);

        draw.rotate(app.time * 0.2)
            .ellipse()
            .x_y(x, y)
            .radius(m.scale * 2.0)
            .color(m.color);
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
