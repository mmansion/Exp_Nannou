use nannou::image::imageops::rotate180;
use nannou::prelude::*;
use std::time::Duration;
use nannou_touchosc::TouchOscClient;
use library::colors::Palette;
use library::vehicle::Vehicle as Vehicle;

//--------------------------------------------------------
static FILENAME: &str = "n-0107";
static CAPTURE  : bool = false; // capture to image sequence (or use obs)
static FRAME    : bool = true; //hide window chrome when set to false
static WIDTH    : f32 = 800.0;
static HEIGHT   : f32 = 800.0; 
static BORDER   : f32 = 10.0;
static WAIT     : u128 = 100;

static VEHICLE_MAX_SPEED : f32 = 3.0;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    window_id: WindowId,
    this_capture_frame : i32,
    last_capture_frame : i32,
    last_calc : Duration,
    colors:Palette,
    redraw:bool,
    last_redraw: u128,
    touchosc: TouchOscClient,
    vehicles: Vec<Vehicle>,
    last_change: Duration,
}

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

    let mut last_calc = Duration::from_millis(0);
    
    let mut last_change = Duration::from_millis(0);

    //--------------------------------------------------------
    let mut this_capture_frame = 0;
    let mut last_capture_frame = 0;

    //--------------------------------------------------------
    let mut redraw = false;
    let mut last_redraw = 0;

    //--------------------------------------------------------

    let colors = Palette::new();
    

    // app.set_loop_mode(LoopMode::loop_once());

    let mut vehicles = Vec::new();
    vehicles.push(Vehicle::new(1.0, 1.0, VEHICLE_MAX_SPEED, vec2(0.0, -1.0), 10));

    Model {
        window_id,
        this_capture_frame, 
        last_capture_frame, 
        last_calc,
        colors,
        redraw,
        last_redraw,
        touchosc,
        vehicles,
        last_change
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

    //----------------------------------

    // VEHICLES

    let since_last_change = _update.since_start.as_millis() - m.last_change.as_millis();
    let mut change_dir = false;
    if since_last_change > 100  { //time interval
        change_dir = true;
        m.last_change = _update.since_start;
        // println!("change");
        
    }
    // println!("{}", since_last_change);

    for v in 0..m.vehicles.len() {

        m.vehicles[v].rotate((app.time).sin()*0.1);
        if change_dir {
            // force = vec2(force.x + vel.x * -0.1,force.y+vel.y * -0.5);
            //     // random_range(0.0, 2.0 * PI), 
            //     // random_range(0.0, 2.0 * PI));
        } 
        
        let force = vec2(0.0, 0.01);

        let steer = force.clamp_length_max(m.vehicles[v].max_force);
        m.vehicles[v].apply_force(force);
        m.vehicles[v].boundaries2(&app.window_rect(), 10);
        m.vehicles[v].update();
    }

}


fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    
    // background
    
    let bg = rgba(0.1, 0.1, 0.2, 0.001);

    if app.elapsed_frames() == 10 { //must clear render context once for fullscreen
        draw.background().color(rgba(0.0, 0.0, 0.0, 0.9));
    } else {
        draw.rect().x_y(0.0, 0.0).w_h(win.w()*2.0, win.w()*2.0).color(bg);
    }

    //--------------------------------------------------------

    for v in 0..m.vehicles.len() {

        m.vehicles[v].display(&draw);
    }
        
    

    //--------------------------------------------------------
    if CAPTURE {
        let directory = "captures/".to_string();
        let app_name = app.exe_name().unwrap().to_string();
        let extension = ".png".to_string();

        let path = format!("{}{}{}", directory, FILENAME, extension);
        println!("Capturing {}", path);
        app.main_window().capture_frame(path);
    }

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}