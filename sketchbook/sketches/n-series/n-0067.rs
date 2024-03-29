/*
* name
*
* description of app
*
* mikhail mansiocarn YYYY
*/

use nannou::prelude::*;
use nannou::geom::*;
use nannou::geom::Point2;
use std::ops::Range;
use nannou::Draw;
use std::time::Duration;

use library::grid;

//--------------------------------------------------------
static CAPTURE  : bool = true; // capture to image sequence

static WIDTH    : f32 = 800.0;
static HEIGHT   : f32 = 800.0; 
static MARGIN   : f32 = 300.0;
static SIZE     : f32 = 10.0;

//--------------------------------------------------------

struct FocalLine {

    //private
    A   : Vec2,
    B   : Vec2,
    rotation: f32,
    curr_len:f32,
    time_offset : f32,
    stroke_weight:f32,
    color:Rgba,
    //public
    pub pos : Vec2,

}

impl FocalLine {

    fn new(origin:Vec2, length: f32) -> Self {
        // let mut A = p1;
        // let mut B = p2;

        let pos = origin;
        let A = vec2(origin.x - length/2.0, origin.y);
        let B = vec2(origin.x + length/2.0, origin.y);

        let rotation = 0.0;
        let time_offset = 0.0;
        let stroke_weight = 2.0;

        let curr_len = length;

        let color =     

        FocalLine {
            A,
            B,
            pos,
            rotation,
            curr_len,
            time_offset,
            stroke_weight,
            color
        }
    }

    pub fn rotate(&mut self, degree:f32) {
        self.rotation = degree;
    }

    pub fn update(&mut self, app:&App) {

        if(self.time_offset == 0.0) {
            self.time_offset = app.time * random_f32() * 1000.0;
        }

        let t = (app.time + self.time_offset).sin() * 2.1;
        // self.rotation = app.time * 0.1;
        self.stroke_weight = abs(app.time.sin() * 2.0 + self.curr_len * 0.2);

        self.curr_len = map_range(t, 0.0, 1.0, 10.0, 100.0);

       // self.color = hsva(app.time.sin(), app.time.sin(), 1.0, 1.0);
        //self.color = hsla(1.0, 1.0, 1.0, 1.0);
        //self.color = hsva(1.0, 0.7, 1.0, 1.0);
        self.color = rgba(1.0, 0.0, 0.0, 1.0);

        self.A = vec2(self.pos.x - self.curr_len/2.0, self.pos.y);
        self.B = vec2(self.pos.x + self.curr_len/2.0, self.pos.y);
    }

    pub fn display(&self, draw:&Draw) {
 
        let points = [
            self.A, self.B
        ];

        draw
        .rotate(self.rotation)
        .polyline()
        .stroke_weight(self.stroke_weight)
        .caps_butt()
        .color(self.color)
        .points(points)
        ;
    }

    pub fn get_midpoint(&self) -> Vec2 {
        return vec2( (self.A.x + self.B.x) / 2.0, (self.A.y + self.B.y) / 2.0 );
    }
}

//--------------------------------------------------------
fn main() {
    nannou::app(model).update(update).run();
}

//--------------------------------------------------------
struct Model {
    this_capture_frame : i32,
    last_capture_frame : i32,
    last_calc : Duration,
    lines: Vec<FocalLine>,
}

//--------------------------------------------------------
fn model(app: &App) -> Model {
    
    // app.set_loop_mode(LoopMode::loop_once());
    // app.set_loop_mode(LoopMode::rate_fps(0.1));
    
    app.new_window()
        .size(WIDTH as u32, HEIGHT as u32)
        //.fullscreen()
        .view(view)
        .build()
        .unwrap();

    let mut last_calc = Duration::from_millis(0);

    //--------------------------------------------------------
    let mut this_capture_frame = 0;
    let mut last_capture_frame = 0;

    let mut lines = Vec::new();

    //--------------------------------------------------------

    for i in 0..100 {
        // println!("{}", i);
        lines.push( 
            FocalLine::new( vec2( 
                random_f32() * WIDTH - (WIDTH/2.0), 
                random_f32() * HEIGHT - (HEIGHT/2.0)
            ), 100.0,)
        );
    }

    //--------------------------------------------------------

    Model {
        this_capture_frame, 
        last_capture_frame, 
        last_calc,
        lines,
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

    for line in m.lines.iter_mut() {

        line.update(&app);   
    }

}

fn view(app: &App, m: &Model, frame: Frame) {

    // get canvas to draw on
    let draw  = app.draw();
    let win   = app.window_rect();
    let time  = app.time;

    //--------------------------------------------------------
    // background

    let bg = rgba(1.0, 0.0, 0.0, 1.0);

    if app.elapsed_frames() == 1 { 
        draw.background().color(rgba(0.0, 0.0, 0.0, 0.2));
    } else {
        draw.rect().x_y(0.0, 0.0).w_h(win.w()*2.0, win.w()*2.0).color(bg);
    }

    for i in 0..400 {
        let x = -WIDTH/2.0;
        let y = HEIGHT - 10.0 * i as f32 ;
        let points = [vec2(x, y), vec2(x+WIDTH, y)];

        draw
        .polyline()
        .stroke_weight(3.0)
        .caps_round()
        .color(WHITE)
        .points(points)
        ;

    }
    for line in m.lines.iter() {

        line.display(&draw);   
    }

    

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