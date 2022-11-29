use nannou::prelude::*;
use library::colors::Palette;
use library::background::BG_Color;
use nannou_touchosc::TouchOscClient;

use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    touch_osc: TouchOscClient,
    bg_color: BG_Color,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).size(800, 800).build().unwrap();
    
    Model { 
        _window,
        touch_osc : TouchOscClient::new(8010),
        bg_color  : BG_Color::new(rgba(0.0, 1.0, 0.0, 1.0)),
    }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    _model.bg_color.update();
}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
   
    _model.bg_color.draw(&draw);
    draw.ellipse().color(STEELBLUE);
    draw.to_frame(app, &frame).unwrap();
}