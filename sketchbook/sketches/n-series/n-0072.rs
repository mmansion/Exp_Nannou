/*
* n-0072
*
* daily sketching
*
* mikhail mansion 2021
*/

use nannou::geom::Point2;
use nannou::geom::*;
use nannou::prelude::*;
use nannou::Draw;
use std::ops::Range;
use std::time::Duration;

use library::easing;

//--------------------------------------------------------
static CAPTURE: bool = false; // capture to image sequence (or use obs)
static WIDTH: f32 = 800.0;
static HEIGHT: f32 = 800.0;
static OFFSET: f32 = 150.0;

static TOP: usize = 0;
static MID: usize = 1;
static BTM: usize = 2;

//--------------------------------------------------------
fn main() {
    nannou::app(model).update(update).run();
}

//--------------------------------------------------------
struct Model {
    // Store the window ID so we can refer to this specific window later if needed.
    _window: WindowId,
    this_capture_frame: i32,
    last_capture_frame: i32,
    last_calc: Duration,
    left_line: Vec<Vec2>,
    right_line: Vec<Vec2>,
    left_orig: Vec2,
    right_orig: Vec2,
}

//--------------------------------------------------------
fn model(app: &App) -> Model {
    let rect = Rect::from_w_h(WIDTH, HEIGHT);

    let _window = app
        .new_window()
        .size(WIDTH as u32, HEIGHT as u32)
        .decorations(false) //creates a borderless window
        .view(view)
        .build()
        .unwrap();

    let mut last_calc = Duration::from_millis(0);

    //--------------------------------------------------------
    let mut left_line = Vec::new();
    let mut right_line = Vec::new();

    let left_orig = vec2(-rect.w() / 2.0 + OFFSET * 2.0, 0.0);
    let right_orig = vec2(rect.w() / 2.0 - OFFSET * 2.0, 0.0);

    left_line.push(vec2(-rect.w() / 2.0 + OFFSET, rect.h() - OFFSET));
    left_line.push(vec2(-rect.w() / 2.0 + OFFSET * 2.0, 0.0));
    left_line.push(vec2(-rect.w() / 2.0 + OFFSET, -rect.h() + OFFSET));

    right_line.push(vec2(rect.w() / 2.0 - OFFSET, rect.h() - OFFSET));
    right_line.push(vec2(rect.w() / 2.0 - OFFSET * 2.0, 0.0));
    right_line.push(vec2(rect.w() / 2.0 - OFFSET, -rect.h() + OFFSET));

    //--------------------------------------------------------
    let mut this_capture_frame = 0;
    let mut last_capture_frame = 0;

    //--------------------------------------------------------

    Model {
        _window,
        this_capture_frame,
        last_capture_frame,
        last_calc,
        left_line,
        right_line,
        left_orig,
        right_orig,
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
    // update line points

    let t = app.time;
    let x = app.time.sin() * 0.9;
    // println!("{}", x);

    m.left_line[MID].x = m.left_orig.x + easing::ease_in_sin(x) * OFFSET;
    m.right_line[MID].x = m.right_orig.x + easing::ease_in_sin(x) * -OFFSET;

    //--------------------------------------------------------
}

fn view(app: &App, m: &Model, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();
    let win = app.window_rect();
    let time = app.time;

    //--------------------------------------------------------
    // background

    let bg = rgba(1.0, 1.0, 1.0, 0.001);

    if app.elapsed_frames() == 1 {
        //must clear render context once for fullscreen
        draw.background().color(rgba(0.0, 0.0, 0.0, 0.9));
    } else {
        draw.rect()
            .x_y(0.0, 0.0)
            .w_h(win.w() * 2.0, win.w() * 2.0)
            .color(bg);
    }

    //--------------------------------------------------------
    let draw = draw.rotate(time * 0.5);

    //--------------------------------------------------------
    let left_line_points = [m.left_line[TOP], m.left_line[MID], m.left_line[BTM]];

    draw.polyline()
        .weight(1.0)
        .color(BLACK)
        .points(left_line_points);

    let right_line_points = [m.right_line[TOP], m.right_line[MID], m.right_line[BTM]];

    draw.polyline()
        .weight(1.0)
        .color(BLACK)
        .points(right_line_points);

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
