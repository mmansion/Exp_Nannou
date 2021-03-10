use nannou::prelude::*;
use nannou::noise::*;
use nannou::Draw;
use std::collections::VecDeque;
use std::time::Duration;

pub mod lib;

// module tree
use crate::lib::points::Point as Point;
use crate::lib::vehicles::Vehicle as Vehicle;

static CAPTURE  : bool = true; // capture to image sequence
static WIDTH    : i32 = 800;
static HEIGHT   : i32 = 800; 
static ANGLES   : i32 = 3;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    points : Vec<Vector2>,
    incs   : Vec<f32>,
    noise  : Perlin,
    xOff   : f64, 
    yOff   : f64,
    this_capture_frame : i32,
    last_capture_frame : i32,
    new_frame : bool,
    last_calc : Duration,
    inc : f32,
}

// ----------------------------------------------------------------------
fn model(app: &App) -> Model {

    let rect = Rect::from_w_h( WIDTH, HEIGHT );
    
    // app.set_loop_mode(LoopMode::loop_once());
    app.set_loop_mode(LoopMode::rate_fps(0.1));
    
    app.new_window()
        .size(800, 800)
        .view(view)
        .build()
        .unwrap();

    let mut noise = Perlin::new();
    noise = noise.set_seed(1);
    let mut xOff = 0.0;
    let mut yOff = 0.0;
    let mut points = Vec::new();
    let mut incs = Vec::new();
    let mut new_frame = false;
    let mut last_calc = Duration::from_millis(0);
    let mut inc = 0.0;
    //----------------------------------
    let mut this_capture_frame = 0;
    let mut last_capture_frame = 0;
    //----------------------------------

    // setup incs
    for i in 0..ANGLES+1 {
        incs.push(i as f32 * random_f32());
    }

    Model {this_capture_frame, last_capture_frame, noise, points, incs,xOff, yOff, new_frame, last_calc, inc}
} 

fn update(app: &App, m: &mut Model, _update: Update) {

    // ref:
    //https://doc.rust-lang.org/nightly/core/time/struct.Duration.html
    //let millis = Duration::from_millis(100).as_millis();

    let since_last_calc = _update.since_start.as_millis() - m.last_calc.as_millis();

    if since_last_calc > 10  {

        m.last_calc = _update.since_start;

        m.inc += 1.5;

        m.new_frame = true;

        for inc in m.incs.iter_mut() {
            *inc += 0.05;
        }

        if m.this_capture_frame != m.last_capture_frame {
            m.last_capture_frame = m. this_capture_frame;
        }
    
        if CAPTURE {
            m.this_capture_frame += 1;
        }
    
        
    } else {
        m.new_frame = false;
    }
}

fn view(app: &App, m: &Model, frame: Frame) {

    // get canvas to draw on
    let draw  = app.draw();
    let win   = app.window_rect();
    let t     = app.time;

    draw.background().color(rgba(1.0, 0.4, 0.3, 1.0));
    
    for n in (0..20).rev() { 

        let atten = 0.1;
        let scale = (n as f32) * atten;
        let mut xStore = 0.0;
        let mut yStore = 0.0;

        let rad_a = 20.0;
        let rad_b = 20.0;
        let num_cusps = 12.0;


        let pts = (0..ANGLES + 1).rev().map(|i| {

            let inc =  ( (360 / ANGLES * i) as f32).to_radians();
            let ix  = i as usize;
            
            let x = ( (num_cusps-1.0) * inc.cos() * rad_b) + (( (num_cusps -1.0) * inc).cos() * rad_a );
            //let y = (inc.sin() * rad_b;
            let y = ( (num_cusps-1.0) * inc.sin() * rad_b) + (( (num_cusps -1.0) * inc).sin() * rad_b );

            let r = 0.4;

            let mut xOff = 0.0;
            let mut yOff = 0.0;

    
            pt2(x + xOff, y + yOff)


        });  
        
        let hue = app.time * 2.0 * PI;
        
        let color = hsla(hue, 0.5, 0.5, 1.0);
        //let color = rgba(0.0, 0.0, 0.0, 1.0);
        // let draw = draw.rotate( (t.sin() * n as f32) * 0.0001);
        let draw = draw.rotate( 3.0*PI/2.0 );
        let draw = draw.translate(pt3(-80.0, 0.0, 0.0));


        if n % 2 == 0 {
            draw
            .scale(scale)
            .polygon()
            .color(color)
            .no_fill()
            .stroke(rgba(0.0, 1.0, 0.5, 1.0))
            .stroke_weight(3.0 + (0.9 * scale))
            .points(pts)
            ;
        } else {
            draw
            .scale(scale)
            .polygon()
            //.color(BLUE)
            .no_fill()
            .stroke(rgba(0.0, 0.5, 0.8, 0.5))
            .stroke_weight(2.5 + (0.5 * scale))
            .points(pts)
            ;
        }

    }



    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();

    


    // end draw frame ---------------------------------------------------------

    
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