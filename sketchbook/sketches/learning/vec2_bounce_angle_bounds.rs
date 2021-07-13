/*
* vec2_bounce_angle
*
* bounce a moving object at a specified angle by changing its velocity
*
* mikhail mansion 2021
*/

use nannou::prelude::*;
use nannou::ui::prelude::*;
use std::ops::Range;
use nannou::Draw;
use std::time::Duration;

//--------------------------------------------------------
static WIDTH      : u32 = 800;
static HEIGHT     : u32 = 800; 
static MARGIN     : f32 = 10.0;
static SIZE       : f32 = 50.0;

fn main() {
    // nannou::app(model).update(update).simple_window(view).run();
    nannou::app(model).update(update).run();
}

struct Mover {
    orig : Vec2,
    pos  : Vec2,
    vel  : Vec2,
    max_speed : f32,
    size : f32
}

impl Mover {
    pub fn new(_pos:Vec2, _size:f32) -> Self {
        let max_speed = 10.0;
        let orig = _pos;
        let pos  = _pos;
        let size = _size;
        let vel  = pt2(random_range(-max_speed, max_speed), random_range(-max_speed, max_speed));
        //let vel  = pt2(-10.0, 0.0);
        Mover {
            orig,
            pos,
            vel,
            max_speed,
            size
        }
    }

    pub fn update(&mut self) {
        self.pos += self.vel;
    }

    pub fn get_normal(&self, p1:Vec2, p2:Vec2) -> Vec2 {
        // A unit normal vector to a two-dimensional curve is 
        // a vector with magnitude 1 that is perpendicular to the curve at some point.

        // calculate 2d normal of line (perpendicular vector)
        let diff_x = p2.x - p1.x;
        let diff_y = p2.y - p1.y;
        let normal = vec2(-diff_y, diff_x);
        let clampled_normal = normal.clamp_length_max(self.max_speed);

        return clampled_normal;
    }

    pub fn check_bounds(&mut self, win_w:f32, win_h:f32) {

        if self.pos.y > win_h/2.0  { //past top edge
            self.pos.y = win_h/2.0 - (self.size/2.0);
            self.vel.y *= -1.0;
        } else 

        if self.pos.y < -win_h/2.0 { // past bottom edge
            self.pos.y = -win_h/2.0 + (self.size/2.0);
            self.vel.y *= -1.0;
        } else 
    
        if self.pos.x < -win_w/2.0 { //past left edge
            self.pos.x = -win_w/2.0 + (self.size/2.0);
            self.vel.x *= -1.0;
            
        } else 

        if self.pos.x > win_w/2.0 { //past right edge
            self.pos.x = win_w/2.0 - (self.size/2.0);
            self.vel.x *= -1.0;
        }
    }

    pub fn display(&self, draw: &Draw) {
        // draw.arrow().weight(5.0).color(BLUE).points(self.orig, self.pos);
        draw
        .ellipse()
        .xy(self.pos)
        .stroke(BLUE)
        // .color(BLUE)
        .stroke_weight(5.0)
        .w_h(self.size, self.size)
        ;
    }
}

//--------------------------------------------------------

struct Model {
    movers  : Vec<Mover>,
}

//--------------------------------------------------------

fn model(app: &App) -> Model {

    app.new_window().size(800, 800)
        .event(event)
        .view(view)
        .build()
        .unwrap()
        ;

    let w = app.window_rect().w();
    let h = app.window_rect().h();

    let mut movers = Vec::new();

    Model {
        movers,
    }
}

//--------------------------------------------------------

fn update(_app: &App, m: &mut Model, _update: Update) {

    let w = _app.window_rect().w();
    let h = _app.window_rect().h();

    // for m in 0..m.movers.len() { {
    //     m.update();

    // }
    for mover in m.movers.iter_mut() {
        mover.check_bounds(w, h);
        mover.update();
    }

    // model.mover.check_bounds(w, h);
    //update the mover
    // model.mover.update();
}

//--------------------------------------------------------

// Draw the state of your `Model` into the given `Frame` here.
fn view(app: &App, m: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    draw.background().color(WHITE);

    for mover in m.movers.iter() {
        mover.display(&draw);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn event(app: &App, m: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(key) => {
            if let Key::Space = key {
                let half_w = app.window_rect().w()/2.0;
                let half_h = app.window_rect().h()/2.0;

                println!("new mover");
                let pos = vec2( 
                    random_range(-half_w+MARGIN, half_w-MARGIN), 
                    random_range(-half_h+MARGIN, half_h-MARGIN)
                );

                m.movers.push( Mover::new(pos, SIZE) );
            }
        }
        MousePressed(button) => {
            // println!("global scope: GLOBAL = {}", GLOBAL);
        }
        _other => (),
    }
}
