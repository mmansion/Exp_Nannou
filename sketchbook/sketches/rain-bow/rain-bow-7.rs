/*
* rain-bow-2
* creative coding application for rain bow sculpture
* mikhail mansion 2021
*/
use nannou::geom::Point2;
use nannou::geom::*;
use nannou::prelude::*;

use nannou::Draw;
use std::ops::Range;
use std::time::Duration;

mod particles;
use particles::Particle2;

mod line;
use line::Line;

// use library::particles::Particle2;
use library::math::intersects_line;
// use library::line::Line;

//--------------------------------------------------------
static CAPTURE: bool = true; // capture to image sequence
static WIDTH: f32 = 800.0;
static HEIGHT: f32 = 800.0;

static VBOW_HEIGHT: f32 = -200.0;
static VBOW_MARGIN: f32 = 0.0;
static PARTICLE_SIZE: f32 = 80.0;
static PARTICLE_MASS: f32 = 1000.0;

//--------------------------------------------------------
fn main() {
    nannou::app(model).update(update).run()
}

// representation of physical v-shaped bow
struct VBow {
    left_point: Vec2,
    cent_point: Vec2,
    right_point: Vec2,
    left_line: Line,
    right_line: Line,

    left_midpoint: Vec2,
    right_midpoint: Vec2,

    right_normal_p1: Vec2,
    right_normal_p2: Vec2,
    left_normal_p1: Vec2,
    left_normal_p2: Vec2,

    right_normal_line: Line,
    left_normal_line: Line,

    left_angle_between: f32,
    right_angle_between: f32,

    left_normal_angle: f32,
    right_normal_angle: f32,
}

//--------------------------------------------------------
impl VBow {
    fn new(l_pt: Vec2, c_pt: Vec2, r_pt: Vec2) -> Self {
        let mut left_point = l_pt;
        let mut cent_point = c_pt;
        let mut right_point = r_pt;

        let mut left_normal_p1 = vec2(0.0, 0.0);
        let mut left_normal_p2 = vec2(0.0, 0.0);

        let mut right_normal_p1 = vec2(0.0, 0.0);
        let mut right_normal_p2 = vec2(0.0, 0.0);

        // represent 2 lines from points on the v-bow`
        let left_line = Line::new(left_point, cent_point);
        let right_line = Line::new(cent_point, right_point);

        let right_normal_line = Line::new(left_normal_p1, left_normal_p2);
        let left_normal_line = Line::new(right_normal_p1, right_normal_p2);

        let mut left_midpoint = vec2(0.0, 0.0);
        let mut right_midpoint = vec2(0.0, 0.0);

        let mut left_angle_between = 0.0;
        let mut right_angle_between = 0.0;

        let mut left_normal_angle = 0.0;
        let mut right_normal_angle = 0.0;

        VBow {
            left_point,
            cent_point,
            right_point,
            left_line,
            right_line,

            left_normal_p1,
            left_normal_p2,
            right_normal_p1,
            right_normal_p2,

            left_midpoint,
            right_midpoint,

            right_normal_line,
            left_normal_line,

            left_normal_angle,
            right_normal_angle,

            left_angle_between,
            right_angle_between,
        }
    }

    fn update(&mut self, x: f32, y: f32) {
        // let x = x;
        self.cent_point.x = x;
        self.cent_point.y = y;

        self.left_line
            .update_points(self.left_point, self.cent_point);
        self.right_line
            .update_points(self.cent_point, self.right_point);

        self.left_midpoint = self.left_line.get_midpoint();
        self.right_midpoint = self.right_line.get_midpoint();

        //--------------------------------------------------------
        // update lines. they have internal normals they're each keeping track of
        self.left_line.update();
        self.right_line.update();
        //--------------------------------------------------------

        // left normal:

        // calc surface normal for left line
        let dx1 = self.left_line.B.x - self.left_line.A.x;
        let dy1 = self.left_line.B.y - self.left_line.A.y;

        self.left_normal_p1 = vec2(-dy1, dx1);
        self.left_normal_p2 = vec2(dy1, -dx1);

        // right normal:

        //calc surf norm for right line
        let dx2 = self.right_line.B.x - self.right_line.A.x;
        let dy2 = self.right_line.B.y - self.right_line.A.y;

        self.right_normal_p1 = vec2(-dy2, dx2);
        self.right_normal_p2 = vec2(dy2, -dx2);

        // tmp test
        self.left_angle_between = (self.left_line.B.angle_between(self.left_line.A));

        self.left_normal_angle = (self.left_normal_p1.angle_between(self.left_normal_p2));
        //println!("{}", self.left_normal_angle);
    }

