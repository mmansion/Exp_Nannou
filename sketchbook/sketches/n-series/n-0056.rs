use nannou::prelude::*;
use nannou::geom::*;
use nannou::geom::Point2;
use std::ops::Range;
use nannou::Draw;
use std::collections::VecDeque;
use std::time::Duration;

pub mod lib;

// module tree
use crate::lib::points::Point as Point;
use crate::lib::vehicles::Vehicle as Vehicle;

static CAPTURE  : bool = true; // capture to image sequence
static WIDTH    : f32 = 800.0;
static HEIGHT   : f32 = 800.0; 

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    this_capture_frame : i32,
    last_capture_frame : i32,
    last_calc : Duration,
}

// ----------------------------------------------------------------------
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

    //----------------------------------

    let mut this_capture_frame = 0;
    let mut last_capture_frame = 0;
  
    //----------------------------------

    //----------------------------------

    Model {
        this_capture_frame, 
        last_capture_frame, 
        last_calc
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
}

fn view(app: &App, m: &Model, frame: Frame) {

    // get canvas to draw on
    let draw  = app.draw();
    let win   = app.window_rect();
    let t     = app.time;

    // draw background----------------------------------------------------------


    let bg = rgba(0.13, 0.0, 0.1, 0.01);

    if app.elapsed_frames() == 1 { 
        draw.background().color(rgba(0.0, 0.0, 0.0, 0.9));
    } else {
        draw.rect().x_y(0.0, 0.0).w_h(win.w()*2.0, win.w()*2.0).color(bg);
    }

    // draw
    // .line()
    // .color(PALEGOLDENROD)
    // .points(pt2(0.0, win.top()), pt2(0.0, win.bottom()))
    // ;

    let draw = draw.rotate(t * 0.5);


    let center:Point2 = pt2(200.0, t.sin() * 100.0);
    let radii:Vector2 = vec2(200.0, 200.0);
    let sweep_angle_radians:f32 = PI/2.0;
    let x_rotation_radians:f32 = 0.0;

    let arc1 = nannou::geom::path()
        .arc(center, radii, sweep_angle_radians, x_rotation_radians)
        .build();


    let center:Point2 = pt2(-200.0, t.sin() * 100.0);
    let radii:Vector2 = vec2(200.0, 200.0);
    let sweep_angle_radians:f32 = PI/2.0;
    let x_rotation_radians:f32 = 0.0;

    let arc2 = nannou::geom::path()
        .arc(center, radii, sweep_angle_radians, x_rotation_radians)
        .build();

    draw
        .path()
        .stroke()
        .stroke_weight(1.5)
        .x_y(-200.0,0.0)
        .color(WHITE)
        .events(arc1.iter());

    draw
        .path()
        .stroke()
        .stroke_weight(1.5)
        .x_y(200.0,0.0)
        .color(WHITE)
        .events(arc2.iter());

    
    //draw.path().color(WHITE);
    // draw frame -------------------------------------------------------------
    
    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();

    // capture fram -----------------------------------------------------------

    if m.this_capture_frame != m.last_capture_frame {
            
        // let mut owned_string: String = "hello ".to_owned();
        // let borrowed_string: String = "output/" + app.exe_name().unwrap() + ".png";
    
        let directory  = "captures/".to_string();
        let app_name   = app.exe_name().unwrap().to_string();
        // let frame_num  = m.this_capture_frame.to_string();
        let extension  = ".png".to_string();

        let frame_num = format!("{:05}", m.this_capture_frame);

        let path = format!("{}{}{}", directory, frame_num, extension);

        app.main_window().capture_frame(path);
        
    }
}