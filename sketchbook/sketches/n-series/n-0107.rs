use nannou::image::imageops::rotate180;
use nannou::prelude::*;
use std::time::Duration;
use nannou_touchosc::TouchOscClient;
use library::colors::Palette;
use library::vehicle::Vehicle as Vehicle;

//--------------------------------------------------------
static FILENAME: &str = "n-0107";
static CAPTURE  : bool = false; // capture to image sequence (or use obs)
static FRAME    : bool = false; //hide window chrome when set to false
static WIDTH    : f32 = 1080.0*0.5;
static HEIGHT   : f32 = 1920.0*0.5; 
static BORDER   : f32 = 10.0;
static WAIT     : u128 = 100;

static NUM_VEHICLES : usize = 30;
static VEHICLE_MAX_SPEED : f32 = 2.0;
static SCALE_FACTOR : f32 = 0.01;

pub struct Palette2 {
    pub colors: Vec<Rgb>,
    pub len: usize,
}
impl Palette2 {
    pub fn new() -> Self {
        //anime sky
        let raw_colors: [u32; 49] = [
            0xFF15283D, 0xFF0F1925, 0xFF203D59, 0xFF2E2A33, 0xFF3B4259, 0xFF487EB3, 0xFF4F537E,
            0xFF325C83, 0xFF5A5366, 0xFF5696C3, 0xFF2D3A68, 0xFF71729D, 0xFF4C344D, 0xFF6B5457,
            0xFF785272, 0xFF7B697E, 0xFF472429, 0xFF43649F, 0xFF682D44, 0xFF61AEE9, 0xFF9387AA,
            0xFF9D4A60, 0xFF822E37, 0xFFB98377, 0xFF87A0D1, 0xFFAA6E81, 0xFFC5737A, 0xFFB69EB0,
            0xFF8D5658, 0xFF907070, 0xFFD69D9E, 0xFFF5BC9F, 0xFFB87BA0, 0xFFFFFCE1, 0xFFFCDCC5,
            0xFF73D3F6, 0xFFE287A3, 0xFFDA4945, 0xFFF19888, 0xFFFDD89E, 0xFFEAC2BE, 0xFFFEF3C6,
            0xFFD89A76, 0xFFD8616A, 0xFFF6B873, 0xFFB4594E, 0xFFF17F63, 0xFFE0E1EA, 0xFFA4A9A5,
        ];
        let raw_colorsv = raw_colors.to_vec();

        //do the conversion myself
        let mut cols_rgb: Vec<Rgb> = raw_colorsv
            .into_iter()
            .map(|c| {
                let blue: u8 = (c & 0xFF) as u8;
                let green: u8 = ((c >> 8) & 0xFF) as u8;
                let red: u8 = ((c >> 16) & 0xFF) as u8;
                let c = Srgb::new(
                    red as f32 / 255.0,
                    green as f32 / 255.0,
                    blue as f32 / 255.0,
                );
                c
            })
            .collect();

        //sort on sat/value/hue
        cols_rgb.sort_unstable_by(|&a, &b| {
            let ahsv: Hsv = a.into();
            let bhsv: Hsv = b.into();
            //colors are rgb
            //convert to hsv
            let ahue = ahsv.hue.to_positive_radians();
            let bhue = bhsv.hue.to_positive_radians();
            ahue.partial_cmp(&bhue).unwrap()
        });

        let len = cols_rgb.len();
        Palette2 {
            colors: cols_rgb,
            len: len,
        }
    }

    pub fn somecolor_frac(&self, mut frac: f32) -> Rgb {
        while frac < 0.0 {
            frac += 1.0;
        }
        while frac >= 1.0 {
            frac -= 1.0;
        }

        let index = (frac * self.colors.len() as f32) as usize;
        self.colors[index]
    }
}

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    window_id: WindowId,
    this_capture_frame : i32,
    last_capture_frame : i32,
    last_calc : Duration,
    colors:Palette,
    palette: Palette2,
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


    for i in 0..NUM_VEHICLES {

        let x = map_range(i, 0, NUM_VEHICLES, -WIDTH*0.5, WIDTH*0.5);
        let y = HEIGHT*0.5;

        vehicles.push(Vehicle::new(x, y, VEHICLE_MAX_SPEED, vec2(0.0, 10.0), 10));
    }
    // loop code here
    

    Model {
        window_id,
        this_capture_frame, 
        last_capture_frame, 
        last_calc,
        colors,
        palette: Palette2::new(),
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
    if since_last_change > 1000  { //time interval
        change_dir = true;
        m.last_change = _update.since_start;
        // println!("change");

        

        
    }
    // println!("{}", since_last_change);

    for v in 0..m.vehicles.len() {

        m.vehicles[v].nib_color = m.palette.somecolor_frac(app.time*0.001);

        m.vehicles[v].nib_size = (1200.0 * abs(app.time.sin()*0.001)) + 2.0;
        m.vehicles[v].rotate((app.time).sin()*SCALE_FACTOR);
        if change_dir {
            // force = vec2(force.x + vel.x * -0.1,force.y+vel.y * -0.5);
            //     // random_range(0.0, 2.0 * PI), 
            //     // random_range(0.0, 2.0 * PI));
        } 
        
        let force = vec2(0.0, 0.01);

        let steer = force.clamp_length_max(m.vehicles[v].max_force);
        m.vehicles[v].apply_force(force);
        // m.vehicles[v].boundaries2(&app.window_rect(), 10);
        m.vehicles[v].boundaries_loop(&app.window_rect());
        m.vehicles[v].update();
    }

}


fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    
    // background
    
    let bg = rgba(0.1, 0.1, 0.2, 0.008);

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
   
    draw.ellipse()
        .x_y(0.0, 0.0)
        .w_h(300.0, 300.0)
        .color(bg);

    //draw a circle
    let circle_resolution = 225;
    let circle_rad = 150.0;
    let angle = TAU / circle_resolution as f32;
    for i in 0..circle_resolution {
        let x = (angle * i as f32).cos() * circle_rad * 2.0;
        let y = (angle * i as f32).sin() * circle_rad * 2.0;
        draw.line()
            .start(pt2(0.0, 0.0))
            .end(pt2(x, y))
            .stroke_weight(1.0)
            .caps_round()
            .color(m.palette.somecolor_frac(app.time*0.0001));
    }

    //--------------------------------------------------------
    // capture frame

    if CAPTURE && m.this_capture_frame != m.last_capture_frame {
        let directory = "captures/".to_string();
        let app_name = app.exe_name().unwrap().to_string();
        let extension = ".png".to_string();
        let frame_num = format!("{:05}", m.this_capture_frame);

        let path = format!("{}{}{}", directory, frame_num, extension);
        app.main_window().capture_frame(path);
    }


    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}