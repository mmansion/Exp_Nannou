use nannou::prelude::*;

// Carry Over Notes:

// [] upgrade and learn ab frame cap -> https://nannou.cc/posts/nannou_v0.13

static _WIDTH_: f32 = 800.0;
static _HEIGHT_: f32 = 800.0;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    mover: Mover,
    inc: f32,
    rad: f32,
}

struct Mover {
    position: Point2,
    velocity: Vector2,
    acceleration: Vector2,
    mass: f32,
    x: f32,
    y: f32,
}

impl Mover {
    fn new(rect: Rect) -> Self {
        let position = pt2(rect.left() + 30.0, rect.top() - 30.0);
        let velocity = vec2(0.0, 0.0);
        let acceleration = vec2(0.0, 0.0);
        let mass = 1.0;

        let mut x = 0.0;
        let mut y = 0.0;

        Mover {
            position,
            velocity,
            acceleration,
            mass,
            x,
            y,
        }
    }

    //mutable access xy
    fn x(&mut self) -> &mut f32 {
        &mut self.x
    }

    fn y(&mut self) -> &mut f32 {
        &mut self.y
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
    let rect = Rect::from_w_h(_WIDTH_, _HEIGHT_);

    app.new_window()
        .size(rect.w() as u32, rect.h() as u32)
        .view(view)
        .build()
        .unwrap();

    let mover = Mover::new(rect);
    let mut inc = 0.0;
    let rad = 300.0;

    Model { mover, inc, rad }
}

// do calculations here
/*
Then notice that you have a &mut Model in update: that's where you can mutate your data.
You can't do that in view, because it's only a reference, not a mutable one.
This is a design choice from nannou where you can't mutate things when you are drawing them.
Coming from processing it might be hard to adapt to this choice, but it makes things clearer.
*/
fn update(app: &App, m: &mut Model, _update: Update) {
    let wind = vec2(0.05, 0.0);
    let gravity = vec2(0.0, -0.1);

    m.mover.apply_force(wind);
    m.mover.apply_force(gravity);
    m.mover.update();
    m.mover.check_edges(app.window_rect());

    //increment inc
    m.inc = m.inc + 0.1;
}

// draw outputs here
fn view(app: &App, m: &Model, frame: Frame) {
    let win = app.window_rect();

    //println!("The window is {} x {}", win.w(), win.h());

    let t = app.time;

    // Begin drawing
    let draw = app.draw();

    let rotate = (app.time * 0.5).sin() * (app.time * 0.25 * PI * 2.0).cos();
    let draw = draw.rotate(rotate);

    // clear the bg
    let mut bg_col = rgba(0.0, 0.0, 0.0, 0.02);
    let mut fg_col = rgba(1.0, 1.0, 1.0, 0.1);
    // draw.background().color(bg_col);

    //background

    draw.rect()
        .x_y(0.0, 0.0)
        .w_h(win.w() * 2.0, win.w() * 2.0)
        .color(bg_col);

    // --------------------------------------

    let x = m.inc.cos() * m.rad;
    let y = m.inc.sin() * m.rad;

    let p = pt2(x, y);

    // --------------------------------------

    let mut color = rgba(1.0, 1.0, 1.0, 1.0);

    // --------------------------------------

    // define points
    // let p1_start = pt2( -win.w() / 2.0, win.h() / 2.0 );
    // let p1_end   = p;

    // draw.line()
    // .start(p1_start)
    // .end(p1_end)
    // .weight(1.0)
    // .color(color);

    // --------------------------------------

    // let p2_start = pt2( win.w() / 2.0, win.h() / 2.0 );
    // let p2_end   = p;

    // draw.line()
    // .start(p2_start)
    // .end(p2_end)
    // .weight(1.0)
    // .color(color);

    // --------------------------------------

    // let p3_start = pt2( win.w() / 2.0, -win.h() / 2.0 );
    // let p3_end   = p;

    // draw.line()
    // .start(p3_start)
    // .end(p3_end)
    // .weight(1.0)
    // .color(color);

    // --------------------------------------

    // let p4_start = pt2( -win.w() / 2.0, -win.h() / 2.0 );
    // let p4_end   = p;

    // draw.line()
    // .start(p4_start)
    // .end(p4_end)
    // .weight(1.0)
    // .color(color);

    //m.mover.display(&draw);
    //println!("hello?");

    for i in 0..8 {
        let f = i as f32;
        draw.ellipse()
            .x_y(0.0, 0.0)
            .w(m.rad * f * 0.5)
            .h(m.rad * f * 0.5)
            .stroke_weight(f * 1.5)
            .color(bg_col)
            .no_fill();
    }

    // ------------------------------------------
    let circle_resolution = 12;
    let angle = TAU / circle_resolution as f32;

    //draw.background().color(BLACK);

    for i in 0..circle_resolution {
        let x = (angle * i as f32).cos() * m.rad * 2.0;
        let y = (angle * i as f32).sin() * m.rad * 2.0;
        draw.line()
            .start(pt2(0.0, 0.0))
            .end(pt2(x, y))
            .stroke_weight(1.0)
            .caps_round()
            .color(fg_col);
    }
    // ------------------------------------------

    //println!("{}", m.mover.x);

    // --------------------------------------

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();

    // capture
    // if app.keys.down.contains(&Key::S) {
    //     app.main_window()
    //         .capture_frame(app.exe_name().unwrap() + ".png");
    // }
}
