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
static CAPTURE  : bool = false; // capture to image sequence (or use obs)
static FRAME    : bool = true; //hide window chrome when set to false
static WIDTH    : f32 = 800.0;
static HEIGHT   : f32 = 800.0; 
static BORDER   : f32 = 10.0;
static WAIT     : u128 = 0;

const LINE_WIDTH: f32 = 1.0;
const MIN_RADIUS: f32 = 1.0;
const MAX_RADIUS: f32 = 50.0;
const N_CIRCLES: usize = 10000;
const CREATE_CIRCLE_ATTEMPTS: usize = 500;


//--------------------------------------------------------
struct Circle {
    x: f32,
    y: f32,
    radius: f32,
    color: Hsva
}

impl Circle {
    fn collides(&self, other: &Circle) -> bool {
        let a = self.radius + other.radius;
        let x = self.x - other.x;
        let y = self.y - other.y;

        if a >= ((x*x) + (y*y)).sqrt() {
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
    calculated: bool
}

fn main() {
    nannou::app(model).update(update).run();
}


fn model(app: &App) -> Model {
    // app.set_loop_mode(LoopMode::loop_once());

    app
    .new_window()
    .size(WIDTH as u32, HEIGHT as u32)
    .decorations(FRAME) //creates a borderless window
    .view(view)
    .build()
    .unwrap()
    ;
    // To initialize a TouchOscClient you instantiate it as "mutable"
    // and pass the OSC port where messages will be received.
    let mut touchosc = TouchOscClient::new(6555);

    touchosc.verbose();//enable debugging

    // Adding touchosc client inputs.

    // touchosc.add_radio("/invert", 2, 0);


    //--------------------------------------------------------

    Model {
        touchosc,
        circles: new::Vec,
        calculated: false
    }
}

fn update(app: &App, m: &mut Model, _update: Update) {

    m.touchosc.update(); 

    if m.circles.len() < 1 {

        let mut circles = Vec::<Circle>::with_capacity(N_CIRCLES);
    
        for _ in 0..=N_CIRCLES {
            for _attempt in 0..=CREATE_CIRCLE_ATTEMPTS {            
                let x = random_range(window.left(), window.right());
                let y = random_range(window.top(), window.bottom());
                let radius = random_range(MIN_RADIUS, MAX_RADIUS);
                let h = map_range(radius, MIN_RADIUS, MAX_RADIUS, 0.0, 1.0);
                let c = Circle {
                    x: x,
                    y: y,
                    radius: radius,
                    color: hsva(h, 1.0, 1.0, 1.0)
                };
                
                if c.any_collision(&circles) {
                    continue;
                }
    
                circles.push(c);
                break;
            }
        }
    }

   

}

fn view(app: &App, m: &Model, frame: Frame) {

    let window = app.window_rect();

    let draw = app.draw();

    draw.background()
        .color(BLACK);


    for c in circles {
        let line_points = (0..=360).map(|i| {
            // Convert each degree to radians.
            let radian = deg_to_rad(i as f32);
            // Get the sine of the radian to find the x co-ordinate of this point of the circle
            // and multiply it by the radius.
            let x_ = c.x + radian.sin() * c.radius;
            // Do the same with cosine to find the y co-ordinate.
            let y_ = c.y + radian.cos() * c.radius;
            // Construct and return a point object with a color.
            (pt2(x_, y_), c.color)
        });

        draw.polyline()
            .weight(LINE_WIDTH)
            .points_colored(line_points)
            ;
    }

    draw.to_frame(app, &frame).unwrap();


}