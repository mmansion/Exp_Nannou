use nannou::prelude::*;
use nannou::noise::*;
use nannou::Draw;
use std::collections::VecDeque;

pub mod lib;

// module tree
use crate::lib::points::Point as Point;
use crate::lib::vehicles::Vehicle as Vehicle;

static CAPTURE  : bool = false; // capture to image sequence
static WIDTH    : i32 = 800;
static HEIGHT   : i32 = 800; 

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    noise: Perlin,
    xOff : f32, 
    yOff : f32,
    this_capture_frame : i32,
    last_capture_frame : i32,
}

// ----------------------------------------------------------------------
fn model(app: &App) -> Model {

    let rect = Rect::from_w_h( WIDTH, HEIGHT );

    app.new_window()
        .size(800, 800)
        .view(view)
        .build()
        .unwrap();

    let mut noise = Perlin::new();
    let mut xOff = 0;
    let mut yOff = 0;
    //----------------------------------
    let mut this_capture_frame = 0;
    let mut last_capture_frame = 0;
    //----------------------------------

    Model {this_capture_frame, last_capture_frame, noise}
} 

fn update(app: &App, m: &mut Model, _update: Update) {

    if m.this_capture_frame != m.last_capture_frame {
        m.last_capture_frame = m. this_capture_frame;
    }

    if CAPTURE {
        m.this_capture_frame += 1;
    }

}

fn view(app: &App, model: &Model, frame: Frame) {

    // get canvas to draw on
    let draw = app.draw();
    let win = app.window_rect();
    let bg = rgba(0.0, 0.0, 0.2, 0.4);
    let t = app.time;

    if t < 0.1 {
        draw.background().color(BLACK);
    } else {
        //background
        draw.rect().x_y(0.0, 0.0).w_h(win.w()*2.0, win.w()*2.0).color(bg)
        ;
    }


    // ------------------------------------------------

   

    
    
    

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();

    if model.this_capture_frame != model.last_capture_frame {
        
        // let mut owned_string: String = "hello ".to_owned();
        // let borrowed_string: String = "output/" + app.exe_name().unwrap() + ".png";
    
        let directory  = "captures/".to_string();
        let app_name   = app.exe_name().unwrap().to_string();
        // let frame_num  = model.this_capture_frame.to_string();
        let extension  = ".png".to_string();

        let frame_num = format!("{:05}", model.this_capture_frame);

        let path = format!("{}{}{}", directory, frame_num, extension);

        app.main_window().capture_frame(path);
        
    }
}