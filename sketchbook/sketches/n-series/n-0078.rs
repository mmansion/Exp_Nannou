use nannou::geom::Point2;
use nannou::geom::*;
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

    theta: f32,
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

    Model {
        window_id,
        this_capture_frame,
        last_capture_frame,
        last_calc,
        receiver,
        received_packets,
        theta: 0.0,
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

    //--------------------------------------------------------
    let win = app.window_rect();

    let x = app.time.sin() * win.w() * 0.5;

    m.theta = map_range(x, win.left(), win.right(), 0.0, PI / 2.0);
}

fn view(app: &App, m: &Model, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();
    let win = app.window_rect();
    let time = app.time;

    //--------------------------------------------------------
    // background

    let bg = rgba(0.1, 0.1, 0.2, 0.01);

    // draw.background().color(bg);

    if app.elapsed_frames() == 10 {
        //must clear render context once for fullscreen
        draw.background().color(rgba(0.0, 0.0, 0.0, 0.9));
    } else {
        draw.rect()
            .x_y(0.0, 0.0)
            .w_h(win.w() * 2.0, win.w() * 2.0)
            .color(bg);
    }

    //--------------------------------------------------------
    let draw = draw.rotate(app.time * 0.25);
    let draw = draw.scale(2.0);
    let draw = draw.scale((app.time * 0.5 * 0.35).sin());

    let length = 120.0;
    branch(&draw, length, m.theta);
    let draw = draw.rotate(PI / 2.0);
    branch(&draw, length, m.theta);
    let draw = draw.rotate(PI / 2.0);
    branch(&draw, length, m.theta);
    let draw = draw.rotate(PI / 2.0);
    branch(&draw, length, m.theta);

    let draw = draw.rotate(app.time * 0.25 * -1.0);
    let draw = draw.scale((app.time * 0.5 * 0.25).sin() * -1.0);

    let length = 120.0;
    branch(&draw, length, m.theta);
    let draw = draw.rotate(PI / 2.0);
    branch(&draw, length, m.theta);
    let draw = draw.rotate(PI / 2.0);
    branch(&draw, length, m.theta);
    let draw = draw.rotate(PI / 2.0);
    branch(&draw, length, m.theta);

    //--------------------------------------------------------

    draw.ellipse().x_y(0.0, 0.0).w_h(200.0, 200.0).color(bg);
    draw.ellipse().x_y(0.0, 0.0).w_h(100.0, 100.0).color(bg);

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

fn branch(draw: &Draw, len: f32, theta: f32) {
    let mut length = len;
    // Each branch will be 2/3rds the size of the previous one
    let mut sw = map_range(length, 1.0, 120.0, 1.0, 10.0);
    let hue = map_range(theta, 0.0, PI * 2.0, 0.0, 1.0) as f32;

    if sw > 3.0 {
        sw = 3.0;
    }
    draw.line()
        .start(pt2(0.0, 0.0))
        .end(pt2(0.0, length))
        .weight(sw)
        .color(hsva(hue, 1.0, 1.0, 0.1));
    // Move to the end of that line
    let draw = draw.x_y(0.0, length);

    length *= 0.36;

    // All recursive functions must have an exit condition!!!!
    // Here, ours is when the length of the branch is 2 pixels or less
    if len > 2.0 {
        let draw2 = draw.rotate(theta); // Save the current state of transformation (i.e. where are we now) and Rotate by theta
        branch(&draw2, length, theta); // Ok, now call myself to draw two new branches!!

        // Repeat the same thing, only branch off to the "left" this time!
        let draw3 = draw.rotate(-theta);
        branch(&draw3, length, theta);
    }
}
