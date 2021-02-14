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

    let bg = rgba(0.0, 0.0, 0.2, 1.0);
    let color = hsva(t.sin() * 0.9, 1.0, 1.0, 1.0);
    
    if m.inc < 0.01 {
        draw.background().color(BLACK);
    } else {
        //background
        draw.rect().x_y(0.0, 0.0).w_h(win.w()*2.0, win.w()*2.0).color(bg);
    }

    for n in 0..20 {

       

        let angles = 12;
        let atten = 0.3;
        let scale = (n as f32) * atten;
        let mut xStore = 0.0;
        let mut yStore = 0.0;

        let points_colored = (0..angles + 1).map(|i| {
            let inc =  ( (360 / angles * i) as f32).to_radians();
            
            let x = inc.cos() * 100.0; 
            let y = inc.sin() * 100.0;

            let r = 10.0;
            // let mut xOff = 0.0;
            // let mut yOff = 0.0;

            let mut xOff = x.cos() * (random_f32() * r); 
            let mut yOff = y.sin() * (random_f32() * r);


            if i == 0 {
                xStore = xOff;
                yStore = yOff;
            } 

            if i == angles {
                xOff = xStore;
                yOff = yStore;
            }

            let color = hsva( (t.sin() * x) * 0.01, 1.0, 1.0, 1.0);
            

            // let n = (m.noise.get([x as f64, y as f64]) * 10.0) as f32;

        
            (pt2(x + xOff, y + yOff), color)
        });  
        
        let color = hsva( t.sin() * 0.01, 1.0, 1.0, 1.0);
        let draw = draw.rotate( (t.sin() * n as f32) * 0.01);
        let draw = draw.rotate( t );

        draw
        .scale(scale)
        .polygon()
        .no_fill()
        .stroke(color)
        .stroke_weight( n as f32 * 0.5)
        .points_colored(points_colored)
        ;

        

    }
   
    


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