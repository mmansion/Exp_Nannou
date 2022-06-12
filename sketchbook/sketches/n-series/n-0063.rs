/*
* n-0063
* exploring nannou (n-series)
* mikhail mansion 2021
*/

use nannou::prelude::*;
use nannou::Draw;
use std::collections::VecDeque;
use std::time::Duration;

//--------------------------------------------------------
static CAPTURE: bool = false; // capture to image sequence

static WIDTH: u32 = 800;
static HEIGHT: u32 = 800;
static MARGIN: f32 = 100.0;
static SIZE: f32 = 10.0;

//--------------------------------------------------------
fn main() {
    nannou::app(model).update(update).run();
}

//--------------------------------------------------------

struct Mover {
    orig: Vec2,
    pos: Vec2,
    last_pos: Vec2,
    vel: Vec2,
    max_speed: f32,
    size: f32,
    history: VecDeque<Vec2>,
}

impl Mover {
    pub fn new(_pos: Vec2, _size: f32) -> Self {
        let max_speed = 10.0;
        let orig = _pos;
        let pos = _pos;
        let last_pos = _pos;
        let size = _size;
        let vel = pt2(
            random_range(-max_speed, max_speed),
            random_range(-max_speed, max_speed),
        );
        let history = VecDeque::<Vec2>::with_capacity(10000);
        Mover {
            orig,
            pos,
            last_pos,
            vel,
            max_speed,
            size,
            history,
        }
    }

    pub fn update(&mut self) {
        self.history.push_back(self.pos);
        if self.history.len() > 10000 {
            self.history.pop_front();
        }

        self.last_pos = self.pos;
        self.pos += self.vel;
    }

    pub fn get_normal(&self, p1: Vec2, p2: Vec2) -> Vec2 {
        // A unit normal vector to a two-dimensional curve is
        // a vector with magnitude 1 that is perpendicular to the curve at some point.

        // calculate 2d normal of line (perpendicular vector)
        let diff_x = p2.x - p1.x;
        let diff_y = p2.y - p1.y;
        let normal = vec2(-diff_y, diff_x);
        let clampled_normal = normal.clamp_length_max(self.max_speed);

        return clampled_normal;
    }

    pub fn check_bounds(&mut self, win_w: f32, win_h: f32) {
        if self.pos.y > win_h / 2.0 - MARGIN {
            //past top edge
            self.pos.y = win_h / 2.0 - (self.size / 2.0) - MARGIN;
            self.vel.y *= -1.0;
        } else if self.pos.y < -win_h / 2.0 + MARGIN {
            // past bottom edge
            self.pos.y = -win_h / 2.0 + (self.size / 2.0) + MARGIN;
            self.vel.y *= -1.0;
        } else if self.pos.x < -win_w / 2.0 + MARGIN {
            //past left edge
            self.pos.x = -win_w / 2.0 + (self.size / 2.0) + MARGIN;
            self.vel.x *= -1.0;
        } else if self.pos.x > win_w / 2.0 - MARGIN {
            //past right edge
            self.pos.x = win_w / 2.0 - (self.size / 2.0) - MARGIN;
            self.vel.x *= -1.0;
        }
    }

    pub fn display(&self, draw: &Draw, app: &App) {
        if self.history.len() > 1 {
            let vertices = self
                .history
                .iter()
                .map(|v| pt2(v.x, v.y))
                .enumerate()
                .map(|(_, p)| {
                    //let rgba = srgba(0.0, 0.0, 0.0, 1.0);

                    let color = hsva(
                        map_range(abs(app.time.sin() * 0.1), 0.0, 1.0, 0.3, 0.75),
                        1.0,
                        1.0,
                        0.1,
                    );
                    (p, WHITE)
                });
            draw.polyline().weight(4.0).points_colored(vertices);
        }
    }
}

//--------------------------------------------------------
struct Model {
    this_capture_frame: i32,
    last_capture_frame: i32,
    last_calc: Duration,
    movers: Vec<Mover>,
    clear: bool,
}

//--------------------------------------------------------
fn model(app: &App) -> Model {
    app.new_window()
        .size(WIDTH, HEIGHT)
        .event(event)
        .view(view)
        .build()
        .unwrap();

    let w = app.window_rect().w();
    let h = app.window_rect().h();

    let mut last_calc = Duration::from_millis(0);

    let mut this_capture_frame = 0;
    let mut last_capture_frame = 0;

    let mut clear = true;

    //--------------------------------------------------------

    let mut movers = Vec::new();

    Model {
        this_capture_frame,
        last_capture_frame,
        last_calc,
        movers,
        clear,
    }
}

fn update(app: &App, m: &mut Model, _update: Update) {
    // ref:
    //https://doc.rust-lang.org/nightly/core/time/struct.Duration.html
    //let millis = Duration::from_millis(100).as_millis();
    let since_last_calc = _update.since_start.as_millis() - m.last_calc.as_millis();
    if since_last_calc > 10 {
        //time interval
        m.last_calc = _update.since_start;
    }

    if m.this_capture_frame != m.last_capture_frame {
        m.last_capture_frame = m.this_capture_frame;
    }

    if CAPTURE {
        m.this_capture_frame += 1;
    }

    //--------------------------------------------------------
    for mover in m.movers.iter_mut() {
        mover.check_bounds(app.window_rect().w(), app.window_rect().h());
        mover.update();
    }

    if m.clear && m.movers.len() < 1 {
        let half_w = app.window_rect().w() / 2.0;
        let half_h = app.window_rect().h() / 2.0;
        let pos = vec2(
            random_range(-half_w + MARGIN, half_w - MARGIN),
            random_range(-half_h + MARGIN, half_h - MARGIN),
        );
        m.movers.push(Mover::new(pos, SIZE));
    }
}

fn view(app: &App, m: &Model, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();
    let win = app.window_rect();
    let time = app.time;

    //--------------------------------------------------------
    // background

    let bg = rgba(0.0, 0.0, 0.0, 0.01);

    if app.elapsed_frames() == 1 || m.clear {
        draw.background().color(rgba(0.0, 0.0, 0.0, 1.0));
    } else {
        draw.rect()
            .x_y(0.0, 0.0)
            .w_h(win.w() * 2.0, win.w() * 2.0)
            .color(bg);
    }

    //--------------------------------------------------------
    //modify draw

    let draw = draw.rotate(time * 0.1);
    //--------------------------------------------------------
    let mut count = 2.0;
    for mover in m.movers.iter() {
        mover.display(&draw, &app);
    }

    //--------------------------------------------------------
    // draw frame

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();

    //--------------------------------------------------------
    // capture frame

    if m.this_capture_frame != m.last_capture_frame {
        let directory = "captures/".to_string();
        let app_name = app.exe_name().unwrap().to_string();
        let extension = ".png".to_string();
        let frame_num = format!("{:05}", m.this_capture_frame);

        let path = format!("{}{}{}", directory, frame_num, extension);
        app.main_window().capture_frame(path);
    }
}

fn event(app: &App, m: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(key) => {
            if let Key::Space = key {
                m.movers.pop();
                m.clear = true;
            }
        }
        MousePressed(button) => {
            // println!("global scope: GLOBAL = {}", GLOBAL);
        }
        _other => (),
    }
}
