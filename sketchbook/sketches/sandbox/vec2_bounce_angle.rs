/*
* vec2_bounce_angle
*
* bounce a moving object at a specified angle by changing its velocity
*
* mikhail mansion 2021
*/

use nannou::geom::Point2;
use nannou::geom::*;
use nannou::prelude::*;
use nannou::prelude::*;
use nannou::ui::prelude::*;
use nannou::Draw;
use std::ops::Range;
use std::time::Duration;

//--------------------------------------------------------
static WIDTH: u32 = 800;
static HEIGHT: u32 = 800;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Mover {
    orig: Vec2,
    pos: Vec2,
    vel: Vec2,
    max_speed: f32,
}

impl Mover {
    pub fn new(_pos: Vec2) -> Self {
        let max_speed = 10.0;
        let orig = _pos;
        let pos = _pos;
        let vel = pt2(0.0, -max_speed);
        Mover {
            orig,
            pos,
            vel,
            max_speed,
        }
    }

    pub fn update(&mut self) {
        self.pos += self.vel;
    }

    pub fn check_bounds(&mut self, A: Vec2, B: Vec2, normal: Vec2, width: f32, height: f32) {
        let clampled_normal = normal.clamp_length_max(self.max_speed);
        println!("{}, {}", clampled_normal.x, clampled_normal.y);

        // check if below line

        let m = (B.y - A.y) / (B.x - A.x); //slope
        let b = m * A.x - A.y; //y-intercept
                               // point-slope form:
                               // y = mx + b
                               // does x and y satisfy the equation
        let diff = self.pos.y - m * self.pos.x + b;

        // check
        if diff < 0.0 {
            println!("true");
            self.vel = clampled_normal;
        } else if self.pos.y < -height / 2.0
            || self.pos.y > height / 2.0
            || self.pos.x < -width / 2.0
            || self.pos.x > width / 2.0
        {
            self.pos.y = height / 2.0;
            self.pos.x = 0.0;
            self.vel = pt2(0.0, -self.max_speed);
        }
    }

    pub fn display(&self, draw: &Draw) {
        // draw.arrow().weight(5.0).color(BLUE).points(self.orig, self.pos);
        draw.ellipse()
            .xy(self.pos)
            .stroke(BLUE)
            // .color(BLUE)
            .stroke_weight(5.0)
            .w_h(20.0, 20.0);
    }
}

//--------------------------------------------------------

struct Model {
    ui: Ui,
    ids: Ids,
    y_slider: f32,
    line_p1: Vec2,
    line_p2: Vec2,
    normal: Vec2,
    mover: Mover,
}

widget_ids! {
    struct Ids {
        x_slider,
        y_slider
    }
}

fn model(app: &App) -> Model {
    // Set the loop mode to wait for events, an energy-efficient option for pure-GUI apps.
    app.set_loop_mode(LoopMode::Wait);

    // Create the UI.
    let mut ui = app.new_ui().build().unwrap();

    // Generate some ids for our widgets.
    let ids = Ids::new(ui.widget_id_generator());

    let w = app.window_rect().w();
    let h = app.window_rect().h();

    // Init our variables
    // let x_slider = 0.0;
    let y_slider = 0.0;

    let line_p1 = vec2(-w / 2.0, -h / 2.0);
    let line_p2 = vec2(w / 2.0, h / 2.0);

    // calculate 2d normal of line (perpendicular vector)
    let diff_x = line_p2.x - line_p1.x;
    let diff_y = line_p2.y - line_p1.y;

    let normal = vec2(-diff_y, diff_x);

    let mover = Mover::new(vec2(0.0, h / 2.0));

    Model {
        ui,
        ids,
        y_slider,
        line_p1,
        line_p2,
        normal,
        mover,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // Calling `set_widgets` allows us to instantiate some widgets.
    let ui = &mut model.ui.set_widgets();

    fn slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
        widget::Slider::new(val, min, max)
            .w_h(200.0, 30.0)
            .label_font_size(15)
            .rgb(0.3, 0.3, 0.3)
            .label_rgb(1.0, 1.0, 1.0)
            .border(0.0)
    }

    let h = _app.window_rect().h();

    for value in slider(model.y_slider, h / 2.0, -h / 2.0)
        .top_left_with_margin(20.0)
        .label("Y-Pos")
        .set(model.ids.y_slider, ui)
    {
        model.y_slider = value.round();
    }

    model.line_p2.y = model.y_slider;

    let diff_x = model.line_p2.x - model.line_p1.x;
    let diff_y = model.line_p2.y - model.line_p1.y;

    model.normal = vec2(-diff_y, diff_x);

    let w = _app.window_rect().w();
    let h = _app.window_rect().h();

    model
        .mover
        .check_bounds(model.line_p1, model.line_p2, model.normal, w, h);
    //update the mover
    model.mover.update();
}

// Draw the state of your `Model` into the given `Frame` here.
fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    draw.background().color(WHITE);

    // draw vector arrow from -> to
    draw.arrow()
        .weight(5.0)
        .color(BLUE)
        .points(model.line_p1, model.line_p2);

    let midpoint = vec2(
        (model.line_p1.x + model.line_p2.x) / 2.0,
        (model.line_p1.y + model.line_p2.y) / 2.0,
    );
    draw.arrow()
        .weight(2.0)
        .color(RED)
        .points(midpoint, model.normal);

    model.mover.display(&draw);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();

    // Draw the state of the `Ui` to the frame.
    model.ui.draw_to_frame(app, &frame).unwrap();
}
