/*
 * Simple Loop example with 4D noise
 * Daily Sketch 2019/09/23 by Alexis Andre (@mactuitui)
 *
 * Demonstration of looping an animation using periodic functions.
 *
 */

use nannou::noise::*;
use nannou::prelude::*;

fn main() {
    nannou::app(model).run();
}

struct Model {
    noise: Perlin,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(800, 800)
        .title("sketch")
        .view(view)
        .build()
        .unwrap();
    let mut noise = Perlin::new();
    noise = noise.set_seed(1);
    Model { noise }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let r = app.window_rect();
    let w = r.w();
    let h = r.h();

    // Put all the provided blend modes in a list.
    let blends = [
        ("NORMAL", BLEND_NORMAL),
        ("ADD", BLEND_ADD),
        ("SUBTRACT", BLEND_SUBTRACT),
        ("REVERSE SUBTRACT", BLEND_REVERSE_SUBTRACT),
        ("DARKEST", BLEND_DARKEST),
        ("LIGHTEST", BLEND_LIGHTEST),
    ];
    let (blend_name, desc) = &blends[4];

    // Prepare to draw.
    let draw = app.draw();
 
    draw.background().color(WHITE);
    // draw.color_blend(BLEND_DARKEST);
    // Assign the blend mode.
    let mut draw = draw.color_blend(desc.clone());

    //the loop is going to be 200 frames long
    let frac = (app.elapsed_frames() % 200) as f32 / (200.0);

    //we'll rotate in the noise space
    let rotcos = 0.2 * (frac * TAU).cos();
    let rotsin = 0.2 * (frac * TAU).sin();

    let n_lines = 100;

    //draw the lines
    for j in 0..n_lines {
        let frac_j = (j as f32) / 100.0;
        let mut pts = Vec::new();
        let mut pts2 = Vec::new();
        for i in 0..n_lines {
            let frac_i = (i as f32) / 100.0;
            //let scale = ((frac_i * PI).sin()).powf(3.0);
            let scale = ((frac_i * PI).sin()).powf(1.0);
            let offset = scale
                * (model.noise.get([
                    i as f64 * 0.015,
                    j as f64 * 0.15,
                    rotcos as f64,
                    rotsin as f64,
                ]) * -0.2
                    + 0.5) as f32;
            pts.push(vec2(
                -w/2.0 + frac_i * w,
                h/4.0 - frac_j * h + 160.0 * offset,
            ));
            pts2.push(vec2(
                -w/2.0 + frac_i * w,
                h/4.0 - frac_j * h + 160.0 * offset,
            ));
        }

        let mut hue = j as f32 / n_lines as f32;
        hue*=1.2;
        
        //let ix = map_range(app.mouse.y, w.top(), w.bottom(), 0, blends.len());

        let mut color = hsl(hue, 1.0, 0.5);

        //fill the line with black
        draw.polygon().color(color).points(pts);
        //draw the white outline on top
        // hue = 10.0;
        color = hsl(hue, 0.5, 0.1);
        draw.polyline().color(color).stroke_weight(1.2).points(pts2);
    }

    draw.to_frame(app, &frame).unwrap();
}
