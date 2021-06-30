/*
* rain-bow-1
* creative coding application for rain bow sculpture
* mikhail mansion 2021
*/

use nannou::prelude::*;
use nannou::geom::*;
use nannou::geom::Point2;
use std::ops::Range;
use nannou::Draw;
use std::time::Duration;

use library::particle::Particle;
use library::math::intersects_line;
use library::line::Line;

//--------------------------------------------------------
static CAPTURE  : bool = false; // capture to image sequence
static WIDTH    : f32 = 800.0;
static HEIGHT   : f32 = 800.0; 

//--------------------------------------------------------
fn main() { nannou::app(model).update(update).run() }

// representation of physical v-shaped bow
struct VBow {
    left_point  : Vector2,
    cent_point  : Vector2,
    right_point : Vector2,
    left_line   : Line,
    right_line  : Line,
}
//--------------------------------------------------------
impl VBow {

    fn new(l_pt:Vector2, c_pt:Vector2, r_pt:Vector2) -> Self {

            let mut left_point  = l_pt;
            let mut cent_point  = c_pt;
            let mut right_point = r_pt;

            // represent 2 lines from points on the v-bow`
            let left_line = Line::new(left_point, cent_point);
            let right_line = Line::new(cent_point, right_point);

        VBow {
            left_point,
            cent_point,
            right_point,
            left_line,
            right_line,
        }
    }

    fn update(&mut self, x:f32, y:f32) {
        // let x = x;
        self.cent_point.x = x;
        self.cent_point.y = y;

        self.left_line.update_points(self.left_point, self.cent_point);
        self.right_line.update_points(self.cent_point, self.right_point);

        // self.cent_point = p;
        // self.cent_point.y = y;
    }

    fn display(&self, draw: &Draw) {

        let points = [
            self.left_point, self.cent_point, self.right_point
        ];
        draw
        .scale(1.0)
        .polyline()
        .weight(2.0)
        .color(rgba(1.0, 1.0, 1.0, 1.0))
        .points(points)
        ;
    }
}
//--------------------------------------------------------

struct Model {
    this_capture_frame : i32,
    last_capture_frame : i32,
    last_calc : Duration,
    particles : Vec<Particle>,
    vbow : VBow,
}

//--------------------------------------------------------
fn model(app: &App) -> Model {

    let rect = Rect::from_w_h( WIDTH, HEIGHT );
    
    // app.set_loop_mode(LoopMode::loop_once());
    // app.set_loop_mode(LoopMode::rate_fps(0.1));
    
    app.new_window()
        .mouse_pressed(mouse_pressed)
        .size(800, 800)
        .view(view)
        .build()
        .unwrap();

    let mut last_calc = Duration::from_millis(0);

    //--------------------------------------------------------
    let mut this_capture_frame = 0;
    let mut last_capture_frame = 0;

    let mut particles = Vec::new();

    let p1 = pt2( -WIDTH/2.0, 0.0 );
    let p2 = pt2( 0.0, 0.0 );
    let p3 = pt2( WIDTH/2.0, 0.0);

    let mut vbow = VBow::new(p1, p2, p3);

    //--------------------------------------------------------

    Model {
        this_capture_frame, 
        last_capture_frame, 
        last_calc,
        particles,
        vbow
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

    //----------------------------------------------------------

    //update the vbow and its line positions
    m.vbow.update(app.mouse.x, app.mouse.y);

    for i in 0..m.particles.len() {

        let wind = vec2(0.01, 0.0);
        let gravity = vec2(0.0, -0.1 * m.particles[i].mass);

        m.particles[i].apply_force(wind);
        m.particles[i].apply_force(gravity);
        m.particles[i].update();
        m.particles[i].check_edges(app.window_rect());

        //line 1
        let orig_pt = m.particles[i].origin;
        let pos_pt  = m.particles[i].position;


        m.particles[i].check_line_bounds(&m.vbow.left_line);
        m.particles[i].check_line_bounds(&m.vbow.right_line);
        
        
        m.particles[i].check_edges(app.window_rect());
        
    }

    

    // println!("{}", intersects_line());

    //----------------------------------------------------------
}

fn view(app: &App, m: &Model, frame: Frame) {

    // get canvas to draw on
    let draw  = app.draw();
    let win   = app.window_rect();
    let time  = app.time;

    //--------------------------------------------------------
    // background

    let bg = rgba(0.0, 0.0, 0.0, 1.0);

    if app.elapsed_frames() == 1 { 
        draw.background().color(rgba(0.0, 0.0, 0.0, 0.9));
    } else {
        draw.rect().x_y(0.0, 0.0).w_h(win.w()*2.0, win.w()*2.0).color(bg);
    }
    
    //--------------------------------------------------------
    for particle in &m.particles {
        particle.display(&draw);
        // particle.display_line(&draw);
    }

   
    m.vbow.display(&draw);

    
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

fn mouse_pressed(app: &App, m: &mut Model, b: MouseButton) {

    let last_ix = m.particles.len() as usize;

    m.particles.push(Particle::new(app.mouse.x, app.mouse.y));

    m.particles[last_ix].display_size = 20.0;

    // println!("{}", m.particles[last_ix].display_size);
    

    // let l = _model.particles.len();
    // p.display_size = 100.0;
    // _model.particles.last_mut().display_size = 100.0;
    // let _p = &_model.particles.last();

    // println!( "{}",  _p.display_size);
}