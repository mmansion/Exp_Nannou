use nannou::prelude::*;
use nannou::Draw;
use std::collections::VecDeque;

static CAPTURE: bool = true; // capture to image sequence
static WIDTH: i32 = 800;
static HEIGHT: i32 = 800;
static DIVS: i32 = 16;
static MARGIN: i32 = 100;
static LINE_LEN: usize = 200;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    debug: bool,
    // points : Vec<Vector2>, // points bin no.1
    points: Vec<Point>,
    this_capture_frame: i32,
    last_capture_frame: i32,
    vehicles: Vec<Vehicle>,
    d: f32,
}
// ---------------------------------------------------------------------------
struct Point {
    position: Point2,
    velocity: Vector2,
    acceleration: Vector2,
    mass: f32,
    size: f32,
}
impl Point {
    fn new(x: f32, y: f32, m: f32, s: f32) -> Self {
        let mass = m;
        let position = pt2(x, y);
        let size = s;
        let velocity = vec2(0.0, 0.0);
        let acceleration = vec2(0.0, 0.0);
        Point {
            position,
            velocity,
            acceleration,
            mass,
            size,
        }
    }
    fn display(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.position)
            .radius(self.size)
            .color(GRAY);
    }
}

// ---------------------------------------------------------------------------

struct Vehicle {
    history: VecDeque<Vector2>,
    position: Vector2,
    velocity: Vector2,
    acceleration: Vector2,
    r: f32,
    // Maximum steering force
    max_force: f32,
    // Maximum speed
    max_speed: f32,
    mass: f32,
}

impl Vehicle {
    fn new(x: f32, y: f32) -> Self {
        let mass = 3.0;
        let history = VecDeque::<Vector2>::with_capacity(10000);
        let position = vec2(x, y);
        let velocity = vec2(3.0, -2.0);
        let acceleration = vec2(0.0, 0.0);
        let r = 10.0;
        let max_force = 0.1;
        let max_speed = 3.0;

        Vehicle {
            mass,
            history,
            position,
            velocity,
            acceleration,
            r,
            max_force,
            max_speed,
        }
    }

    // Method to update position
    fn update(&mut self) {
        // Update velocity
        self.velocity += self.acceleration;
        // Limit speed
        self.velocity.limit_magnitude(self.max_speed);
        self.position += self.velocity;
        // Reset accelerationelertion to 0 each cycle
        self.acceleration *= 0.0;
        self.history.push_back(self.position);
        if self.history.len() > LINE_LEN {
            self.history.pop_front();
        }
    }

    fn apply_force(&mut self, force: Vector2) {
        // We could add mass here if we want A = F / M
        self.acceleration += force;
    }

    fn repel(&self, p: &Point) -> Vector2 {
        let mut force = self.position - p.position; // Calculate direction of force
        let mut distance = force.magnitude(); // Distance between objects
        distance = distance.max(1.0).min(10000.0); // Limiting the distance to eliminate "extreme" results for very cose or very far object
        force = force.normalize(); // Normalize vector (distance doesn't matter, we just want this vector for direction)
        let g = 1.0;
        let strength = (g * self.mass * p.mass) / (distance * distance); // Calculate gravitational force magnitude

        force * (-1.0 * strength) // Get force vector --> magnitude * direction
    }

    fn boundaries(&mut self, d: f32, win: &Rect) {
        let left = win.left() + MARGIN as f32;
        let right = win.right() - MARGIN as f32;
        let top = win.top() - MARGIN as f32;
        let bottom = win.bottom() + MARGIN as f32;

        let desired = match self.position {
            Vector2 { x, .. } if x < left => Some(vec2(self.max_speed, self.velocity.y)),
            Vector2 { x, .. } if x > right => Some(vec2(-self.max_speed, self.velocity.y)),
            Vector2 { y, .. } if y < bottom => Some(vec2(self.velocity.x, self.max_speed)),
            Vector2 { y, .. } if y > top => Some(vec2(self.velocity.x, -self.max_speed)),
            _ => None,
        };

        if let Some(desired) = desired {
            let desired = desired.normalize() * self.max_speed;
            let steer = (desired - self.velocity).limit_magnitude(self.max_force);
            self.apply_force(steer);
        }
    }
}
// ----------------------------------------------------------------------
fn model(app: &App) -> Model {
    let rect = Rect::from_w_h(WIDTH, HEIGHT);

    app.new_window()
        .size(800, 800)
        .view(view)
        .mouse_pressed(mouse_pressed)
        .build()
        .unwrap();

    let debug = false;
    let d = 50.0;
    let d2 = 1.0;

    let mut vehicles = Vec::new();
    for i in 0..30 {
        let randX = random_f32();
        let randY = random_f32();

        vehicles.push(Vehicle::new(randX, randY));
    }

    let mut points = Vec::new();
    let mut this_capture_frame = 0;
    let mut last_capture_frame = 0;
    //----------------------------------

    // let _points = (0..DIVS*2+2)
    //     .map(|_| {
    //         Point::new(
    //             random_range(4.0f32, 12.0),
    //             random_range(rect.left(), rect.right()),
    //             random_range(rect.top(), rect.bottom()),
    //         )
    //     })
    //     .collect();

    // let attractor = Attractor::new(rect);

    //----------------------------------

    for row in 0..(DIVS + 1) {
        let y = ((HEIGHT / DIVS * row) + (-HEIGHT / 2)) as f32;

        for col in 0..(DIVS + 1) {
            let x = ((WIDTH / DIVS * col) + (-WIDTH / 2)) as f32;

            points.push(Point::new(x, y, 4.0, 10.0))
            // points.push(pt2(x + (-WIDTH/2) as f32 , y + (-HEIGHT/2) as f32));
        }
    }

    Model {
        points,
        vehicles,
        this_capture_frame,
        last_capture_frame,
        debug,
        d,
    }
}

