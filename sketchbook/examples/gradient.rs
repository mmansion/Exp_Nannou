use std::vec;

// use crate::nannou::color::{IntoLinSrgba, Srgb, Srgba};
use nannou::{prelude::*};
// use nannou::color::{Srgb, LinSrgb, Gradient};
use nannou::color::{LinSrgba, Srgba, Gradient};

fn main() {
    nannou::sketch(view)
        .size(800, 800)
        .loop_mode(LoopMode::loop_once())
        .run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let win_w = app.window_rect().w();
    let win_h = app.window_rect().h();

    draw.background().color(WHITE);

    draw.to_frame(app, &frame).unwrap();

    // let orangeish = Srgb::new(1.0 as f32, 0.6 as f32, 0.0 as f32);
    // let blueish = Srgb::new(0.0 as f32, 0.2 as f32, 1.0 as f32);

    // let gradient = Gradient::new(vec![
    //     orangeish.IntoLinSrgba(),
    //     blueish.IntoLinSrgba()
    // ]);
    // let c1:IntoLinSrgba<f32> = Srgba<f32>::new(0.0 as f32, 0.0 as f32, 0.0 as f32, 0.0 as f32);
    // let c2:IntoLinSrgba<f32> = Srgba<f32>::new(0.0 as f32, 0.0 as f32, 0.0 as f32, 1.0 as f32);

    // let lin_srgba_1 = c1.into_lin_srgba();
    // let lin_srgba_2 = c2.into_lin_srgba();


    let blue1 = LinSrgba::<f64>::from_components((0.0, 0.0, 1.0, 1.0)).into_linear();
    let blue2 = lin_srgba(0.0, 0.0, 1.0, 1.0);
    // let gold1 = LinSrgba::<f64>::from(rgba(GOLD.red, GOLD.green, GOLD.blue, 255).into_lin_srgba());
    // let gold2: LinSrgba::<f64> = rgba(GOLD.red, GOLD.green, GOLD.blue, 255).into_lin_srgba();

    let gradient: Gradient<LinSrgba<f64>> = Gradient::new(
	    vec![blue1, blue2]
    );


    // draw.rect()
    //     .xy(app.window_rect().xy())
    //     .w_h(win_w, win_h)
    //     .color(gradient);

    //draw rectangle with gradient
    let count = 100;
    for i in 0..count {
        draw_gradient_rect(&draw, win_w, win_h, i, count, &gradient);
    }


}