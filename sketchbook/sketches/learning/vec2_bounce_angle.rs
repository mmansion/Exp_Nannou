/*
* vec2_bounce_angle
*
* bounce a moving object at a specified angle by changing its velocity
*
* mikhail mansion 2021
*/

use nannou::prelude::*;
use nannou::ui::prelude::*;

//--------------------------------------------------------
static WIDTH      : u32 = 400;
static HEIGHT     : u32 = 400; 

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Mover {
    orig : Vec2,
    pos  : Vec2,
    vel  : Vec2
}

impl Mover {
    pub fn new(_pos:Vec2) -> Self {
        let orig = _pos;
        let pos  = _pos;
        let vel  = pt2(0.0, -1.0);
        Mover {
            orig,
            pos,
            vel
        }
    }

    pub fn update(&mut self, normal:Vec2) {
        self.pos += self.vel;
    }
    pub fn display(&self, draw: &Draw) {
        // draw.arrow().weight(5.0).color(BLUE).points(self.orig, self.pos);
        draw
        .ellipse()
        .xy(self.pos)
        .stroke(BLUE)
        // .color(BLUE)
        .stroke_weight(5.0)
        .w_h(20.0, 20.0)
        ;
    }
}

//--------------------------------------------------------

struct Model {
    ui: Ui,
    ids: Ids,
    y_slider : f32,
    line_p1  : Vec2,
    line_p2  : Vec2,
    normal   : Vec2,
    mover    : Mover,
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

    // Init our variables
    // let x_slider = 0.0;
    let y_slider = 0.0;

    let w = app.window_rect().w();
    let h = app.window_rect().h();

    let line_p1 = vec2(-w/2.0, -h/2.0);
    let line_p2 = vec2(w/2.0, h/2.0);

    // calculate 2d normal of line (perpendicular vector)
    let diff_x = line_p2.x - line_p1.x;
    let diff_y = line_p2.y - line_p1.y;

    let normal = vec2(-diff_y, diff_x);

    let mover = Mover::new(vec2(0.0, h/2.0));

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

    for value in slider(model.y_slider, h/2.0, -h/2.0)
        .top_left_with_margin(20.0)
        .label("Y-Pos")
        .set(model.ids.y_slider, ui) {

        model.y_slider = value.round();
    }

    model.line_p2.y = model.y_slider;

    let diff_x = model.line_p2.x - model.line_p1.x;
    let diff_y = model.line_p2.y - model.line_p1.y;

    model.normal = vec2(-diff_y, diff_x);

    //update the mover
    model.mover.update(model.normal);
}

// Draw the state of your `Model` into the given `Frame` here.
fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    draw.background().color(WHITE);

    // draw vector arrow from -> to
    draw.arrow().weight(5.0).color(BLUE).points(model.line_p1, model.line_p2);
    draw.arrow().weight(2.0).color(RED).points(vec2(0.0, 0.0), model.normal);

    model.mover.display(&draw);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();

    // Draw the state of the `Ui` to the frame.
    model.ui.draw_to_frame(app, &frame).unwrap();
}
