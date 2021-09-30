use nannou::prelude::*;
use nannou::geom::*;
use nannou::geom::Point2;
use nannou_osc as osc;
use std::ops::Range;
use nannou::Draw;
use std::time::Duration;

use library::lines::ContinousLine as ContinousLine;

//--------------------------------------------------------
static CAPTURE  : bool = false; // capture to image sequence (or use obs)
static FRAME    : bool = true; //hide window chrome when set to false
static WIDTH    : f32 = 800.0;
static HEIGHT   : f32 = 800.0; 

// Make sure this matches the `TARGET_PORT` in the `osc_sender.rs` example.
const PORT: u16 = 6555;

//--------------------------------------------------------
fn main() {
    nannou::app(model).update(update).run();
}

//--------------------------------------------------------
struct Model {
    window_id: WindowId,
    this_capture_frame : i32,
    last_capture_frame : i32,
    last_calc : Duration,
    receiver: osc::Receiver,
    received_packets: Vec<(std::net::SocketAddr, osc::Packet)>,

    cont_line: ContinousLine,
    position : Vec2,
    speed : f32, //speed
    x_dir: f32,
    y_dir: f32,
}

//--------------------------------------------------------
fn model(app: &App) -> Model {

    let window_id = app
        .new_window()
        .size(WIDTH as u32, HEIGHT as u32)
        .decorations(FRAME) //creates a borderless window
        .view(view)
        .build()
        .unwrap()
        ;

    // Bind an `osc::Receiver` to a port.
    let receiver = osc::receiver(PORT).unwrap();

    // A vec for collecting packets and their source address.
    let received_packets = vec![];
    
    // app.set_loop_mode(LoopMode::loop_once());
    // app.set_loop_mode(LoopMode::rate_fps(0.1));
    
    let mut last_calc = Duration::from_millis(0);

    //--------------------------------------------------------
   
    let mut position = vec2(0.0, 0.0);
    let mut speed = 1.0;
    let mut x_dir = 1.0;
    let mut y_dir = 1.0;

    let mut cont_line = ContinousLine::new( position );

    cont_line.stroke = rgba(1.0, 1.0, 1.0, 1.0);

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
        position,
        speed,
        x_dir,
        y_dir,
        cont_line,
    }
} 

fn update(app: &App, m: &mut Model, _update: Update) {

    // ref:
    //https://doc.rust-lang.org/nightly/core/time/struct.Duration.html
    //let millis = Duration::from_millis(100).as_millis();
    let since_last_calc = _update.since_start.as_millis() - m.last_calc.as_millis();
    if since_last_calc > 10  { //time interval
        m.last_calc = _update.since_start;
    }

    if m.this_capture_frame != m.last_capture_frame {
        m.last_capture_frame = m. this_capture_frame;
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
    // vertices

    let win   = app.window_rect();

    m.position.x += (m.speed * m.x_dir);
    m.position.y += (m.speed * m.y_dir);

    let mut v = vec2( m.position.x, m.position.y);

    v.x += (app.time).cos() * 100.0;
    v.y += (app.time).sin() * (app.time * 250.0).sin() * (app.time * 0.1).sin() * random_range(1.0, 100.0);
    // v.y += (app.time * 0.1).sin() * random_range(1.0, 100.0);

    m.cont_line.history.push(v);

    if(m.position.x > win.w()/2.0 || m.position.x < -win.w()/2.0 ) {
        m.x_dir *= -1.0;
    }
    
    if(m.position.y > win.h()/2.0 || m.position.y < -win.h()/2.0 ) {
        m.y_dir *= -1.0;
    }

    m.cont_line.weight=  (app.time * 0.1).sin() + 1.0;
    let dist = (v - vec2(0.0, 0.0)).length();

    m.cont_line.stroke = rgba(
        map_range(dist, 1.0, win.w()/4.0, 1.0, 0.0), 
        map_range(dist, 1.0, win.w()/4.0, 1.0, 0.0),
        map_range(dist, 1.0, win.w()/4.0, 1.0, 0.0), 
        1.0);
    // println!("{}", )
 
}

fn view(app: &App, m: &Model, frame: Frame) {

    // get canvas to draw on
    let draw  = app.draw();
    let win   = app.window_rect();
    let time  = app.time;

    //--------------------------------------------------------
    // background

    let bg = rgba(0.13, 0.0, 0.1, 0.01);

    if app.elapsed_frames() == 10 { //must clear render context once for fullscreen
        draw.background().color(rgba(0.0, 0.0, 0.0, 0.9));
    } else {
        draw.rect().x_y(0.0, 0.0).w_h(win.w()*2.0, win.w()*2.0).color(bg);
    }

    //--------------------------------------------------------

    let draw = draw.rotate( app.time * 0.05);
    
    //--------------------------------------------------------
    
    m.cont_line.display(&draw);

    //--------------------------------------------------------
    // draw frame
    
    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();

    //--------------------------------------------------------
    // capture frame

    if m.this_capture_frame != m.last_capture_frame {      
        let directory  = "captures/".to_string();
        let app_name   = app.exe_name().unwrap().to_string();
        let extension  = ".png".to_string();
        let frame_num  = format!("{:05}", m.this_capture_frame);

        let path = format!("{}{}{}", directory, frame_num, extension);
        app.main_window().capture_frame(path);
    }
}