use nannou::prelude::Point2;
use nannou::prelude::*;
// use std::ops::Range;
use nannou::noise::*;
// use nannou::Draw;
// use std::collections::VecDeque;
use std::time::Duration;

//pub mod lib;

// module tree
// use crate::lib::points::Point as Point;
// use crate::lib::vehicles::Vehicle as Vehicle;

static CAPTURE: bool = true; // capture to image sequence
static WIDTH: f32 = 800.0;
static HEIGHT: f32 = 800.0;

fn main() {
    nannou::app(model).update(update).run();
}

// A simple particle type
#[derive(Clone)]
struct Particle {
    position: Point2,
    velocity: Vector2,
    acceleration: Vector2,
    life_span: f32,
    r: f32,
    highlight: bool,
    idx: u64,
}

impl Particle {
    fn new(l: Point2, idx: u64) -> Self {
        let acceleration = vec2(0.0, 0.1);

        let velocity = vec2(random_range(-5.0, 5.0), random_range(-5.0, 5.0));
        let position = l;
        let life_span = 255.0;
        Particle {
            acceleration,
            velocity,
            position,
            life_span,
            r: 6.0,
            highlight: false,
            idx,
        }
    }

    fn intersects(&mut self, particles: &Vec<Particle>) {
        self.highlight = false;
        for i in 0..particles.len() {
            if particles[i].idx != self.idx {
                let d = particles[i].position.distance(self.position);
                if d < self.r + particles[i].r {
                    self.highlight = true;
                }
            }
        }
    }

    fn _apply_force(&mut self, f: Vector2) {
        self.acceleration += f;
    }

    // Method to update position
    fn update(&mut self) {
        // self._apply_force(pt2(2.0, 1.2));
        self.velocity += self.acceleration;
        self.position -= self.velocity;
        self.acceleration *= 0.0;
        self.life_span -= 2.0;
    }

    // Method to display
    fn display(&self, draw: &Draw) {
        let c = if self.highlight {
            rgba(0.5, 0.0, 0.0, 1.0)
        } else {
            rgba(0.5, 0.5, 0.5, self.life_span / 255.0)
        };

        let r = 20.0;

        let pts = (0..6).rev().map(|i| {
            let x = (i as f32).cos() * r;
            let y = (i as f32).sin() * r;

            pt2(x, y)
        });

        draw.polygon()
            .xy(self.position)
            .no_fill()
            .stroke(rgba(0.0, 1.0, 0.5, 0.9))
            .stroke_weight(1.0)
            .points(pts);

        // draw.ellipse()
        //     .xy(self.position)
        //     .radius(self.r)
        //     .color(c)
        //     .stroke(rgba(0.0, 0.0, 0.0, self.life_span / 255.0))
        //     .stroke_weight(2.0);
    }

    // Is the particle still useful?
    fn is_dead(&self) -> bool {
        if self.life_span < 0.0 {
            true
        } else {
            false
        }
    }
}

struct ParticleSystem {
    particles: Vec<Particle>,
    pub origin: Point2,
}

impl ParticleSystem {
    fn new(position: Point2) -> Self {
        let origin = position;
        let particles = Vec::new();
        ParticleSystem { origin, particles }
    }

    fn add_particle(&mut self, frame_num: u64) {
        self.particles.push(Particle::new(self.origin, frame_num));
    }

    fn update(&mut self) {
        for i in (0..self.particles.len()).rev() {
            self.particles[i].update();
            if self.particles[i].is_dead() {
                self.particles.remove(i);
            }
        }
    }

    fn intersection(&mut self) {
        let particles = self.particles.clone();
        for i in 0..self.particles.len() {
            self.particles[i].intersects(&particles);
        }
    }

    fn draw(&self, draw: &Draw) {
        for p in self.particles.iter() {
            p.display(&draw);
        }
    }
}

struct Model {
    noise: Perlin,
    ps: ParticleSystem,
    this_capture_frame: i32,
    last_capture_frame: i32,
    last_calc: Duration,
}

// ----------------------------------------------------------------------
fn model(app: &App) -> Model {
    let rect = Rect::from_w_h(WIDTH, HEIGHT);

    // app.set_loop_mode(LoopMode::loop_once());
    // app.set_loop_mode(LoopMode::rate_fps(0.1));

    app.new_window().size(800, 800).view(view).build().unwrap();

    let mut noise = Perlin::new();
    noise = noise.set_seed(1);
    let mut last_calc = Duration::from_millis(0);

    //----------------------------------
    let mut this_capture_frame = 0;
    let mut last_capture_frame = 0;

    //----------------------------------

    let ps = ParticleSystem::new(pt2(0.0, 0.0));

    //----------------------------------

    //----------------------------------

    Model {
        this_capture_frame,
        last_capture_frame,
        last_calc,
        noise,
        ps,
    }
}

fn update(app: &App, m: &mut Model, _update: Update) {
    // ref:
    //https://doc.rust-lang.org/nightly/core/time/struct.Duration.html
    //let millis = Duration::from_millis(100).as_millis();
    let since_last_calc = _update.since_start.as_millis() - m.last_calc.as_millis();
    if since_last_calc > 10 {
        // timed interval
    }

    // m.ps.origin = pt2(app.mouse.x, app.mouse.y);

    m.ps.origin = pt2(0.0, 0.0);
    m.ps.add_particle(app.elapsed_frames());
    m.ps.update();
    m.ps.intersection();

    if m.this_capture_frame != m.last_capture_frame {
        m.last_capture_frame = m.this_capture_frame;
    }

    if CAPTURE {
        m.this_capture_frame += 1;
    }
}

fn view(app: &App, m: &Model, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();
    let win = app.window_rect();
    let t = app.time;

    // draw background----------------------------------------------------------

    let bg = rgba(0.0, 0.0, 0.2, 0.08);

    if app.elapsed_frames() == 1 {
        draw.background().color(rgba(0.0, 0.0, 0.0, 0.1));
    } else {
        draw.rect()
            .x_y(0.0, 0.0)
            .w_h(win.w() * 2.0, win.w() * 2.0)
            .color(bg);
    }

    // transform --------------------------------------------------------------

    let draw = draw.rotate(t * 0.9);

    // particle system --------------------------------------------------------

    m.ps.draw(&draw);

    // draw thigns -------------------------------------------------------------

    let color = rgb(0.0, 0.0, 0.2);
    draw
    .ellipse()
    .color(color)
    .radius(80.0)
    // .color(bg)
    ;

    let r = 500.0;
    let pts = (0..360).rev().map(|i| {
        let x = (i as f32).cos() * r;
        let y = (i as f32).sin() * r;

        pt2(x, y)
    });

    draw.polygon()
        .no_fill()
        .stroke(color)
        .stroke_weight(200.0)
        .points(pts);

    // draw frame -------------------------------------------------------------

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();

    // capture fram -----------------------------------------------------------

    if m.this_capture_frame != m.last_capture_frame {
        // let mut owned_string: String = "hello ".to_owned();
        // let borrowed_string: String = "output/" + app.exe_name().unwrap() + ".png";

        let directory = "captures/".to_string();
        let app_name = app.exe_name().unwrap().to_string();
        // let frame_num  = m.this_capture_frame.to_string();
        let extension = ".png".to_string();

        let frame_num = format!("{:05}", m.this_capture_frame);

        let path = format!("{}{}{}", directory, frame_num, extension);

        app.main_window().capture_frame(path);
    }
}
