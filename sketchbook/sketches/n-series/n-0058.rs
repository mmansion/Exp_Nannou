use nannou::geom::Point2;
use nannou::geom::*;
use nannou::prelude::*;
use nannou::Draw;
use std::ops::Range;
use std::time::Duration;

use library::bezier::Bezier;
use library::grid;

//--------------------------------------------------------
static CAPTURE: bool = false; // capture to image sequence
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

    curves: Vec<Bezier>,
    incs: Vec<f32>,
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

    let mut curves = Vec::new();
    let mut incs = Vec::new();

    let angles = 60;
    let radius = 350.0;

    for i in 0..angles {
        let inc = ((360 / angles * i) as f32).to_radians();

        let x = inc.cos() * radius;
        let y = inc.sin() * radius;

        let p1 = pt2(x, y);

        let cp1 = pt2(x * 0.25, y * 0.25);
        let cp2 = pt2(x * 0.75, y * 0.75);
        let p2 = pt2(x, y);

        curves.push(Bezier::new(p1, cp1, cp2, p2));

        curves[i].show_handles = false;
        curves[i].stroke_weight(0.5);
    }

    // setup incs
    for i in 0..angles {
        incs.push(i as f32 * random_f32());
    }

    //--------------------------------------------------------

    Model {
        this_capture_frame,
        last_capture_frame,
        last_calc,
        curves,
        incs,
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

        //increment incs
        for inc in m.incs.iter_mut() {
            *inc += 0.05;
        }
    }

    if m.this_capture_frame != m.last_capture_frame {
        m.last_capture_frame = m.this_capture_frame;
    }

    if CAPTURE {
        m.this_capture_frame += 1;
    }

    //--------------------------------------------------------
    for i in 0..m.curves.len() {
        let t = app.time;
        let x = t.cos() * 50.0;
        let y = t.sin() * 50.0;

        m.curves[i].end_point = pt2(x, y);

        let xOff = (m.incs[i]).cos() * 50.0;
        let yOff = (m.incs[i]).sin() * 50.0;

        m.curves[i].control_point_1 = pt2(xOff, yOff);
    }
    //--------------------------------------------------------
}

fn view(app: &App, m: &Model, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();
    let win = app.window_rect();
    let time = app.time;

    //--------------------------------------------------------
    // background

    let bg = rgba(1.0, 1.0, 1.0, 0.05);

    if app.elapsed_frames() == 1 {
        draw.background().color(rgba(0.0, 0.0, 0.0, 0.9));
    } else {
        draw.rect()
            .x_y(0.0, 0.0)
            .w_h(win.w() * 2.0, win.w() * 2.0)
            .color(bg);
    }

    //--------------------------------------------------------

    for i in 0..m.curves.len() {
        m.curves[i].draw(&draw);
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