fn update(app: &App, m: &mut Model, _update: Update) {
    if m.this_capture_frame != m.last_capture_frame {
        m.last_capture_frame = m.this_capture_frame;
    }

    if CAPTURE {
        m.this_capture_frame += 1;
    }

    //----------------------------------

    for v in 0..m.vehicles.len() {
        for i in 0..m.points.len() {
            let force = m.vehicles[v].repel(&m.points[i]);
            let steer = force.limit_magnitude(m.vehicles[v].max_force);
            m.vehicles[v].apply_force(steer);
        }

        m.vehicles[v].boundaries(m.d, &app.window_rect());
        m.vehicles[v].update();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();
    let win = app.window_rect();

    let bg = rgba(0.0, 0.0, 0.1, 0.0001);
    draw.background().color(bg);

    //let draw = draw.x_y((-WIDTH/2) as f32, (-HEIGHT/2) as f32);

    let t = app.time;

    for i in 0..model.points.len() {
        // println!( "{},{}", model.points[i].x, model.points[i].y );
        // let color = hsv( (t * 0.001 * i as f32).sin(), 1.0, 1.0);
        let color = hsva(
            map_range(abs(app.time.sin() * 0.001), 0.4, 0.9, 0.3, 0.75),
            1.0,
            1.0,
            0.1,
        );

        // if i > model.points.len() / 2  {
        //     color = hsva ( map_range( i, 0 , model.points.len() , 0.4 , 0.7), 1.0, 1.0, 1.0);
        // }
        // draw.ellipse()
        // .x_y(model.points[i].x, model.points[i].y)
        // .radius( ( (t*0.9) + i as f32).sin() * model.points.len() as f32 )
        // .color(color);

        // draw.ellipse()
        // .xy(model.points[i].position)
        // .radius( (t + i as f32).sin() * 10.0 as f32 )
        // .color( rgb(0.1,0.1, 0.1) );

        // draw.scale(1.3).rect()
        // .xy(model.points[i].position)
        // .w((t + i as f32).sin() * 10.0 as f32)
        // .h((t + i as f32).sin() * 10.0 as f32)
        // .color(GRAY);

        // if(i > 0) {
        //     let p1   = pt2( model.points[i-1].x, model.points[i-1].y);
        //     let p2   = pt2( model.points[i].x, model.points[i].y);

        //     draw.line()
        //     .start(p1)
        //     .end(p2)
        //     .weight(1.0)
        //     .color(color);
        // }
    }

    // ------------------------------------------------
    for v in 0..model.vehicles.len() {
        display(&model.vehicles[v], &draw, &app, v as i32);
    }

    // ------------------------------------------------

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();

    if model.this_capture_frame != model.last_capture_frame {
        // let mut owned_string: String = "hello ".to_owned();
        // let borrowed_string: String = "output/" + app.exe_name().unwrap() + ".png";

        let directory = "captures/".to_string();
        let app_name = app.exe_name().unwrap().to_string();
        // let frame_num  = model.this_capture_frame.to_string();
        let extension = ".png".to_string();

        let frame_num = format!("{:05}", model.this_capture_frame);

        let path = format!("{}{}{}", directory, frame_num, extension);

        app.main_window().capture_frame(path);
    }
}

fn display(vehicle: &Vehicle, draw: &Draw, app: &App, num: i32) {
    let Vehicle {
        history,
        position,
        velocity,
        r,
        ..
    } = vehicle;

    if history.len() > 1 {
        let vertices = history
            .iter()
            .map(|v| pt2(v.x, v.y))
            .enumerate()
            .map(|(_, p)| {
                //let rgba = srgba(0.0, 0.0, 0.0, 1.0);
                let color = hsva(
                    map_range(
                        abs(app.time.sin() * 0.001 + (num * 2) as f32),
                        0.4,
                        0.9,
                        0.3,
                        0.75,
                    ),
                    1.0,
                    1.0,
                    1.0,
                );
                (p, color)
            });
        draw.polyline()
            .caps_round()
            .weight(num as f32)
            .points_colored(vertices);
    }

    // Draw a triangle rotated in the direction of velocity
    // This calculation is wrong
    // let theta = (velocity.angle() + PI / 2.0) * -1.0;
    // let points = vec![pt2(0.0, -r * 2.0), pt2(-r, r * 2.0), pt2(*r, r * 2.0)];
    // draw.polygon()
    //     .stroke(BLACK)
    //     .stroke_weight(1.0)
    //     .points(points)
    //     .xy(*position)
    //     .rgb(0.5, 0.5, 0.5)
    //     .rotate(-theta);
}

fn mouse_pressed(_app: &App, model: &mut Model, _button: MouseButton) {
    model.debug = !model.debug;
}
