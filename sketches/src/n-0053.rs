use nannou::prelude::*;
use nannou::prelude::Point2;
// use std::ops::Range;
use nannou::noise::*;
// use nannou::Draw;
// use std::collections::VecDeque;
use std::time::Duration;

//--------------------------------------------------------

static CAPTURE  : bool = false; // capture to image sequence
static WIDTH    : f32 = 800.0;
static HEIGHT   : f32 = 800.0; 

static RESOLUTION: f32 = 30.0;
static ITERATIONS: i32 = 10000;

//--------------------------------------------------------

pub mod lib;

// module tree
use crate::lib::grid::Grid as Grid;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    this_capture_frame : i32,
    last_capture_frame : i32,
    last_calc : Duration,
    grid      : Grid,
    noise     : Perlin,
    noiseGen  : Point2,
    xOff      : f32, 
    yOff      : f32,
}

// ----------------------------------------------------------------------
fn model(app: &App) -> Model {

    let rect = Rect::from_w_h( WIDTH, HEIGHT );

    //app.set_loop_mode(LoopMode::loop_once());
    
    // app.set_loop_mode(LoopMode::loop_once());
    // app.set_loop_mode(LoopMode::rate_fps(0.1));
    
    app.new_window()
        .size(800, 800)
        .view(view)
        .build()
        .unwrap();

    let grid = Grid::new(10.0, 10.0, 10, 10, &rect);

    //--------------------------------------------------------
    let mut noise = Perlin::new();
    noise = noise.set_seed(1);
    let mut noiseGen = pt2(0.0, 0.0);

    let xOff = 0.0;
    let yOff = 0.0;

    //--------------------------------------------------------

    let mut last_calc = Duration::from_millis(0);
    //----------------------------------
    let mut this_capture_frame = 0;
    let mut last_capture_frame = 0;

    //----------------------------------

    Model {
        this_capture_frame, 
        last_capture_frame, 
        last_calc,
        // texture,
        grid,
        noise,
        noiseGen,
        xOff,
        yOff,
    }
} 

fn update(app: &App, m: &mut Model, _update: Update) {

    // ref:
    //https://doc.rust-lang.org/nightly/core/time/struct.Duration.html
    //let millis = Duration::from_millis(100).as_millis();
    let since_last_calc = _update.since_start.as_millis() - m.last_calc.as_millis();
    if since_last_calc > 10  {
        // timed interval
    }

    if m.this_capture_frame != m.last_capture_frame {
        m.last_capture_frame = m. this_capture_frame;
    }

    if CAPTURE {
        m.this_capture_frame += 1;
    }

    //--------------------------------------------------------
    //calculations here

    // noise
    m.noiseGen.x+=0.1;
    m.noiseGen.y+=0.1;

    m.xOff = m.noise.get([m.noiseGen.x as f64, m.noiseGen.y as f64]) as f32;
    m.yOff = m.noise.get([m.noiseGen.x as f64, m.noiseGen.y as f64]) as f32;
    //--------------------------------------------------------
}

fn get_value(x:f32, y:f32) -> f32 {
    return (x*0.01).cos() +  ((y*0.001).sin() * (PI * 2.0));
}

fn view(app: &App, m: &Model, frame: Frame) {

    // get canvas to draw on
    let draw  = app.draw();
    let win   = app.window_rect();
    let t     = app.time;

    // draw background----------------------------------------------------------

    let bg = rgba(0.0, 0.0, 0.2, 0.4);

    if app.elapsed_frames() == 1 { //initial background pass
        draw.background().color(rgba(0.0, 0.0, 0.0, 0.1));
    } else {
        draw.rect().x_y(0.0, 0.0).w_h(win.w()*2.0, win.w()*2.0).color(bg);
    }

    //--------------------------------------------------------
    //m.grid.draw(&draw);
    // let draw = draw.rotate(t * 0.8);

    // let draw = draw.scale(0.5);
    let draw = draw.translate(pt3(-WIDTH/2.0, -HEIGHT/2.0, 0.0));

    for i in 0..ITERATIONS {
       
        let x = random_f32() * WIDTH;
        let y = random_f32() * HEIGHT;
        let r = get_value(x * (t.cos() * 1.1), y * (t.sin() * 1.1));

        let start_pt = pt2(0.0, 0.0);
        let end_pt   = pt2(RESOLUTION + (t.sin() * 0.1), 0.0);

        draw
        .line()
        .x_y(x, y)
        .rotate(r)
        .stroke_weight(y * 0.001)
        .color(hsv(
            map_range(y + t.sin(), 0.0, WIDTH, 0.8, 1.0), 
            1.0, 
            1.0))
        .points(start_pt, end_pt)
        ;
   
    }
    draw
    .x_y(WIDTH/2.0, HEIGHT/2.0)
    .ellipse()
    .color(BLACK)
    .radius(200.0)
    ;
    


    //--------------------------------------------------------
    
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