/*
* n-0093
*
* recursive random, circle packing
*
* mikhail mansion 2022
*/

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
static WAIT: u128 = 100;

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
    redraw: bool,
    last_redraw: u128,
    touchosc: TouchOscClient,
    rings: Vec<Vec3>,
    points: Vec<Vec3>,
    colors: Vec<Hsva>,
    points_counter: i32,
    max_rings: i32,
    max_points: usize,
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

    // touchosc.add_radio("/invert", 2, 0);

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
    let mut rings = Vec::new();
    let mut points = Vec::new();
    let mut colors = Vec::new();

    let mut points_counter = 0;
    let mut max_rings = 10;
    let mut max_points = 40000;

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
        rings,
        points,
        points_counter,
        max_rings,
        max_points,
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
    if m.points_counter < m.max_rings {
        let max = m.max_points;

        let win = app.window_rect();
        let win_w = win.w();
        let win_h = win.h();
        let mut r = random::<f32>() * win_w / 4.0;
        if r < 100.0 {
            r = 100.0;
        }

        let mut found_position = false;

        let mut y = map_range(
            random::<f32>() * win_w,
            0.0,
            win_w,
            -win_w / 2.0,
            win_w / 2.0,
        );
        let mut x = map_range(
            random::<f32>() * win_w,
            0.0,
            win_w,
            -win_w / 2.0,
            win_w / 2.0,
        );

        // if(m.rings.len() > 1) {

        //     let prev_ring = m.rings[m.rings.len()-1];

        //     while !found_position {

        //         if abs(prev_ring.x - x) > (prev_ring.z + r) && abs(prev_ring.y - y) > (prev_ring.z + r) {
        //             found_position = true;

        //         } else {
        //             x = random::<f32>() * win_w/2.0;
        //             y = random::<f32>() * win_h/2.0;
        //         }
        //     }
        // }

        if !m.rings.contains(&pt3(x, y, r)) {
            m.rings.push(pt3(x, y, r));

            for i in 0..max {
                let R = (1.0 - random::<f32>() * random::<f32>() * random::<f32>()) * r;
                let angle = random::<f32>() * TAU;
                let X = x + angle.cos() * R;
                let Y = y + angle.sin() * R;

                let h = map_range(Y, 0.0, r, 0.8, 0.85);
                let c = hsva(h, 1.0, 1.0, 1.0);
                let s = random::<f32>() * (3.0 + r / win_w * 0.7);
                //let s = 1.0;

                m.colors.push(c);
                m.points.push(pt3(X, Y, s));
            }

            m.points_counter = m.points_counter + 1;
        }
    }
}

fn view(app: &App, m: &Model, frame: Frame) {
    if (m.redraw) {
        // get canvas to draw on
        let draw = app.draw();
        let win = app.window_rect();
        let time = app.time;

        draw.background().color(BLACK);
        //--------------------------------------------------------
        // background
        // let bg = rgba(0.01, 0.0, app.time.sin()*0.1, 0.1);
        // if app.elapsed_frames() == 10 { //must clear render context once for fullscreen
        // } else {
        //     draw.rect().x_y(0.0, 0.0).w_h(win.w()*2.0, win.w()*2.0).color(bg);
        // }

        // for i in 0..m.rings.len()-1 {
        //     let ring = m.rings[i];
        //     draw.ellipse()
        //     .color(WHITE)
        //     .w_h(ring.z, ring.z)
        //     .x_y(ring.x, ring.y)
        //     ;
        // }

        //--------------------------------------------------------
        let mut n = 0;
        let ring_color = rgba(1.0, 1.0, 1.0, 1.0);

        for i in 0..m.points.len() - 1 {
            if i % m.max_points == 0 {
                let ring = m.rings[n];
                let ring_diam = ring.z * 2.0;
                n = n + 1;

                draw.ellipse()
                    .color(ring_color)
                    .w_h(ring_diam, ring_diam)
                    .x_y(ring.x, ring.y);
            }

            let point = m.points[i];
            let color = m.colors[i];

            draw.ellipse()
                .x_y(point.x, point.y)
                .w_h(point.z, point.z)
                .color(color);
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
    }
}
