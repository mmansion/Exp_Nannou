/*
* n-0092
*
* dynamic grid, controlled via touchosc
*
* mikhail mansion 2022
*/

use library::colors::Palette;
use nannou::prelude::*;
use std::time::Duration;

use nannou::prelude::*;
use nannou_touchosc::TouchOscClient;

//--------------------------------------------------------
static CAPTURE: bool = false; // capture to image sequence (or use obs)
static FRAME: bool = true; //hide window chrome when set to false
static WIDTH: f32 = 800.0;
static HEIGHT: f32 = 800.0;
static BORDER: f32 = 10.0;
static WAIT: u128 = 10;

// Make sure this matches the `TARGET_PORT` in the `osc_sender.rs` example.
const PORT: u16 = 6555;

//--------------------------------------------------------
fn main() {
    nannou::app(model).update(update).run();
}

//--------------------------------------------------------
struct Model {
    window_id: WindowId,
    this_capture_frame: i32,
    last_capture_frame: i32,
    last_calc: Duration,
    colors: Palette,
    redraw: bool,
    last_redraw: u128,
    touchosc: TouchOscClient,
    outer_circles: Vec<Vec3>,
    inner_circles: Vec<Vec3>,
    circle_colors: Vec<Hsva>,
    once: bool,
}

//--------------------------------------------------------
fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(WIDTH as u32, HEIGHT as u32)
        .decorations(FRAME) //creates a borderless window
        .view(view)
        .build()
        .unwrap();

    // To initialize a TouchOscClient you instantiate it as "mutable"
    // and pass the OSC port where messages will be received.
    let mut touchosc = TouchOscClient::new(6555);

    touchosc.verbose(); //enable debugging

    // Adding touchosc client inputs.

    touchosc.add_radio("/invert", 2, 0);

    // app.set_loop_mode(LoopMode::loop_once());
    // app.set_loop_mode(LoopMode::rate_fps(0.1));

    let mut last_calc = Duration::from_millis(0);

    //--------------------------------------------------------
    let mut this_capture_frame = 0;
    let mut last_capture_frame = 0;

    //--------------------------------------------------------
    let mut redraw = false;
    let mut last_redraw = 0;

    //--------------------------------------------------------

    let colors = Palette::new();
    let mut outer_circles = Vec::new();
    let mut inner_circles = Vec::new();
    let mut circle_colors = Vec::new();

    //--------------------------------------------------------

    Model {
        window_id,
        this_capture_frame,
        last_capture_frame,
        last_calc,
        colors,
        redraw,
        last_redraw,
        touchosc,
        outer_circles,
        inner_circles,
        circle_colors,
        once: false,
    }
}

fn update(app: &App, m: &mut Model, _update: Update) {
    // ref:
    //https://doc.rust-lang.org/nightly/core/time/struct.Duration.html
    //let millis = Duration::from_millis(100).as_millis();
    let since_last_calc = _update.since_start.as_millis() - m.last_calc.as_millis();
    if since_last_calc > 10 {
        //time interval
        m.last_calc = _update.since_start;
    }

    if m.this_capture_frame != m.last_capture_frame {
        m.last_capture_frame = m.this_capture_frame;
    }

    if CAPTURE {
        m.this_capture_frame += 1;
    }

    //--------------------------------------------------------
    // redraw framerate workaround
    // change WAIT to increase interval

    if (_update.since_start.as_millis() - m.last_redraw > WAIT) {
        m.last_redraw = _update.since_start.as_millis();
        m.redraw = true;
    } else {
        m.redraw = false;
    }
    //--------------------------------------------------------

    m.touchosc.update();

    //--------------------------------------------------------
    if !m.once {
        let win = app.window_rect();
        let win_w = win.w();
        let win_h = win.h();
        // let x = random::<f32>() * win_w/2.0;
        let x = 0.0;
        // let y = random::<f32>() * win_h/2.0;
        let y = 0.0;
        let r = 300.0;
        let max = 2000;
        if !m.outer_circles.contains(&pt3(x, y, r)) {
            m.outer_circles.push(pt3(x, y, r));

            for i in 0..max {
                let R = (1.0 - random::<f32>() * random::<f32>() * random::<f32>()) * r;
                let angle = random::<f32>() * TAU;
                let X = x + angle.cos() * R;
                let Y = y + angle.sin() * R;

                let h = map_range(Y, 0.0, r, 0.7, 0.8);
                let c = hsva(h, 1.0, 1.0, 1.0);
                m.circle_colors.push(c);
                m.inner_circles.push(pt3(X, Y, 1.0));
            }
        }
        m.once = true;
    }
}

fn view(app: &App, m: &Model, frame: Frame) {
    // if(m.redraw) {

    // get canvas to draw on
    let draw = app.draw();
    let win = app.window_rect();
    let time = app.time;

    //--------------------------------------------------------
    // background

    let invert = m.touchosc.radio("/invert");

    let bg = rgba(0.01, 0.0, app.time.sin() * 0.1, 0.1);

    if app.elapsed_frames() == 10 {
        //must clear render context once for fullscreen
        draw.background().color(rgba(0.0, 0.0, 0.0, 0.9));
    } else {
        draw.rect()
            .x_y(0.0, 0.0)
            .w_h(win.w() * 2.0, win.w() * 2.0)
            .color(bg);
    }

    draw.ellipse()
        .color(rgba(0.1, 0.1, 1.0, 0.001))
        .w_h(600.0, 600.0)
        .x_y(0.0, 0.0);

    //--------------------------------------------------------

    for i in 0..m.inner_circles.len() {
        let circle = m.inner_circles[i];
        let color = m.circle_colors[i];

        let draw = draw.rotate(app.time * -0.01);

        if i > 0 && i % 10 == 0 {
            let prev_circle = m.inner_circles[i - 1];
            draw.line()
                .points(pt2(prev_circle.x, prev_circle.y), pt2(circle.x, circle.y))
                .stroke_weight(0.1)
                .color(rgba(0.0, 0.0, 1.0, 0.1));
        }

        let draw = draw.translate(pt3(circle.x, circle.y, 0.0));
        let scale = 2.0;
        let angles = 360;
        let pts = (0..angles + 1).map(|i| {
            let inc = ((360 / angles * i) as f32).to_radians();
            let x = inc.cos() * scale;
            let y = inc.sin() * scale;
            pt2(x, y)
        });

        draw.polyline()
            .color(color)
            .stroke_weight(0.4)
            .points_closed(pts);
    }

    //--------------------------------------------------------
    // draw frame

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();

    //--------------------------------------------------------
    // capture frame

    if m.this_capture_frame != m.last_capture_frame {
        let directory = "captures/".to_string();
        let app_name = app.exe_name().unwrap().to_string();
        let extension = ".png".to_string();
        let frame_num = format!("{:05}", m.this_capture_frame);

        let path = format!("{}{}{}", directory, frame_num, extension);
        app.main_window().capture_frame(path);
    }
    // }
}