    fn display(&self, draw: &Draw) {
        //draw vbow
        let vbow_points = [self.left_point, self.cent_point, self.right_point];
        draw.scale(1.0)
            .polyline()
            .weight(2.0)
            .color(rgba(1.0, 1.0, 1.0, 1.0))
            .points(vbow_points);

        // TODO: translate to the midpoint
        let draw_left_norm = draw.translate(pt3(self.left_midpoint.x, self.left_midpoint.y, 0.0));
        let draw_right_norm =
            draw.translate(pt3(self.right_midpoint.x, self.right_midpoint.y, 0.0));

        //draw left normal line
        let left_normal_points = [self.left_normal_p1, vec2(0.0, 0.0)];
        let right_normal_points = [self.right_normal_p1, vec2(0.0, 0.0)];

        draw_left_norm
            .polyline()
            .weight(1.0)
            .color(RED)
            .points(left_normal_points);

        draw_right_norm
            .polyline()
            .weight(1.0)
            .color(RED)
            .points(right_normal_points);

        //--------------------------------------------------------
        // let angle_line_points = [
        //     vec2(0.0, 0.0), vec2(0.0, 100.0)
        // ];
        // let draw_left_angle = draw.rotate(self.left_normal_angle);

        // draw_left_angle
        // .polyline()
        // .weight(1.0)
        // .color(YELLOW)
        // .points(angle_line_points)
        // ;
    }
}
//--------------------------------------------------------

struct Model {
    this_capture_frame: i32,
    last_capture_frame: i32,
    last_calc: Duration,
    particles: Vec<Particle2>,
    vbow: VBow,
    angle: f32,
}

