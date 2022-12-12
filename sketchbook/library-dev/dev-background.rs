use nannou::prelude::*;
// use library::colors::Palette;
use library::background::Background;
use nannou_touchosc::TouchOscClient;

use nannou::color::Hsv;
use nannou::color::Rgb;
use nannou::color::Srgb;

//research
/*
nannou/generative_design $ cargo run --release --example p_1_2_3_01

wgpu:

https://github.com/gfx-rs/wgpu

*/

pub struct Palette {
    pub colors: Vec<Rgb>,
    pub len: usize,
}

impl Palette {
    pub fn new() -> Self {
        //anime sky
        let raw_colors: [u32; 49] = [
            0xFF15283D, 0xFF0F1925, 0xFF203D59, 0xFF2E2A33, 0xFF3B4259, 0xFF487EB3, 0xFF4F537E,
            0xFF325C83, 0xFF5A5366, 0xFF5696C3, 0xFF2D3A68, 0xFF71729D, 0xFF4C344D, 0xFF6B5457,
            0xFF785272, 0xFF7B697E, 0xFF472429, 0xFF43649F, 0xFF682D44, 0xFF61AEE9, 0xFF9387AA,
            0xFF9D4A60, 0xFF822E37, 0xFFB98377, 0xFF87A0D1, 0xFFAA6E81, 0xFFC5737A, 0xFFB69EB0,
            0xFF8D5658, 0xFF907070, 0xFFD69D9E, 0xFFF5BC9F, 0xFFB87BA0, 0xFFFFFCE1, 0xFFFCDCC5,
            0xFF73D3F6, 0xFFE287A3, 0xFFDA4945, 0xFFF19888, 0xFFFDD89E, 0xFFEAC2BE, 0xFFFEF3C6,
            0xFFD89A76, 0xFFD8616A, 0xFFF6B873, 0xFFB4594E, 0xFFF17F63, 0xFFE0E1EA, 0xFFA4A9A5,
        ];
        let raw_colorsv = raw_colors.to_vec();

        //do the conversion myself
        let mut cols_rgb: Vec<Rgb> = raw_colorsv
            .into_iter()
            .map(|c| {
                let blue: u8 = (c & 0xFF) as u8;
                let green: u8 = ((c >> 8) & 0xFF) as u8;
                let red: u8 = ((c >> 16) & 0xFF) as u8;
                let c = Srgb::new(
                    red as f32 / 255.0,
                    green as f32 / 255.0,
                    blue as f32 / 255.0,
                );
                c
            })
            .collect();

        //sort on sat/value/hue
        cols_rgb.sort_unstable_by(|&a, &b| {
            let ahsv: Hsv = a.into();
            let bhsv: Hsv = b.into();
            //colors are rgb
            //convert to hsv
            let ahue = ahsv.hue.to_positive_radians();
            let bhue = bhsv.hue.to_positive_radians();
            ahue.partial_cmp(&bhue).unwrap()
        });

        let len = cols_rgb.len();
        Palette {
            colors: cols_rgb,
            len: len,
        }
    }

    pub fn somecolor_frac(&self, mut frac: f32) -> Rgb {
        while frac < 0.0 {
            frac += 1.0;
        }
        while frac >= 1.0 {
            frac -= 1.0;
        }

        let index = (frac * self.colors.len() as f32) as usize;
        self.colors[index]
    }
}

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    touch_osc: TouchOscClient,
    palette: Palette,
    inc: f32,
    bg: Background,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).size(800, 200).build().unwrap();

    app.set_loop_mode(LoopMode::loop_once());
    let palette = Palette::new();

    let mut bg = Background::new(app.window_rect().w(), app.window_rect().h());
    
    let start_col = palette.colors[4];
    let end_col = palette.colors[10];

    let mut gradient_colors = Vec::new();

    gradient_colors.push( rgb(1.0, 0.0, 0.0) ); //red
    gradient_colors.push( rgb(0.0, 1.0, 0.0) ); //green
    gradient_colors.push( rgb(0.0, 0.0, 1.0) ); //blue
    // gradient_colors.push( rgb(0.0, 0.0, 0.0) ); //black
    // gradient_colors.push( rgb(1.0, 1.0, 1.0) ); //white
    
    // bg.set_gradient_colors(start_col, end_col);
    bg.set_colors(gradient_colors);


    Model { 
        _window,
        touch_osc : TouchOscClient::new(8010),
        palette,inc: 0.0,
        bg
    }
}

fn update(_app: &App, m: &mut Model, _update: Update) {
   // m.inc+=0.0001;

    

}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    let win  = app.window_rect();
    draw.background().color(WHITE);

    m.bg.draw(&draw);

    // for i in 0..win.w() as i32 {
    //     let x = map_range(i, 0, win.w() as i32, -win.w()*0.5, win.w()*0.5);
    //     let gradient =
    //         vec3(0.0, 0.0, 0.0).lerp( vec3(1.0, 1.0, 1.0), map_range(x, -win.w()*0.5, win.w()*0.5, 0.0, 1.0) );
    //     draw.line()
    //         .start(pt2(x, -win.h()*0.5))
    //         .end(pt2(x, win.h()*0.5))
    //         .weight(1.0)
    //         .color(rgba(gradient.x, gradient.y, gradient.z, 1.0));
    // // loop code here
    // }

    //  let gradient =
    //     vec3(0.0, 0.0, 0.0).lerp( vec3(1.0, 1.0, 1.0), map_range(app.mouse.x, -win.w()*0.5, win.w()*0.5, 0.0, 1.0) );
   
    // m.bg_color.draw(&draw);
   // let c = m.palette.somecolor_frac(m.inc);
    // draw.background().color(m.palette.colors[0]);

    // draw.background().color(rgba(gradient.x, gradient.y, gradient.z, 1.0));
    // draw.ellipse().color( m.palette.colors[0] );

    draw.to_frame(app, &frame).unwrap();
}