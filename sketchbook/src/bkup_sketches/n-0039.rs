use nannou::prelude::*;
use nannou::noise::*;
use nannou::Draw;
use std::collections::VecDeque;

pub mod lib;

// module tree
use crate::lib::points::Point as Point;
use crate::lib::vehicles::Vehicle as Vehicle;

static CAPTURE  : bool = true; // capture to image sequence
static WIDTH    : i32 = 800;
static HEIGHT   : i32 = 800; 

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    points : Vec<Vector2>,
    noise  : Perlin,
    xOff   : f64, 
    yOff   : f64,
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
    noise = noise.set_seed(1);
    let mut xOff = 0.0;
    let mut yOff = 0.0;
    let mut points = Vec::new();
    //----------------------------------
    let mut this_capture_frame = 0;
    let mut last_capture_frame = 0;
    //----------------------------------

    Model {this_capture_frame, last_capture_frame, noise, points, xOff, yOff}
} 

fn update(app: &App, m: &mut Model, _update: Update) {

    app.set_loop_mode(LoopMode::rate_fps(1.0));

    if !m.points.is_empty() {
        m.points.clear();// clears the vector, removing all values
    }

    let w = WIDTH as f32;
    let h = HEIGHT as f32;
    m.points.push(pt2(-25.0, -h));
    // increment by a custom step value
    // see: iter step by method
    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.step_by
    
    let frac = (app.elapsed_frames() % 200) as f32 / (200.0);

    //we'll rotate in the noise space
    let rotcos = 0.002 * (frac * TAU).cos();
    let rotsin = 0.002 * (frac * TAU).sin();

    for x in (1..WIDTH+50).step_by(10) {
        let y = map_range( m.noise.get(
            [
                m.xOff, 
                m.yOff,
                rotcos as f64,
                rotsin as f64
            ]), 0., 1., 1, 10);

        m.xOff += 0.1;
        m.points.push(pt2(x as f32, y as f32));
    }
    
    m.yOff += 100.1;
    m.points.push(pt2(w+25.0, -h));
    

    if m.this_capture_frame != m.last_capture_frame {
        m.last_capture_frame = m. this_capture_frame;
    }

    if CAPTURE {
        m.this_capture_frame += 1;
    }

}

fn view(app: &App, m: &Model, frame: Frame) {

    // get canvas to draw on
    let draw = app.draw();
    let win = app.window_rect();
    let bg = rgba(0.0, 0.0, 0.2, 0.1);
    let t = app.time;

    if t < 0.1 {
        draw.background().color(BLACK);
    } else {
        //background
        draw.rect().x_y(0.0, 0.0).w_h(win.w()*2.0, win.w()*2.0).color(bg)
        ;
    }
    
   
    let draw2 = draw.translate( pt3(-win.w()/2.0-25.0, t.sin() * win.h()/4.0, 0.0));
    let draw3 = draw.translate( pt3(-win.w()/2.0-25.0, t.sin() * win.h()/4.0 * -1.0, 0.0));
    let color = hsva(t.sin() * 0.1, 1.0, 1.0, 1.0);

    // ------------------------------------------------ 
    let pts = (0..m.points.len()).map(|i| {
        (pt2(m.points[i].x, m.points[i].y))
    });
    let pts2 = (0..m.points.len()).map(|i| {
        (pt2(m.points[i].x, m.points[i].y))
    });

    draw2
    .polygon()
    .stroke_weight(1.0)
    .caps_round()
    .stroke(color)
    .no_fill()
    // .color(color2)
    .points(pts)
    ;

    draw3
    .polygon()
    .stroke_weight(1.0)
    .caps_round()
    .stroke(color)
    .no_fill()
    // .color(color2)
    .points(pts2)
    ;

    draw.ellipse().color(color).radius(100.0);

  
    // ------------------------------------------------    

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();

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