use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    mover: Mover,
}

struct Mover {
    position: Point2,
    velocity: Vector2,
    acceleration: Vector2,
    mass: f32,
    name: String,
    x: f32,
    y: f32,
}

impl Mover {
    fn new(rect: Rect, n: String) -> Self {
        let position = pt2(rect.left() + 30.0, rect.top() - 30.0);
        let velocity = vec2(0.0, 0.0);
        let acceleration = vec2(0.0, 0.0);
        let mass = 1.0;
        let name = n;

        let mut x = 0.0;
        let mut y = 0.0;

        Mover {
            position,
            velocity,
            acceleration,
            mass,
            name,
            x,
            y,
        }
    }

    // Immutable access.
    fn name(&self) -> &String {
        &self.name
    }
    fn x(&self) -> &f32 {
        &self.x
    }
    fn y(&self) -> &f32 {
        &self.y
    }

    fn apply_force(&mut self, force: Vector2) {
        let f = force / self.mass;
        self.acceleration += f;
    }

    fn update(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
        self.acceleration *= 0.0;

        self.x = self.position.x;
        self.y = self.position.y;
    }

    fn check_edges(&mut self, rect: Rect) {
        if self.position.x > rect.right() {
            self.position.x = rect.right();
            self.velocity.x *= -1.0;
        } else if self.position.x < rect.left() {
            self.velocity.x *= -1.0;
            self.position.x = rect.left();
        }
        if self.position.y < rect.bottom() {
            self.velocity.y *= -1.0;
            self.position.y = rect.bottom();
        }
    }

    fn display(&self, draw: &Draw) {
        // display circle at x position
        draw.ellipse()
            .xy(self.position)
            .w_h(48.0, 48.0)
            .gray(0.3)
            .stroke(BLACK)
            .stroke_weight(2.0);
    }
}

fn model(app: &App) -> Model {
    let rect = Rect::from_w_h(640.0, 360.0);

    app.new_window()
        .size(rect.w() as u32, rect.h() as u32)
        .view(view)
        .build()
        .unwrap();
    let mover = Mover::new(rect, String::from("John"));
    Model { mover }
}

// do calculations here
fn update(app: &App, m: &mut Model, _update: Update) {
    let wind = vec2(0.01, 0.0);
    let gravity = vec2(0.0, -0.1);

    m.mover.apply_force(wind);
    m.mover.apply_force(gravity);
    m.mover.update();
    m.mover.check_edges(app.window_rect());
}

// draw outputs here
fn view(app: &App, m: &Model, frame: Frame) {
    let win = app.window_rect();
    //println!("The window is {} x {}", win.w(), win.h());

    // Begin drawing
    let draw = app.draw();

    // clear the bg
    let mut bg_col = rgba(0.0, 0.0, 0.0, 0.1);
    //draw.background().color(bg_col);

    draw.rect()
        .x_y(0.0, 0.0)
        .w_h(win.w(), win.w())
        .color(bg_col);

    // // Draw a rect that follows a different inverse of the ellipse.
    // draw.rect()
    //     .x_y(app.mouse.y, app.mouse.x)
    //     .w(app.mouse.x * 0.25)
    //     .hsv(t, 1.0, 1.0);

    // println!("{}", m.mover.x);

    // --------------------------------------

    let mut color = rgba(1.0, 1.0, 1.0, 1.0);

    // define points
    let p1_start = pt2(-win.w() / 2.0, win.h() / 2.0);
    let p1_end = pt2(m.mover.x, m.mover.y);

    draw.line()
        .start(p1_start)
        .end(p1_end)
        .weight(1.0)
        .color(color);

    // --------------------------------------

    let p2_start = pt2(win.w() / 2.0, win.h() / 2.0);
    let p2_end = pt2(m.mover.x, m.mover.y);

    draw.line()
        .start(p2_start)
        .end(p2_end)
        .weight(1.0)
        .color(color);

    // --------------------------------------

    let p3_start = pt2(win.w() / 2.0, -win.h() / 2.0);
    let p3_end = pt2(m.mover.x, m.mover.y);

    draw.line()
        .start(p3_start)
        .end(p3_end)
        .weight(1.0)
        .color(color);

    // --------------------------------------

    let p4_start = pt2(-win.w() / 2.0, -win.h() / 2.0);
    let p4_end = pt2(m.mover.x, m.mover.y);

    draw.line()
        .start(p4_start)
        .end(p4_end)
        .weight(1.0)
        .color(color);

    //m.mover.display(&draw);

    //println!("hello?");

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
