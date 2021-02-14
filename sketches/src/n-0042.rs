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
    let mut new_frame = false;
    let mut last_calc = Duration::from_millis(0);
    let mut inc = 0.0;
    //----------------------------------
    let mut this_capture_frame = 0;
    let mut last_capture_frame = 0;
    //----------------------------------


    Model {this_capture_frame, last_capture_frame, noise, points, xOff, yOff, new_frame, last_calc, inc}
} 

fn update(app: &App, m: &mut Model, _update: Update) {

    
    // ref:
    //https://doc.rust-lang.org/nightly/core/time/struct.Duration.html
    //let millis = Duration::from_millis(100).as_millis();

    let since_last_calc = _update.since_start.as_millis() - m.last_calc.as_millis();

    if since_last_calc > 10  {

        m.last_calc = _update.since_start;

        m.inc += 0.01;

        m.new_frame = true;

        if !m.points.is_empty() {
            m.points.clear();// clears the vector, removing all values
        }
    
        let w = WIDTH as f32;
        let h = HEIGHT as f32;
        //m.points.push(pt2(-25.0, -h));
        // increment by a custom step value
        // see: iter step by method
        // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.step_by
        
        let frac = (app.elapsed_frames() % 200) as f32 / (200.0);
    
    
        for i in (1..WIDTH+50).step_by(20) {

            let mut x = i as f32;
            
            //let x = (i as f32).sin() * 100.0;
    
            let n = m.noise.get([m.xOff, m.yOff]) as f32;
    
            // let n = map_range( m.noise.get(
            //     [
            //         m.xOff, 
            //         m.yOff,
            //         // rotcos as f64,
            //         // rotsin as f64
            //     ]), 0., 1., 0, 1) as f32;
    
            let y = x.sin() * 10.0 * n;
    
            m.xOff += 0.0001;

            // let rotsin = (frac * y).sin() * 100.0;
            // let rotcos = (frac * x).cos() * 50.0;
            let rotsin = (m.yOff as f32).sin() * 100.0;
            let rotcos = (m.xOff as f32).cos() * 50.0;
            // let rotcos = (frac * TAU).cos();
            // let rotsin = (frac * TAU).sin();

            let x2 = (x).cos() * 20.0;
            let y2 = (y).sin() * 20.0;
        
            m.points.push( pt2(x+x2, y + rotsin + y2) );
        }
        
        m.yOff += 0.0001;
        //m.points.push(pt2(w+25.0, -h));

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
    let draw = app.draw();
    let win = app.window_rect();
    let t = app.time;

    if m.new_frame  {

    let bg = rgba(0.0, 0.0, 0.0, 0.001);
    let color = hsva(t.sin() * 0.9, 1.0, 1.0, 1.0);
    
    if m.inc < 0.1 {
        draw.background().color(BLACK);
    } else {
        //background
        draw.rect().x_y(0.0, 0.0).w_h(win.w()*2.0, win.w()*2.0).color(bg)
        ;
    }
   
    // ------------------------------------------------ 
    let pts = (0..m.points.len()).map(|i| {
        (pt2(m.points[i].x, m.points[i].y))
    });    

    let pts2 = (0..m.points.len()).map(|i| {
        (pt2(m.points[i].x, m.points[i].y))
    });  

    // ------------------------------------------------ 

    let draw = draw.rotate(t * 0.1);

    let draw1 = draw.translate( 
        pt3(
            -win.w()/2.0-25.0 + (m.inc*10.0).sin() * win.h()/40.0, 
            -(m.inc * 1.1).sin() * win.h()/3.0, 
            //0.0,
            0.0) );

    // ------------------------------------------------ 

    // let draw2 = draw.rotate(t * -0.1);

    let draw2 = draw.translate( 
        pt3(
            -win.w()/2.0-25.0 + (m.inc*10.0).sin() * win.h()/40.0, 
            (m.inc * 1.1).sin() * win.h()/3.0, 
            //0.0,
            0.0) );

    // ------------------------------------------------ 

    draw1
    .polyline()
    .stroke_weight(10.0)
    // .caps_round()
    .color(color)
    // .no_fill()
    // .color(color2)
    .points(pts)
    ;

    draw2
    .polyline()
    .stroke_weight(10.0)
    // .caps_round()
    .color(color)
    // .no_fill()
    // .color(color2)
    .points(pts2)
    ;

    // ------------------------------------------------ 
    let pts3 = (0..360).map(|i| {
        let x = (i as f32).sin() * 500.0;
        let y = (i as f32).cos() * 500.0;

        ( pt2(x, y) )
    });  
    
    draw
    .polyline()
    .stroke_weight(100.0)
    // .caps_round()
    .color(BLACK)
    // .no_fill()
    // .color(color2)
    .points(pts3)
    ;

    // draw
    // .rect()
    // .w_h(win.w(), 400.0)
    // .x_y(0.0, 400.0)
    // .color(rgb(0.0, 0.0, 0.2))
    // ;
    // draw
    // .rect()
    // .w_h(win.w(), 400.0)
    // .x_y(0.0, -400.0)
    // .color(rgb(0.0, 0.0, 0.2))
    // ;

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

    

    
}