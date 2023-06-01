/*
* name
*
* description of app
*
* mikhail mansion YYYY
*/

use nannou::prelude::*;
use nannou::geom::*;
use nannou::geom::Point2;
use std::ops::Range;
use nannou::Draw;
use std::time::Duration;

use library::grid;

// mod colors;
// mod quadtree;
// use crate::colors::Palette;
// use crate::grid::

// use library;

// pub mod library;

// // module tree
// use crate::lib::grid::Grid as Grid;

//--------------------------------------------------------
static CAPTURE  : bool = false; // capture to image sequence
static WIDTH    : f32 = 800.0;
static HEIGHT   : f32 = 800.0; 

//--------------------------------------------------------
fn main() {
    nannou::app(model).update(update).run();
}

//--------------------------------------------------------
struct Model {
    this_capture_frame : i32,
    last_capture_frame : i32,
    last_calc : Duration,
    trans_perc: f32,
    start_time: f32,
}

//--------------------------------------------------------
fn model(app: &App) -> Model {

    let rect = Rect::from_w_h( WIDTH, HEIGHT );
    
    // app.set_loop_mode(LoopMode::loop_once());
    // app.set_loop_mode(LoopMode::rate_fps(0.1));
    
    app.new_window()
        .size(800, 800)
        .view(view)
        .build()
        .unwrap();

    let mut last_calc = Duration::from_millis(0);

    //--------------------------------------------------------
    let mut this_capture_frame = 0;
    let mut last_capture_frame = 0;

    let mut start_time = app.time;
    let mut trans_perc = 0.0;

    //--------------------------------------------------------

    Model {
        this_capture_frame, 
        last_capture_frame, 
        last_calc,
        trans_perc,
        start_time
    }
} 

fn update(app: &App, m: &mut Model, _update: Update) {

    // ref:
    //https://doc.rust-lang.org/nightly/core/time/struct.Duration.html
    //let millis = Duration::from_millis(100).as_millis();
    let since_last_calc = _update.since_start.as_millis() - m.last_calc.as_millis();
    if since_last_calc > 10  { //time interval
        m.last_calc = _update.since_start;
    }

    if m.this_capture_frame != m.last_capture_frame {
        m.last_capture_frame = m. this_capture_frame;
    }

    if CAPTURE {
        m.this_capture_frame += 1;
    }

    let end = 1.0;
    if end - m.trans_perc > 0.0 {
        m.trans_perc = ease_in_quad(app.time - m.start_time, 0.0, end, 3.0);
    } else {
        m.trans_perc = end;
    }

    println!("trans_perc = {}", m.trans_perc);
}

fn view(app: &App, m: &Model, frame: Frame) {

    // get canvas to draw on
    let draw  = app.draw();
    let win   = app.window_rect();
    let time  = app.time;

    //--------------------------------------------------------
    // background

    let bg = rgba(0.13, 0.0, 0.1, 0.01);

    if app.elapsed_frames() == 10 { //must clear render context once for fullscreen
        draw.background().color(rgba(0.0, 0.0, 0.0, 0.9));
    } else {
        draw.rect().x_y(0.0, 0.0).w_h(win.w()*2.0, win.w()*2.0).color(bg);
    }
    
    //--------------------------------------------------------

    draw.line()
        .start(pt2(-400.0, 0.0))
        .end(pt2(-400.0 + (800.0 * m.trans_perc), 0.0))
        .color(rgba(1.0, 1.0, 1.0, 1.0));

    
    //--------------------------------------------------------
    // draw frame
    
    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();

    //--------------------------------------------------------
    // capture frame

    if m.this_capture_frame != m.last_capture_frame {      
        let directory  = "captures/".to_string();
        let app_name   = app.exe_name().unwrap().to_string();
        let extension  = ".png".to_string();
        let frame_num  = format!("{:05}", m.this_capture_frame);

        let path = format!("{}{}{}", directory, frame_num, extension);
        app.main_window().capture_frame(path);
    }
}
// fn easeLinear(t:f32, b:f, c, d) -> f32{
//     return c * t / d + b;
// }
fn ease_in_quad(t:f32, b:f32, c:f32, d:f32) -> f32 {
    c * pow(t/d,2) + b
}