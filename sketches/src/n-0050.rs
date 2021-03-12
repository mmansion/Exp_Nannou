use nannou::prelude::*;
use std::ops::Range;
use nannou::noise::*;
use nannou::Draw;
use std::collections::VecDeque;
use std::time::Duration;

pub mod lib;

// module tree
use crate::lib::points::Point as Point;
use crate::lib::vehicles::Vehicle as Vehicle;

static CAPTURE  : bool = true; // capture to image sequence
static WIDTH    : f32 = 800.0;
static HEIGHT   : f32 = 800.0; 
static ANGLES   : i32 = 3;

// A Type to manage the CA
struct Ca {
    cells: Vec<i32>,    // An array of 0s and 1s
    generation: i32,    // How many generations?
    rule_set: Vec<i32>, // An array to store the ruleset, for example {0,1,1,0,1,1,0,1}
    scl: i32,
    cells_range: Range<usize>,
}

impl Ca {
    fn new(r: Vec<i32>, rect: Rect) -> Self {
        let rule_set = r;
        let scl = 20;
        let cells = vec![0; ( rect.w() as i32 / scl) as usize];
        let cells_range = 1..cells.len() - 1;
        let generation = 0;

        let mut ca = Ca {
            scl,
            rule_set,
            cells,
            generation,
            cells_range,
        };
        ca.restart();
        ca
    }

    // Set the rules of the CA
    fn _set_rules(&mut self, r: Vec<i32>) {
        self.rule_set = r;
    }

    // Make a random rule set
    fn randomize(&mut self) {
        for i in (0..self.rule_set.len()).rev() {
            self.rule_set[i] = (random_f32() * 2.0).floor() as i32;
        }
    }

    // Reset generation to 0
    fn restart(&mut self) {
        for i in (0..self.rule_set.len()).rev() {
            self.cells[i] = 0;
        }
        let length = self.cells.len();
        self.cells[length / 2] = 1; // We arbitrarily start with just the middle cell having a state of "1"
        self.generation = 0;
    }

    // The process of creating the new generation
    fn generate(&mut self) {
        // First we create an empty array for the new values
        let mut next_gen = vec![0; self.cells.len()];

        // For every spot, determine new state by examing current state, and neighbor states
        // Ignore edges that only have one neighor
        for i in self.cells_range.clone().rev() {
            let left = self.cells[i - 1]; // Left neighbor state
            let me = self.cells[i]; // Current state
            let right = self.cells[i + 1]; // Right beighbor state
            next_gen[i] = self.rules(right, me, left); // Compute next generation state based on ruleset
        }
        // The current generation is the new generation
        self.cells = next_gen;
        self.generation += 1;
    }

    // This is the easy part, just draw the cells fill white if 1, black if 0
    fn display(&self, draw: &Draw, rect: &Rect) {

        for n in (0..8) {
            let draw = draw.rotate(PI/6.0);
            for i in (0..self.cells.len()).rev() {
                let mut fill = rgba(1.0, 0.0, 1.0, 0.1);
                if self.cells[i] == 1 {
                    fill = rgba( 0.0, 0.0, 0.0, 0.0);
                }
                draw
                    .scale(n as f32 * 0.1)
                    .rect()
                    .x_y(
                        ((self.scl / 2) + i as i32 * self.scl) as f32 - rect.right() as f32,
                        rect.top() as f32 - (self.generation * self.scl - (self.scl / 2)) as f32,
                    )
                    .w_h(self.scl as f32, self.scl as f32)
                    //.gray(fill)
                    
                    // .color(GREEN)
                    .color(fill);
                    // .stroke(BLACK);
    
            }
        }
        

    
    }

    // Implementing the Wolfram rules
    // Could be improved and made more concise, but here we can explicitly see what is going on for each case
    fn rules(&self, a: i32, b: i32, c: i32) -> i32 {
        if a == 1 && b == 1 && c == 1 {
            return self.rule_set[0];
        }
        if a == 1 && b == 1 && c == 0 {
            return self.rule_set[1];
        }
        if a == 1 && b == 0 && c == 1 {
            return self.rule_set[2];
        }
        if a == 1 && b == 0 && c == 0 {
            return self.rule_set[3];
        }
        if a == 0 && b == 1 && c == 1 {
            return self.rule_set[4];
        }
        if a == 0 && b == 1 && c == 0 {
            return self.rule_set[5];
        }
        if a == 0 && b == 0 && c == 1 {
            return self.rule_set[6];
        }
        if a == 0 && b == 0 && c == 0 {
            return self.rule_set[7];
        }
        0
    }

    // The CA is done if it reaches the bottom of the screen
    fn finished(&self, rect: &Rect) -> bool {
        if self.generation > rect.h() as i32 / self.scl {
            true
        } else {
            false
        }
    }
}

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    ca     : Ca,
    points : Vec<Vector2>,
    incs   : Vec<f32>,
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
    let mut incs = Vec::new();
    let mut new_frame = false;
    let mut last_calc = Duration::from_millis(0);
    let mut inc = 0.0;
    //----------------------------------
    let mut this_capture_frame = 0;
    let mut last_capture_frame = 0;
    //----------------------------------

    let rule_set = vec![0, 1, 1, 1, 1, 0, 1, 1];
    let ca = Ca::new(rule_set, rect);
    //----------------------------------

    // setup incs
    for i in 0..ANGLES+1 {
        incs.push(i as f32 * random_f32());
    }

    Model {
        this_capture_frame, 
        last_capture_frame, 
        noise, points, incs,
        xOff, yOff, new_frame, last_calc, inc, ca
    }
} 

