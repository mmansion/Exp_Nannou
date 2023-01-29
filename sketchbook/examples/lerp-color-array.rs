//INCOMPLETE

use nannou::prelude::*;
use nannou::color::*;

struct Model {
    colors: Vec<LinSrgb>,
    current_color_index: usize,
    next_color_index: usize,
    t: f32,
}

fn update(app: &App, model: &mut Model, update: Update) {
    let t = (app.time * 2.0).sin();
    model.t = t;
    model.current_color_index = (t * (model.colors.len() - 1) as f32) as usize;
    model.next_color_index = (model.current_color_index + 1) % model.colors.len();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let current_color = model.colors[model.current_color_index];
    let next_color = model.colors[model.next_color_index];
    let current_color_vec = current_color.into_linear().into_vec();
    let next_color_vec = next_color.into_linear().into_vec();
    let lerp_color_vec = current_color_vec.lerp(next_color_vec, model.t);
    let lerp_color = LinSrgb::from_vec(lerp_color_vec);

    let draw = app.draw();
    draw.background().color(lerp_color);
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    let colors = vec![
        LinSrgb::new(1.0, 0.0, 0.0),
        LinSrgb::new(0.0, 1.0, 0.0),
        LinSrgb::new(0.0, 0.0, 1.0),
    ];

    let model = Model {
        colors,
        current_color_index: 0,
        next_color_index: 1,
        t: 0.0,
    };

    nannou::app(model)
        .update(update)
        .view(view)
        .run();
}
