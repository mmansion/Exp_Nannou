/*
* n-0093
*
* recursive random, circle packing
*
* mikhail mansion 2022
*/

use nannou::prelude::*;
use std::time::Duration;

use nannou::prelude::*;
use nannou_touchosc::TouchOscClient;

//--------------------------------------------------------
static CAPTURE  : bool = false; // capture to image sequence (or use obs)
static FRAME    : bool = true; //hide window chrome when set to false
static WIDTH    : f32 = 800.0;
static HEIGHT   : f32 = 800.0; 
static BORDER   : f32 = 10.0;
static WAIT     : u128 = 100;

// Make sure this matches the `TARGET_PORT` in the `osc_sender.rs` example.
const PORT: u16 = 6555;


pub struct Circle {
    x: f32,
    y: f32,
    radius: f32
}

impl Circle {
    pub fn new(min_diam:f32, max_diam:f32) -> Self {

        let radius = random_range(min_diam, max_diam);
        let x = random_range(radius, WIDTH - radius);
        let y = random_range(radius, HEIGHT - radius);

        Circle {
            x,
            y,
            radius
        }
    }
}
//--------------------------------------------------------
fn main() {
    nannou::app(model).update(update).run();
}

//--------------------------------------------------------
struct Model {
    window_id: WindowId,
    this_capture_frame : i32,
    last_capture_frame : i32,
    last_calc : Duration,
    redraw:bool,
    last_redraw: u128,
    touchosc: TouchOscClient
}

//--------------------------------------------------------
fn model(app: &App) -> Model {

    let window_id = app
        .new_window()
        .size(WIDTH as u32, HEIGHT as u32)
        .decorations(FRAME) //creates a borderless window
        .view(view)
        .build()
        .unwrap()
        ;

    // To initialize a TouchOscClient you instantiate it as "mutable"
    // and pass the OSC port where messages will be received.
    let mut touchosc = TouchOscClient::new(6555);

    touchosc.verbose();//enable debugging

    // Adding touchosc client inputs.

    // touchosc.add_radio("/invert", 2, 0);
   
    
    // app.set_loop_mode(LoopMode::loop_once());
    // app.set_loop_mode(LoopMode::rate_fps(0.1));
    
    let mut last_calc = Duration::from_millis(0);

    //--------------------------------------------------------
    let mut this_capture_frame = 0;
    let mut last_capture_frame = 0;

    //--------------------------------------------------------
    let mut redraw = false;
    let mut last_redraw = 0;

    //--------------------------------------------------------


    //--------------------------------------------------------

    Model {
        window_id,
        this_capture_frame, 
        last_capture_frame, 
        last_calc,
        colors,
        redraw,
        last_redraw,
        touchosc,
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

    //--------------------------------------------------------
    // redraw framerate workaround
    // change WAIT to increase interval

    if( _update.since_start.as_millis() - m.last_redraw > WAIT) {
        m.last_redraw = _update.since_start.as_millis();
        m.redraw = true;
    } else {
        m.redraw = false;
    }
    //--------------------------------------------------------

    m.touchosc.update(); 

    //--------------------------------------------------------





}

fn view(app: &App, m: &Model, frame: Frame) {

    if(m.redraw) {

        // get canvas to draw on
        let draw  = app.draw();
        let win   = app.window_rect();
        let time  = app.time;
    
        draw.background().color(BLACK);
        //--------------------------------------------------------
       
       

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

}