fn update(app: &App, m: &mut Model, _update: Update) {

    // ref:
    //https://doc.rust-lang.org/nightly/core/time/struct.Duration.html
    //let millis = Duration::from_millis(100).as_millis();

    let since_last_calc = _update.since_start.as_millis() - m.last_calc.as_millis();

    if since_last_calc > 10  {

        m.last_calc = _update.since_start;

        m.inc += 1.5;

        m.new_frame = true;

        for inc in m.incs.iter_mut() {
            *inc += 0.05;
        }

        if m.this_capture_frame != m.last_capture_frame {
            m.last_capture_frame = m. this_capture_frame;
        }
    
        if CAPTURE {
            m.this_capture_frame += 1;
        }
    
        
    } else {
        m.new_frame = false;
    }


    if m.ca.finished(&app.window_rect()) == false {
        m.ca.generate();
    } else {
        m.ca.randomize();
        m.ca.restart();
    }
}

fn view(app: &App, m: &Model, frame: Frame) {

    // get canvas to draw on
    let draw  = app.draw();
    let win   = app.window_rect();
    let t     = app.time;

    let bg = rgba(0.0, 0.0, 0.2, 0.08);

    if app.elapsed_frames() == 1 { 
        draw.background().color(rgba(0.0, 0.0, 0.0, 0.9));
    } else {
        draw.rect().x_y(0.0, 0.0).w_h(win.w()*2.0, win.w()*2.0).color(bg);
    }


    let draw1 = draw.rotate(t *0.5);

    m.ca.display(&draw1, &app.window_rect());
    m.ca.display(&draw1, &app.window_rect());

    let r = 280.0;

    let pts = (0..6).rev().map(|i| {

        let x = (i as f32).cos() * r;
        let y = (i as f32).sin() * r;

        pt2(x, y)

    });

    // draw
    // .polygon()
    // .no_fill()
    // .stroke(rgba(0.0, 1.0, 0.5, 0.9))
    // .stroke_weight(10.0)
    // .points(pts)
    // ;
    // for n in (0..20).rev() { 

    //     let atten = 0.1;
    //     let scale = (n as f32) * atten;
    //     let mut xStore = 0.0;
    //     let mut yStore = 0.0;

    //     let rad_a = 20.0;
    //     let rad_b = 20.0;
    //     let num_cusps = 12.0;


    //     let pts = (0..ANGLES + 1).rev().map(|i| {

    //         let inc =  ( (360 / ANGLES * i) as f32).to_radians();
    //         let ix  = i as usize;
            
    //         let x = ( (num_cusps-1.0) * inc.cos() * rad_b) + (( (num_cusps -1.0) * inc).cos() * rad_a );
    //         //let y = (inc.sin() * rad_b;
    //         let y = ( (num_cusps-1.0) * inc.sin() * rad_b) + (( (num_cusps -1.0) * inc).sin() * rad_b );

    //         let r = 0.4;

    //         let mut xOff = 0.0;
    //         let mut yOff = 0.0;

    
    //         pt2(x + xOff, y + yOff)


    //     });  
        
    //     let hue = app.time * 2.0 * PI;
        
    //     let color = hsla(hue, 0.5, 0.5, 1.0);
    //     //let color = rgba(0.0, 0.0, 0.0, 1.0);
    //     // let draw = draw.rotate( (t.sin() * n as f32) * 0.0001);
    //     let draw = draw.rotate( 3.0*PI/2.0 );
    //     let draw = draw.translate(pt3(-80.0, 0.0, 0.0));

    //     if n == 19 {
    //         draw
    //         .scale(scale)
    //         .polygon()
    //         .color(BLACK)
    //         .stroke(rgba(0.0, 1.0, 0.5, 1.0))
    //         .stroke_weight(3.0 + (0.9 * scale))
    //         .points(pts)
    //         ;
    //     } else if n % 2 == 0 {
    //         draw
    //         .scale(scale)
    //         .polygon()
    //         .color(color)
    //         .no_fill()
    //         .stroke(rgba(0.0, 1.0, 0.5, 1.0))
    //         .stroke_weight(3.0 + (0.9 * scale))
    //         .points(pts)
    //         ;
    //     } else {
    //         draw
    //         .scale(scale)
    //         .polygon()
    //         //.color(BLUE)
    //         .no_fill()
    //         .stroke(rgba(0.0, 0.5, 0.8, 0.5))
    //         .stroke_weight(2.5 + (0.5 * scale))
    //         .points(pts)
    //         ;
    //     }

    // }

 

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();


    // end draw frame ---------------------------------------------------------

    
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