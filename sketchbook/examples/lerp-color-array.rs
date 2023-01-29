use nannou::prelude::*;
use nannou::color::*;

struct Model {
    colors: Vec<LinSrgb>,
    current_color_index: usize,
    t: f32,
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.t += 0.01;
    if model.t > 1.0 {
        model.t = 0.0;
        model.current_color_index += 1;
        if model.current_color_index >= model.colors.len() {
            model.current_color_index = 0;
        }
    }
}

fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw();
    let current_color = model.colors[model.current_color_index];
    let next_color = model.colors[(model.current_color_index + 1) % model.colors.len()];
    let lerp_color = current_color.lerp(next_color, model.t);
    draw.background().color(lerp_color);
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model)
        .update(update)
        .view(view)
        .run();
}
