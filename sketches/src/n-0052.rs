use nannou::prelude::*;
use nannou::prelude::Point2;
// use std::ops::Range;
use nannou::noise::*;
// use nannou::Draw;
// use std::collections::VecDeque;
use std::time::Duration;

//--------------------------------------------------------

static CAPTURE  : bool = true; // capture to image sequence
static WIDTH    : f32 = 800.0;
static HEIGHT   : f32 = 800.0; 

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

fn view(app: &App, m: &Model, frame: Frame) {

    // get canvas to draw on
    let draw  = app.draw();
    let win   = app.window_rect();
    let t     = app.time;

    // draw background----------------------------------------------------------

    let bg = rgba(0.0, 0.0, 0.2, 0.08);

    if app.elapsed_frames() == 1 { //initial background pass
        draw.background().color(rgba(0.0, 0.0, 0.0, 0.1));
    } else {
        draw.rect().x_y(0.0, 0.0).w_h(win.w()*2.0, win.w()*2.0).color(bg);
    }

    //--------------------------------------------------------
    // m.grid.draw(&draw);
    // let draw = draw.rotate(t * 0.8);

    let draw = draw.scale(0.5);

    let draw = draw.translate(pt3(-75.0, -75.0, 0.0));

    for p in 0..m.grid.points.len() {

        let r = abs(30.0 * (t.sin())) + 100.0;

        let pts = (0..3).map(|i| {
            let a = i as f32;
            let x = a.cos() * r;
            let y = a.sin() * r; 
            pt2(x, y)
        });

        draw
        .polygon()
        .x_y(
            m.grid.points[p].x + (t + p as f32).sin() * 100.0 + m.xOff, 
            m.grid.points[p].y + (t + p as f32).sin() * 10.0 + m.yOff)
        .no_fill()
        .stroke(rgba(t.sin() * 0.1, 1.0, t.sin() * 0.1, 0.9))
        .stroke_weight(1.0)
        .points(pts)
        ;
    }


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