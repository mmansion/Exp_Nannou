use nannou::prelude::*;
use nannou::ui::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    ui: Ui,
    ids: Ids,
    x_slider: f32,
    y_slider: f32,
    base_vec2: Vec2,
    pos_a_vec2: Vec2,
    pos_b_vec2: Vec2,
    pos_c_vec2: Vec2,
    // vel2 :
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
    let x_slider = 0.0;
    let y_slider = 0.0;

    let base_vec2 = vec2(0.0, 0.0);

    let pos_a_vec2 = vec2(0.0, 0.0);
    let pos_b_vec2 = vec2(0.0, 0.0);
    let pos_c_vec2 = vec2(0.0, 0.0);

    Model {
        ui,
        ids,
        x_slider,
        y_slider,
        base_vec2,
        pos_a_vec2,
        pos_b_vec2,
        pos_c_vec2,
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

    for value in slider(model.x_slider, 0.0, 300.0)
        .top_left_with_margin(20.0)
        .label("X-Pos")
        .set(model.ids.x_slider, ui)
    {
        model.x_slider = value.round();
    }

    for value in slider(model.y_slider, 0.0, 300.0)
        .down(10.0)
        .label("Y-Pos")
        .set(model.ids.y_slider, ui)
    {
        model.y_slider = value;
    }

    model.pos_a_vec2.x = model.x_slider;
    model.pos_a_vec2.y = model.y_slider;

    model.pos_b_vec2.x = -model.pos_a_vec2.y;
    model.pos_b_vec2.y = model.pos_a_vec2.x;

    model.pos_c_vec2 = model.pos_a_vec2 + model.pos_b_vec2;
}

// Draw the state of your `Model` into the given `Frame` here.
fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    draw.background().color(WHITE);
    //draw.background().color(BLACK);

    // draw.ellipse()
    //     .xy(pt2(0.0, 0.0))
    //     .radius(10.0 * model.x_slider)
    //     .color(BLACK);

    // draw vector arrow from -> to
    draw.arrow()
        .weight(5.0)
        .color(BLUE)
        .points(model.base_vec2, model.pos_a_vec2);

    draw.arrow()
        .weight(5.0)
        .color(GREEN)
        .points(model.base_vec2, model.pos_b_vec2);

    draw.arrow()
        .weight(5.0)
        .color(BLACK)
        .points(model.base_vec2, model.pos_c_vec2);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();

    // Draw the state of the `Ui` to the frame.
    model.ui.draw_to_frame(app, &frame).unwrap();
}
