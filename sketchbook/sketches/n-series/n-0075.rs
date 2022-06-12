use nannou::geom::Point2;
use nannou::geom::*;
use nannou::noise::*;
use nannou::prelude::*;
use nannou::Draw;
use nannou_osc as osc;
use std::ops::Range;
use std::time::Duration;

//use library::grid;

//--------------------------------------------------------
static CAPTURE: bool = false; // capture to image sequence (or use obs)
static FRAME: bool = false; //hide window chrome when set to false
static WIDTH: f32 = 800.0;
static HEIGHT: f32 = 800.0;
static BORDER: f32 = 10.0;

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
    receiver: osc::Receiver,
    received_packets: Vec<(std::net::SocketAddr, osc::Packet)>,
    noise: Perlin,
    last_pos: Vec2,
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

    app.set_loop_mode(LoopMode::loop_ntimes(1));

    // Bind an `osc::Receiver` to a port.
    let receiver = osc::receiver(PORT).unwrap();

    // A vec for collecting packets and their source address.
    let received_packets = vec![];

    // app.set_loop_mode(LoopMode::loop_once());
    // app.set_loop_mode(LoopMode::rate_fps(0.1));

    let mut last_calc = Duration::from_millis(0);

    //--------------------------------------------------------
    let mut this_capture_frame = 0;
    let mut last_capture_frame = 0;

    //--------------------------------------------------------

    let noise = Perlin::new();

    let mut last_pos = vec2(WIDTH / 2.0 + BORDER, 0.0);

    //--------------------------------------------------------

    Model {
        window_id,
        this_capture_frame,
        last_capture_frame,
        last_calc,
        receiver,
        received_packets,
        noise,
        last_pos,
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

    //OSC

    // Receive any pending osc packets.
    for (packet, addr) in m.receiver.try_iter() {
        m.received_packets.push((addr, packet));
    }

    //handle received packets
    for &(addr, ref packet) in m.received_packets.iter().rev() {
        println!("{}: {:?}\n", addr, packet);
    }

    while m.received_packets.len() > 0 {
        m.received_packets.remove(0);
    }
}

fn view(app: &App, m: &Model, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();
    let win = app.window_rect();
    let time = app.time;

    //--------------------------------------------------------
    // background

    let bg = rgba(0.0, 0.0, 0.0, 1.0);

    draw.background().color(bg);

    // if app.elapsed_frames() == 10 { //must clear render context once for fullscreen
    //     draw.background().color(rgba(0.0, 0.0, 0.0, 0.9));
    // } else {
    //     draw.rect().x_y(0.0, 0.0).w_h(win.w()*2.0, win.w()*2.0).color(bg);
    // }

    //--------------------------------------------------------

    /*
    let mut last = 0;
    let mut range_y = (win.h()/8.0) as i32;
    let mut range_x = (win.w()/8.0) as i32;

    let x_start = random_f32() * 10.0;
    let mut y_noise = random_f32() * 10.0;

    for y in -range_y..range_y {

        y_noise += 0.02;
        let mut x_noise = x_start;

        for x in -range_x..range_x {
            x_noise += 0.02;

            let n_factor = (m.noise.get( [x_noise as f64, y_noise as f64] ) * 4.0) as f32;

            let draw = draw.translate(
                    pt3((x as f32) * n_factor,
                        (y as f32) * n_factor,
                        (-y as f32)
                    ));

            let edge_size = n_factor * 10.0;

            draw
            .ellipse()
            .x_y(0.0, 0.0)
            .w_h(edge_size, edge_size)
            .stroke_weight(0.5)
            .color(hsl(x_noise, x_noise, 1.0))
            ;


        }
    }
    */

    let from = (-WIDTH / 2.0 + BORDER) as i32;
    let to = (WIDTH / 2.0 - BORDER) as i32;
    let step_x = 10;
    let step_y = 10;

    let mut y_noise = random_f64() * 10.0; //seed
    let variance = 10.0;

    for y_off in ((-HEIGHT / 2.0 + BORDER) as i32..(HEIGHT / 2.0 - BORDER) as i32).step_by(step_y) {
        let mut last_x = from as f32;
        let mut last_y = 0.0;

        for x in (from..to).step_by(step_x) {
            let x = x as f32;
            let n = m.noise.get([0.0 as f64, y_noise]) as f32;
            let y = n * variance + y_off as f32;

            if last_y != 0.0 {
                draw.line()
                    .weight(1.0)
                    .caps_round()
                    .color(rgba(
                        238.0 / 255.0,
                        232.0 / 255.0,
                        170.0 / 255.0,
                        ((y - 400.0) * -1.0 / HEIGHT) as f32,
                    ))
                    .points(pt2(x, y), pt2(last_x, last_y));
            }

            last_x = x;
            last_y = y;

            y_noise += 0.1;
        }
    }

    //--------------------------------------------------------

    let mut last = 0;
    let mut range_y = (win.h() / 8.0) as i32;
    let mut range_x = (win.w() / 8.0) as i32;

    let x_start = random_f32() * 10.0;
    let mut y_noise = random_f32() * 10.0;

    for y in -range_y..range_y {
        y_noise += 0.02;
        let mut x_noise = x_start;

        for x in -range_x..range_x {
            x_noise += 0.02;

            let n_factor = (m.noise.get([x_noise as f64, y_noise as f64]) * 4.0) as f32;

            let draw = draw.translate(pt3(
                (x as f32) * n_factor,
                (y as f32) * n_factor,
                (-y as f32),
            ));

            let edge_size = n_factor * 4.0;

            draw.ellipse()
                .x_y(0.0, 0.0)
                .w_h(edge_size, edge_size)
                .stroke_weight(0.5)
                .stroke(PALEGOLDENROD)
                .color(rgba(
                    238.0 / 255.0,
                    232.0 / 255.0,
                    170.0 / 255.0,
                    (n_factor / 255.0) as f32,
                ));
        }
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
