// use nannou::lyon::path::AttributeStore;
use nannou::prelude::*;
use std::any::type_name;
fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}
// use nannou::geom::*;
// use nannou::geom::Point2;
use nannou_osc as osc;
// use std::ops::Range;
// use nannou::Draw;
use std::time::Duration;

use library::colors::Palette;
// use library::line::Line;
use library::grid2::Grid2 as Grid;

// beginning of touch library for nannou
use library::touchosc::TouchOscFader as Fader;

//--------------------------------------------------------
static CAPTURE  : bool = false; // capture to image sequence (or use obs)
static FRAME    : bool = true; //hide window chrome when set to false
static WIDTH    : f32 = 800.0;
static HEIGHT   : f32 = 800.0; 
static BORDER   : f32 = 10.0;
static WAIT     : u128 = 100;

static NUM_SLIDERS  : usize  = 4; //num of sliders used 

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
    colors:Palette,
    bg_color:Rgb8,
    redraw:bool,
    last_redraw: u128,
    shape_size: f32,
    rot_speed: f32,
    faders: Vec<Fader>,
    grid: Grid
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
    let mut this_capture_frame = 0;
    let mut last_capture_frame = 0;

    //--------------------------------------------------------
    let mut redraw = false;
    let mut last_redraw = 0;

    //--------------------------------------------------------

    let colors = Palette::new();
    let bg_color = colors.get_random();

    let rot_speed = 0.0;
    let shape_size = 10.0;

    let faders = Vec::new();

    for x in 0..NUM_SLIDERS {
        faders.push(Fader::new())
    }

    //--------------------------------------------------------
    let rect = Rect::from_w_h( WIDTH, HEIGHT );
    let mut grid = Grid::new(10, 10, 10, 10, &rect);

    //--------------------------------------------------------

    Model {
        window_id,
        this_capture_frame, 
        last_capture_frame, 
        last_calc,
        receiver,
        received_packets,
        colors,
        bg_color,
        redraw,
        last_redraw,
        rot_speed,
        shape_size,
        sliders,
        grid
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
    // redraw framerate workaround
    // change WAIT to increase interval

    if _update.since_start.as_millis() - m.last_redraw > WAIT {
        m.last_redraw = _update.since_start.as_millis();
        m.redraw = true;
    } else {
        m.redraw = false;
    }
    //--------------------------------------------------------

    //OSC

    // Receive any pending osc packets.
    for (packet, addr) in m.receiver.try_iter() {
         //m.received_packets.push((addr, packet));

        let mut val = 0.0;


        for msg in packet.into_msgs() {
            let args = msg.args.unwrap();

            for i in m.sliders.iter_mut() {
                match (&msg.addr[..], &args[..]) {
                    ("/fader1", [osc::Type::Float(i)]) => sliders = *i,
                }
            // loop code here
            }


                

                for i in 0..sliders.len() {
                // loop code here
                }
                
                // ("/button2", [osc::Type::Float(x), osc::Type::Float(y)]) => println!("{:?}", (x, y)),
                _etc => (),
            }
        }

        println!("{}", val);

        if val > 0.0 {
            m.bg_color = m.colors.get_random();
            m.rot_speed = random_range(10., 100.);
            m.shape_size = random_range(10., 100.);
        }
    }

    

    //handle received packets
    for &(addr, ref packet) in m.received_packets.iter().rev() {
       //println!("{}: {:?}\n", addr, packet);    
    }


    while m.received_packets.len() > 0 {
        //m.received_packets.remove(0);
    }

    //--------------------------------------------------------
 
 
}

fn view(app: &App, m: &Model, frame: Frame) {

    if(m.redraw) {

        // get canvas to draw on
        let draw  = app.draw();
        let win   = app.window_rect();
        let time  = app.time;
    
        //--------------------------------------------------------
        // background
        // let c = m.colors.mango;
        // let c = m.colors.get_random();
        // let bg = rgba8(c.red, c.green, c.blue, 15);
    
        if app.elapsed_frames() < 10 { //must clear render context once for fullscreen
            draw.background().color(BLACK);
        } else {
            draw.rect().x_y(0.0, 0.0).w_h(win.w()*2.0, win.w()*2.0).color(m.bg_color);
        }
        
        //--------------------------------------------------------
        m.grid.draw(&draw);

        for i in 0..m.grid.points.len() {

            let rotation = m.grid.angles[i];
            let position =  m.grid.points[i];

            //let d = draw.rotate( rotation.angle()  );
            let _draw = draw.translate(pt3(position.x, position.y, 0.0));

            _draw.line().points( pt2(0.0, 0.0), rotation);

            _draw.rect()
            .w_h(m.shape_size, m.shape_size)
            .rotate(m.rot_speed)
            .xy(position)
            .stroke_weight(10.0)
            .color(BLACK)
            ;
        }
            
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

}

fn mouse_pressed(_app: &App, m: &mut Model, _button: MouseButton) {

    
}