//--------------------------------------------------------
fn model(app: &App) -> Model {
    let rect = Rect::from_w_h(WIDTH, HEIGHT);

    let mut angle = 0.0;

    // app.set_loop_mode(LoopMode::loop_once());
    // app.set_loop_mode(LoopMode::rate_fps(0.1));

    // set_fullscreen

    app.new_window()
        .mouse_pressed(mouse_pressed)
        // .fullscreen()
        // .fullscreen( Some(nannou::winit::window::Fullscreen::Borderless(app.primary_monitor(),)) )
        .size(1920, 800)
        .view(view)
        .build()
        .unwrap();

    let mut last_calc = Duration::from_millis(0);

    //--------------------------------------------------------
    let mut this_capture_frame = 0;
    let mut last_capture_frame = 0;

    let mut particles = Vec::new();

    let p1 = pt2(-WIDTH / 2.0 + VBOW_MARGIN, VBOW_HEIGHT);
    let p2 = pt2(0.0, VBOW_HEIGHT);
    let p3 = pt2(WIDTH / 2.0 - VBOW_MARGIN, VBOW_HEIGHT);

    let mut vbow = VBow::new(p1, p2, p3);

    //--------------------------------------------------------

    Model {
        this_capture_frame,
        last_capture_frame,
        last_calc,
        particles,
        vbow,
        angle,
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

    //----------------------------------------------------------

    // UPDATE VBOW

    //update the vbow and its line positions
    m.vbow.update(app.mouse.x, app.mouse.y);

    for i in 0..m.particles.len() {
        //avoid
        //return
        //update
        //render

        // let wind = vec2(0.01, 0.0);
        let gravity = vec2(0.0, -0.1 * m.particles[i].mass);

        // calculate friction
        let c = 0.9;
        let mut friction = m.particles[i].vel;
        friction *= -1.0;
        friction.normalize();
        friction *= c;

        // apply forces
        m.particles[i].apply_force(gravity);
        m.particles[i].apply_force(friction);

        m.particles[i].update();
        m.particles[i].check_bounds(app.window_rect());

        //line 1
        let orig_pt = m.particles[i].orig;
        let pos_pt = m.particles[i].pos;

        let bow_center = m.vbow.left_line.B;

        // if particle falls below line and within x-range of line segment's points
        if !m
            .vbow
            .left_line
            .point_above_line(m.particles[i].pos, 0.0, -m.particles[i].size / 2.0)
            && m.particles[i].pos.x > m.vbow.left_line.A.x
            && m.particles[i].pos.x < m.vbow.left_line.B.x
        {
            m.particles[i].collide_line(&m.vbow.left_line);
        }

        if !m
            .vbow
            .right_line
            .point_above_line(m.particles[i].pos, 0.0, -m.particles[i].size / 2.0)
            && m.particles[i].pos.x > m.vbow.right_line.A.x
            && m.particles[i].pos.x < m.vbow.right_line.B.x
        {
            m.particles[i].collide_line(&m.vbow.right_line);
        }

        //--------------------------------------------------------
        //avoid
        for j in 0..m.particles.len() {
            if j != i {
                let pos = m.particles[j].pos;
                m.particles[i].avoid(pos);
            }
        }

        //--------------------------------------------------------
        // apply forces
    } //end particle iter

    // println!("{}", intersects_line());

    //----------------------------------------------------------
}

fn view(app: &App, m: &Model, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();
    let win = app.window_rect();
    let time = app.time;

    //--------------------------------------------------------
    // background

    let bg = rgba(0.0, 0.0, 0.0, 1.0);

    if app.elapsed_frames() == 1 {
        draw.background().color(rgba(0.0, 0.0, 0.0, 0.9));
    } else {
        draw.rect()
            .x_y(0.0, 0.0)
            .w_h(win.w() * 2.0, win.w() * 2.0)
            .color(bg);
    }

    //--------------------------------------------------------
    for particle in &m.particles {
        particle.display(&draw);
        // particle.display_line(&draw);
    }

    //m.vbow.display(&draw);

    let points = [
        m.vbow.left_line.A,
        m.vbow.left_line.B,
        m.vbow.right_line.B,
        vec2(WIDTH / 2.0, -HEIGHT / 2.0),
        vec2(-WIDTH / 2.0, -HEIGHT / 2.0),
    ];

    draw.polygon()
        .color(rgba(1.0, 0.0, 0.0, 1.0))
        .points(points);

    //--------------------------------------------------------

    // TEXT OUTPUT

    // We'll align to the window dimensions, but padded slightly.
    //let win_rect = app.main_window().rect().pad(20.0);

    // let text = format!("WEIRD phys{}cs", m.angle);

    // draw.text(&text)
    //     .color(BLACK)
    //     .color(WHITE)
    //     .font_size(24)
    //     .wh(win_rect.wh());

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

//--------------------------------------------------------
// EVENT HANDLERS

fn mouse_pressed(app: &App, m: &mut Model, b: MouseButton) {
    let last_ix = m.particles.len() as usize;
    m.particles.push(Particle2::new(
        app.mouse.x,
        HEIGHT / 2.0,
        PARTICLE_MASS,
        PARTICLE_SIZE,
    ));
}

// fn event(app: &App, model: &mut Model, event: Event) {

//     if let Event::WindowEvent {
//         id: _,
//         simple: window_event,
//     } = event

//     {
//         if let Some(window_event) = window_event {
//             if let KeyPressed(key) = window_event {
//                 match key {
//                     Key::F => {
//                         let window = app.window(model.window).unwrap();
//                         let new_fullscreen = if let Some(_) = window.fullscreen() {
//                             None
//                         } else {
//                             Some(nannou::winit::window::Fullscreen::Borderless(
//                                 app.primary_monitor(),
//                             ))
//                         };
//                         window.set_fullscreen(new_fullscreen);
//                     }
//                     _ => (),
//                 }
//             }
//         }
//     }
// }
