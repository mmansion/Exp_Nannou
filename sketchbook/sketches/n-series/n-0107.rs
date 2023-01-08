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



fn main() {
    nannou::app(model).run();
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
    vehicles.push(Vehicle::new(0.0, 0.0, 5.0, vec2(0.0, 0.0), 20));

    Model {
        window_id,
        this_capture_frame, 
        last_capture_frame, 
        last_calc,
        colors,
        redraw,
        last_redraw,
        touchosc,
        vehicles
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

    // for v in 0..m.vehicles.len() {

    //     for i in 0..m.points.len() {

    //         m.vehicles[v].redirect( &m.points[i] );
    //         // let steer = force.limit_magnitude(m.vehicles[v].max_force);
    //         // m.vehicles[v].apply_force(steer);
    //     }
        
    //     m.vehicles[v].boundaries(&app.window_rect());
    //     m.vehicles[v].update();
    // }

}


fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    draw.background().color(WHITE);
    

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