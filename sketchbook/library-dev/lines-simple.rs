/*
* n-0107
*
* evolving the library::line
*
* mikhail mansion 2022
*/

use nannou::prelude::*;
use std::time::Duration;
use library::colors::Palette;
use nannou_touchosc::TouchOscClient;

use library::line::Line as Line;

//--------------------------------------------------------
static CAPTURE  : bool = false; // capture to image sequence (or use obs)
static FRAME    : bool = true; //hide window chrome when set to false
static WIDTH    : f32 = 800.0;
static HEIGHT   : f32 = 800.0; 
static BORDER   : f32 = 10.0;
static WAIT     : u128 = 100;

// Make sure this matches the `TARGET_PORT` in the `osc_sender.rs` example.
const PORT: u16 = 6555;

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
    colors:Palette,
    redraw:bool,
    last_redraw: u128,
    touchosc: TouchOscClient,

    line: Line,
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

    // EXAMPLE: Adding client inputs.
    // Any type of TouchOSC controller inputs can be added to the TouchOscClient instance.
    // Inputs are initialized by calling their respective add_ method, and passing initialization values.
    // See the README documentaiton for a breakdown of the init values used for each type of TouchOSC controller.
    // touchosc.add_button("/show_points", true);
    // touchosc.add_radio("/invert", 2, 0);
    // touchosc.add_grid("/grid", 2, 3.0, 24.0, 10.0);
    // touchosc.add_encoder("/rotate", 0.0, PI * 2.0, 0.0);
    // touchosc.add_radial("/offset", 0.0, 10.0, 0.0);
    // touchosc.add_fader("/color_r", 0.0, 1.0, 1.0);
    // touchosc.add_fader("/color_g", 0.0, 1.0, 0.0);
    // touchosc.add_fader("/color_b", 0.0, 1.0, 1.0);
    // touchosc.add_fader("/color_a", 0.0, 1.0, 1.0);
    // touchosc.add_xy("/scale", 0.1, 3.0, 1.0);
    // touchosc.add_fader("/stroke_width", 1.0, 10.0, 2.0);
    // touchosc.add_fader("/vertices", 3.0, 8.0, 3.0);
    // touchosc.add_radar("/scale_rotate", (0.1, 10.0, 1.0), (0.0, PI * 2.0, PI / 4.0));


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
    let colors = Palette::new();

    let mut line = Line::new(vec2(-WIDTH/2.0, -HEIGHT/2.0), vec2(WIDTH/2.0, HEIGHT/2.0));
    line.color(rgba(1.0, 0.0, 1.0, 1.0));
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
        line
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
 
}

fn view(app: &App, m: &Model, frame: Frame) {

    if(m.redraw) {

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
        
        m.line.draw(&draw);

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