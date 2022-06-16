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
const MAX_RADIUS: f32 = 50.0;
const N_CIRCLES: usize = 50;
const N_LINES:usize = 10;
const CREATE_CIRCLE_ATTEMPTS: usize = 500;


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
    window_id: WindowId,
    touchosc: TouchOscClient,
    circles: Vec<Circle>,
    line_len: f32
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    // app.set_loop_mode(LoopMode::loop_once());

    let window_id = app.new_window()
        .size(WIDTH as u32, HEIGHT as u32)
        .decorations(FRAME) //creates a borderless window
        .view(view)
        .build()
        .unwrap();

    app.main_window().set_outer_position_pixels(0,0);

    let line_len = 100.0;

    // To initialize a TouchOscClient you instantiate it as "mutable"
    // and pass the OSC port where messages will be received.
    let mut touchosc = TouchOscClient::new(6555);

    touchosc.verbose(); //enable debugging

    // Adding touchosc client inputs.
    touchosc.add_button("/redraw", false);  
    touchosc.add_button("/show-line", false);  
    touchosc.add_fader("/line-length", 50.0, 380.0, line_len);  

    let mut circles = Vec::<Circle>::with_capacity(N_CIRCLES);

    //--------------------------------------------------------

    Model {
        window_id,
        touchosc,
        circles,
        line_len
    }
}

fn update(app: &App, m: &mut Model, _update: Update) {

    m.touchosc.update();
    
    // if m.touchosc.button("/redraw") {
    if m.touchosc.fader("/line-length") != m.line_len {

        m.line_len = m.touchosc.fader("/line-length");
        
        m.circles.clear(); //dump prev circles

        let window = app.window_rect();
        let spacing = window.h() / N_LINES as f32;

        for l in 0..N_LINES {

            let y = window.bottom() + (spacing * l as f32) + spacing/2.0;
        
            for _ in 0..=N_CIRCLES {
            for _attempt in 0..=CREATE_CIRCLE_ATTEMPTS {
    
                let line_extent = m.touchosc.fader("/line-length");
                let x = random_range(-line_extent, line_extent);
    
    
                let radius = random_range(MIN_RADIUS, MAX_RADIUS);
                let h = map_range(radius, MIN_RADIUS, MAX_RADIUS, 0.5, 1.0);
                let a = map_range(radius, MIN_RADIUS, MAX_RADIUS, 0.5, 1.0);
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
                break;
            }
        }
        }
            

    }
}
//--------------------------------------------------------
fn view(app: &App, m: &Model, frame: Frame) {
    let window = app.window_rect();
    let draw = app.draw();
    // let bg = rgba(1.0, 1.0, 1.0, 1.0);
    let bg = BLACK;

    if app.elapsed_frames() < 10 {
        //must clear render context once for fullscreen
        draw.background().color(BLACK);
    } else {
        draw.rect()
            .x_y(0.0, 0.0)
            .w_h(window.w() * 2.0, window.w() * 2.0)
            .color(bg);
    }

    if m.touchosc.button("/show-line") {

        let line_len =m.touchosc.fader("/line-length");
        let spacing = window.h() / N_LINES as f32;

        for l in 0..N_LINES {
            let y = window.bottom() + (spacing * l as f32) + spacing/2.0;

            draw
            .line()
            .color(WHITE)
            .stroke_weight(2.0)
            .points(pt2(-line_len, y), pt2(line_len, y))
            ;
        }
    }



    for c in m.circles.iter() {
        let points = (0..=360).map(|i| {
            let radian = deg_to_rad(i as f32);
            let x_ = c.x + radian.sin() * c.radius;
            let y_ = c.y + radian.cos() * c.radius;
            (pt2(x_, y_), c.color)
        });

        let mut fill = c.color;
        let d = c.radius*2.0;
        let pos = pt2(c.x, c.y);

        fill.alpha = 0.1;
        
        draw.ellipse()
        .xy(pos)
        .color(fill)
        .w_h(d, d)
        ;

        draw.polyline()
            .weight(c.weight)
            .points_colored(points);
    }
    draw.to_frame(app, &frame).unwrap();
}
