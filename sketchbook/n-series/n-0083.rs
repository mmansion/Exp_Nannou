use nannou::geom::Point2;
use nannou::geom::*;
use nannou::lyon::path::AttributeStore;
use nannou::prelude::*;
use nannou::Draw;
use nannou_osc as osc;
use std::ops::Range;
use std::time::Duration;

use library::colors::Palette;
use library::line::Line;

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
    receiver: osc::Receiver,
    received_packets: Vec<(std::net::SocketAddr, osc::Packet)>,
    colors: Palette,
    redraw: bool,
    last_redraw: u128,
    lines: Vec<Line>,
    lines_inc: Vec<f32>,
}

//--------------------------------------------------------
fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(WIDTH as u32, HEIGHT as u32)
        .mouse_pressed(mouse_pressed)
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
    let mut redraw = false;
    let mut last_redraw = 0;

    //--------------------------------------------------------

    let colors = Palette::new();

    //--------------------------------------------------------

    let mut lines = Vec::new();
    let mut lines_inc = Vec::new();

    let rand_p1 = vec2(random_range(10.0, 100.0), random_range(10.0, 100.0));
    let rand_p2 = vec2(random_range(10.0, 100.0), random_range(10.0, 100.0));

    lines.push(Line::new(rand_p1, rand_p2));

    //--------------------------------------------------------

    Model {
        window_id,
        this_capture_frame,
        last_capture_frame,
        last_calc,
        receiver,
        received_packets,
        colors,
        redraw,
        last_redraw,
        lines,
        lines_inc,
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

    let ix = m.lines.len() - 1;
    let p1 = m.lines[ix].B;
    let p2 = vec2(
        random_range(p1.x - 10.0, p1.x + 10.0),
        random_range(p1.y - 10.0, p1.y + 10.0),
    );

    m.lines.push(Line::new(p1, p2));
}

fn view(app: &App, m: &Model, frame: Frame) {
    if (m.redraw) {
        // get canvas to draw on
        let draw = app.draw();
        let win = app.window_rect();
        let time = app.time;

        //--------------------------------------------------------
        // background
        let c = m.colors.mango;
        let bg = rgba8(c.red, c.green, c.blue, 15);

        if app.elapsed_frames() < 10 {
            //must clear render context once for fullscreen
            draw.background().color(BLACK);
        } else {
            draw.rect()
                .x_y(0.0, 0.0)
                .w_h(win.w() * 2.0, win.w() * 2.0)
                .color(bg);
        }

        //--------------------------------------------------------
        for line in m.lines.iter() {
            line.draw(&draw);
            // m.lines[0].draw(&draw);
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

fn mouse_pressed(_app: &App, m: &mut Model, _button: MouseButton) {
    let ix = m.lines.len() - 1;
    let p1 = m.lines[ix].B;
    let p2 = vec2(
        random_range(p1.x - 10.0, p1.x + 10.0),
        random_range(p1.y - 10.0, p1.y + 10.0),
    );

    m.lines.push(Line::new(p1, p2));
}
