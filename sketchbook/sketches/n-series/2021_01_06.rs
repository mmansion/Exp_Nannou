use nannou::prelude::*;

// Carry Over Notes: 

// [] upgrade and learn ab frame cap -> https://nannou.cc/posts/nannou_v0.13

static _WIDTH_  : f32  = 800.0;
static _HEIGHT_ : f32  = 800.0;

fn main() {

    nannou::app(model).update(update).run();
}

// -------------------------------------------------------------------

// A simple particle type
struct Particle {
    position: Point2,
    velocity: Vector2,
    acceleration: Vector2,
    life_span: f32,
    rad: f32,
    inc: f32,
}

impl Particle {
    fn new(l: Point2) -> Self {
        let acceleration = vec2(0.0, 0.05);
        let velocity = vec2(random_f32() * 3.0 - 1.0, random_f32() - 1.0);
        let position = l;
        let life_span = 800.0;
        let rad = 13.0;
        let inc = 0.0;
        Particle {
            acceleration,
            velocity,
            position,
            life_span,
            rad,
            inc,
        }
    }

    // Method to update position
    fn update(&mut self) {

        let x = self.inc.cos() * self.rad;
        let y = self.inc.sin() * self.rad;
        let vecOffset = vec2(x, y);

        self.velocity += self.acceleration;
        self.position -= self.velocity;
        self.position += vecOffset;

        self.life_span -= 1.0;
        self.inc+=0.2;
    }

    // Method to display
    fn display(&self, draw: &Draw) {

        let size = self.life_span / 255.0 * 10.0;

        // Do the same, but give each point a unique colour.
        let n_points = 7;
        let points_colored = (0..n_points).map(|i| {
            let fract = i as f32 / n_points as f32;
            let phase = fract;
            let x = size * (TAU * phase).cos();
            let y = size * (TAU * phase).sin();
            let r = fract;
            let g = 1.0 - fract;
            let b = (0.5 + fract) % 2.0;
            (pt2(x, y), rgb(r, g, b))
        });
        draw.polygon()
            .xy(self.position)
            .rotate(self.inc * 0.2)
            .points_colored(points_colored);

        // let size = self.life_span / 255.0 * 10.0;
   
        let r = 255.0 / self.life_span;
        let g = 255.0 / self.life_span;
        let b = 255.0 / self.life_span;
        let col = srgba(r, g, b, 1.0);

          

        draw.rect()
            .xy(self.position)
            .w_h(size, size)
            .rgba(1.0, 0.0, 0.0, 0.1)
            .stroke(col)
            .color(rgba(0.0, 0.0, 0.0, self.life_span / 255.0))
            //.stroke( rgba(0.0, 0.0, 0.0, self.life_span / 255.0))
            .stroke_weight(1.0);
            ;
    }

    // Is the poarticel still useful?
    fn is_dead(&self) -> bool {
        if self.life_span < 0.0 {
            true
        } else {
            false
        }
    }
}

// -------------------------------------------------------------------

struct ParticleSystem {
    particles: Vec<Particle>,
    origin: Point2,
}

impl ParticleSystem {
    fn new(position: Point2) -> Self {
        let origin = position;
        let particles = Vec::new();
        ParticleSystem { origin, particles }
    }

    fn add_particle(&mut self) {
        self.particles.push(Particle::new(self.origin));
    }

    fn update(&mut self) {
        for i in (0..self.particles.len()).rev() {
            self.particles[i].update();
            if self.particles[i].is_dead() {
                self.particles.remove(i);
            }
        }
    }

    fn draw(&self, draw: &Draw) {
        for p in self.particles.iter() {
            p.display(&draw);
        }
    }
}

struct Model {
    inc : f32,
    rad : f32,
    ps  : ParticleSystem,
    clear: bool,
}

fn model(app: &App) -> Model {

    let rect = Rect::from_w_h( _WIDTH_, _HEIGHT_ );

    app.new_window()
        .size(rect.w() as u32, rect.h() as u32)
        .view(view)
        .build()
        .unwrap();


    let ps = ParticleSystem::new( pt2( 0.0, 0.0) );

    let mut inc = 0.0;
    let rad = 300.0;

    let clear = false;
    
    Model { ps, inc, rad, clear }

}    

// do calculations here 
/*
have a &mut Model in update: that's where you can mutate your data. 
You can't do that in view, because it's only a reference, not a mutable one. 
This is a design choice from nannou where you can't mutate things when you are drawing them. 
Coming from processing it might be hard to adapt to this choice, but it makes things clearer.
*/
fn update(app: &App, m: &mut Model, _update: Update) {

    //increment inc
    m.inc = m.inc + 0.02;

    m.ps.add_particle();
    m.ps.update();

    if(m.inc > 1.0) {
        m.clear = true;
    }
   
}

// draw outputs here
fn view(app: &App, m: &Model, frame: Frame) {

    let win = app.window_rect();
    
    //println!("The window is {} x {}", win.w(), win.h());

    let t = app.time;

    // Begin drawing
    let draw = app.draw();

    // let rotate = (app.time).sin() * (m.inc).cos();
    // let rotate =
    let draw = draw.rotate(m.inc);

    // clear the bg
    let mut bg_col = rgba(0.0, 0.0, 0.0, 0.01);
    let mut fg_col = rgba(1.0, 1.0, 1.0, 0.1);
    
    if(!m.clear) {
        draw.background().color(BLACK);
    }

    //background
    draw.rect()
        .x_y(0.0, 0.0)
        .w_h(win.w()*2.0, win.w()*2.0)
        .color(bg_col)
        ;

     // --------------------------------------
    
    let x = m.inc.cos() * m.rad;
    let y = m.inc.sin() * m.rad;

    let p = pt2(x, y);

    // ------------------------------------------

    m.ps.draw(&draw);
    // ------------------------------------------

    //println!("{}", m.mover.x);


     // --------------------------------------


    // ------------------------------------------
    let circle_resolution = 12;
    let angle = TAU / circle_resolution as f32;

    //draw.background().color(BLACK);

    for i in 0..circle_resolution {
        let x = (angle * i as f32).cos() * m.rad*2.0;
        let y = (angle * i as f32).sin() * m.rad*2.0;
        draw.line()
            .start(pt2(0.0, 0.0))
            .end(pt2(x, y))
            .stroke_weight(1.0)
            .caps_round()
            .color(BLACK);
    }
    
    
    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();

    // capture
    // if app.keys.down.contains(&Key::S) {
    //     app.main_window()
    //         .capture_frame(app.exe_name().unwrap() + ".png");
    // }
}
