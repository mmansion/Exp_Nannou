use nannou::prelude::*;
use nannou::ui::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

struct Model {
    ui: Ui,
    ids: Ids,
    x_slider : f32,
    y_slider : f32,
    pos_vec2 : Vec2,
    vel_vec2 : Vec2,
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

    let pos_vec2 = vec2(0.0, 0.0);
    let vel_vec2 = vec2(0.0, 0.0);

    Model {
        ui,
        ids,
        x_slider,
        y_slider,
        pos_vec2,
        vel_vec2,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    // Calling `set_widgets` allows us to instantiate some widgets.
    let ui = &mut model.ui.set_widgets();

    // update sliders

    fn x_slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
        widget::Slider::new(val, min, max)
            // .x_y(100.0, 30.0)
            .w_h(200.0, 30.0)
            .label_font_size(15)
            .rgb(0.3, 0.3, 0.3)
            .label_rgb(1.0, 1.0, 1.0)
            .border(0.0)
    }

    fn y_slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
        widget::Slider::new(val, min, max)
            // .x_y(10.0, 70.0)
            .w_h(200.0, 30.0)
            .label_font_size(15)
            .rgb(0.3, 0.3, 0.3)
            .label_rgb(1.0, 1.0, 1.0)
            .border(0.0)
    }

    for value in x_slider(model.x_slider as f32, 3.0, 15.0)
        .top_left_with_margin(60.0)
        .label("X Slider")
        .set(model.ids.x_slider, ui)
    {
        model.x_slider = value;
    }

    for value in y_slider(model.y_slider as f32, 3.0, 15.0)
        .top_left_with_margin(20.0)
        .label("Y Slider")
        .set(model.ids.y_slider, ui)
    {
        model.y_slider = value;
    }

}

// Draw the state of your `Model` into the given `Frame` here.
fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();

    draw.background().color(WHITE);

    draw.ellipse()
        .xy(pt2(0.0, 0.0))
        .radius(10.0 * model.x_slider)
        .color(BLACK);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();

    // Draw the state of the `Ui` to the frame.
    model.ui.draw_to_frame(app, &frame).unwrap();
}
