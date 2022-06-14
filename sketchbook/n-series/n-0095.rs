/// See these two great guides!
///
/// https://generativeartistry.com/tutorials/circle-packing/
/// https://guide.nannou.cc/tutorials/draw/drawing-2d-shapes.html  
use nannou::prelude::*;
use nannou_touchosc::TouchOscClient;
use std::time::Duration;

const PORT: u16 = 6555;

// CONSTANTS
//--------------------------------------------------------
static CAPTURE: bool = false; // capture to image sequence (or use obs)
static FRAME: bool = true; //hide window chrome when set to false
static WIDTH: f32 = 800.0;
static HEIGHT: f32 = 800.0;
static BORDER: f32 = 10.0;

const LINE_WIDTH: f32 = 1.0;
const MIN_RADIUS: f32 = 1.0;
const MAX_RADIUS: f32 = 200.0;
const MIN_BREATH_RADIUS: f32 = 50.0;
const N_CIRCLES: usize = 1000;
const CREATE_CIRCLE_ATTEMPTS: usize = 500;

//--------------------------------------------------------
struct BreathingCircle {
    x: f32,
    y: f32,
    weight: f32,
    radius: f32,
    color: Hsva,
    inc: f32
}
impl BreathingCircle {

    fn update(&mut self, speed:f32) {
        self.inc = self.inc + speed;
    }

    fn draw(&self, draw:&Draw, offset:f32) {
        let diam = (offset + self.inc).sin() * (self.radius * 2.0);
        draw.ellipse()
            .x_y(self.x, self.y)
            .w_h(diam, diam)
            .stroke_weight(self.weight)
            .color(self.color)
            ;
    }
}
//--------------------------------------------------------
struct Circle {
    x: f32,
    y: f32,
    weight: f32,
    radius: f32,
    color: Hsva,
}

impl Circle {
    fn collides(&self, other: &Circle) -> bool {
        let a = self.radius + other.radius;
        let x = self.x - other.x;
        let y = self.y - other.y;

        if a >= ((x * x) + (y * y)).sqrt() {
            true
        } else {
            false
        }
    }

    fn any_collision(&self, others: &Vec<Circle>) -> bool {
        for other in others {
            if self.collides(other) {
                return true;
            }
        }
        false
    }
}

//--------------------------------------------------------

struct Model {
    touchosc: TouchOscClient,
    circles: Vec<Circle>,
    breathing_circles: Vec<BreathingCircle>
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    // app.set_loop_mode(LoopMode::loop_once());

    app.new_window()
        .size(WIDTH as u32, HEIGHT as u32)
        .decorations(FRAME) //creates a borderless window
        .view(view)
        .build()
        .unwrap();
    // To initialize a TouchOscClient you instantiate it as "mutable"
    // and pass the OSC port where messages will be received.
    let mut touchosc = TouchOscClient::new(6555);

    touchosc.verbose(); //enable debugging

    // Adding touchosc client inputs.
    touchosc.add_button("/redraw", false);

    let mut circles = Vec::<Circle>::with_capacity(N_CIRCLES);
    let mut breathing_circles = Vec::<BreathingCircle>::with_capacity(N_CIRCLES);

    //--------------------------------------------------------

    Model {
        touchosc,
        circles,
        breathing_circles
    }
}

fn update(app: &App, m: &mut Model, _update: Update) {

    m.touchosc.update();
    if m.breathing_circles.len() > 0 {
        for i in 0..m.breathing_circles.len() {
            m.breathing_circles[i].update(0.1);
        }
    }
    if m.touchosc.button("/redraw") {
        m.circles.clear(); //dump prev circles
        m.breathing_circles.clear();

        let window = app.window_rect();
        for _ in 0..=N_CIRCLES {
            for _attempt in 0..=CREATE_CIRCLE_ATTEMPTS {
                let x = random_range(window.left(), window.right());
                let y = random_range(window.top(), window.bottom());
                let radius = random_range(MIN_RADIUS, MAX_RADIUS);
                let h = map_range(radius, MIN_RADIUS, MAX_RADIUS, 0.5, 1.0);
                let a = map_range(radius, MIN_RADIUS, MAX_RADIUS, 0.1, 1.0);
                let a2 = map_range(radius, MIN_RADIUS, MAX_RADIUS, 1.0, 0.5);
                let w = map_range(radius, MIN_RADIUS, MAX_RADIUS, 0.1, 3.0);
                let c = Circle {
                    x: x,
                    y: y,
                    radius: radius,
                    weight: w,
                    color: hsva(h, 1.0, 1.0, a),
                };
               
                if c.any_collision(&m.circles) {
                    continue;
                }

                m.circles.push(c);

                if radius > MIN_BREATH_RADIUS {
                    let bc = BreathingCircle {
                        x: x,
                        y: y,
                        radius: radius,
                        weight: w,
                        color: hsva(h, 1.0, 1.0, a2),
                        inc: 0.0
                    };
                    m.breathing_circles.push(bc);
                }

                break;
            }
        }
    }
}
//--------------------------------------------------------
fn view(app: &App, m: &Model, frame: Frame) {
    let window = app.window_rect();
    let draw = app.draw();
    let bg = rgba(0.0, 0.0, 0.0, 0.1);
    if app.elapsed_frames() < 10 {
        //must clear render context once for fullscreen
        draw.background().color(BLACK);
    } else {
        draw.rect()
            .x_y(0.0, 0.0)
            .w_h(window.w() * 2.0, window.w() * 2.0)
            .color(bg);
    }

    for i in 0..m.breathing_circles.len() {
        m.breathing_circles[i].draw(&draw, i as f32);
    }
    for c in m.circles.iter() {
        let points = (0..=360).map(|i| {
            let radian = deg_to_rad(i as f32);
            let x_ = c.x + radian.sin() * c.radius;
            let y_ = c.y + radian.cos() * c.radius;
            (pt2(x_, y_), c.color)
        });

        draw.polyline()
            .weight(c.weight)
            .points_colored(points);
    }
    draw.to_frame(app, &frame).unwrap();
}
