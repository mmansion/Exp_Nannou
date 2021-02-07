use nannou::prelude::*;
use nannou::Draw;
use std::collections::VecDeque;

pub mod lib;

// module tree
use crate::lib::points::Point as Point;
use crate::lib::vehicles::Vehicle as Vehicle;

static CAPTURE  : bool = false; // capture to image sequence
static WIDTH    : i32 = 800;
static HEIGHT   : i32 = 800; 
static DIVS     : i32 = 16;
static MARGIN   : i32 = 100; 
static LINE_LEN : usize = 200;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    debug : bool,
    points: Vec<Point>,
    this_capture_frame : i32,
    last_capture_frame : i32,
    vehicles : Vec<Vehicle>,
}

// ----------------------------------------------------------------------
fn model(app: &App) -> Model {

    let rect = Rect::from_w_h( WIDTH, HEIGHT );

    app.new_window()
        .size(800, 800)
        .view(view)
        .mouse_pressed(mouse_pressed)
        .build()
        .unwrap();

    let debug = false;

    let mut vehicles = Vec::new();
    for i in 0..4 {
        let randX    = random_f32() * (HEIGHT/2) as f32;
        let randY    = random_f32() * (WIDTH/2) as f32;
        let maxspeed = 5.0;
        let velocity = vec2(randX, randY);
        let length = 20;

        vehicles.push( Vehicle::new(0.0, 0.0, maxspeed, velocity.limit_magnitude(maxspeed), length) );
    }

    let mut points  = Vec::new();

    let mut this_capture_frame = 0;
    let mut last_capture_frame = 0;
    //----------------------------------

    for row in 0..(DIVS+1) {

        let y =  ((HEIGHT/DIVS * row) + (-HEIGHT/2)) as f32;

        for col in 0..(DIVS+1) {

            let x =  ( (WIDTH/DIVS  * col) + (-WIDTH/2) ) as f32;
            
            points.push(Point::new(x, y, 1.0, 1.0));
            
        } 
    }
    //----------------------------------


    Model { points, vehicles, this_capture_frame, last_capture_frame, debug}
} 

fn update(app: &App, m: &mut Model, _update: Update) {

    if m.this_capture_frame != m.last_capture_frame {
        m.last_capture_frame = m. this_capture_frame;
    }

    if CAPTURE {
        m.this_capture_frame += 1;
    }

    //----------------------------------

    for v in 0..m.vehicles.len() {

        for i in 0..m.points.len() {

            m.vehicles[v].redirect( &m.points[i] );
            // let steer = force.limit_magnitude(m.vehicles[v].max_force);
            // m.vehicles[v].apply_force(steer);
        }
        
        m.vehicles[v].boundaries(&app.window_rect());
        m.vehicles[v].update();
    }

}

fn view(app: &App, model: &Model, frame: Frame) {

    // get canvas to draw on
    let draw = app.draw();
    let win = app.window_rect();

    let bg = rgba(0.0, 0.0, 0.2, 0.05);
    // draw.background().color( bg );

    //let draw = draw.x_y((-WIDTH/2) as f32, (-HEIGHT/2) as f32);

    let t = app.time;

    if t < 0.1 {
        draw.background().color(BLACK);
    } else {
        //background
        draw.rect()
        .x_y(0.0, 0.0)
        .w_h(win.w()*2.0, win.w()*2.0)
        .color(bg)
        ;
    }

    // let draw = draw.rotate(t * 0.05);

    for i in 0..model.points.len() {

        // println!( "{},{}", model.points[i].x, model.points[i].y );
        let color = hsv( (t * 0.0001 * i as f32).sin(), 1.0, 1.0);
        //let color = hsva ( map_range( abs(app.time.sin() * i as f32 * 0.001), 0.0, 1.0, 0.4, 0.9), 1.0, 1.0, 0.1);

        draw.ellipse()
        .xy(model.points[i].position)
        .radius( model.points[i].size )
        .color(color); 
    }

    // ------------------------------------------------
    let mut pts = Vec::new();
    for v in 0..model.vehicles.len() {

        display(&model.vehicles[v], &draw, &app, v as i32, 0.4);
        pts.push(model.vehicles[v].position);
    }

    draw.polygon()
        .stroke_weight(4.0)
        .caps_round()
        .stroke(PALEGOLDENROD)
        .no_fill()
        .points(pts)
        ;
    
    // ------------------------------------------------

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();

    if model.this_capture_frame != model.last_capture_frame {
        
        // let mut owned_string: String = "hello ".to_owned();
        // let borrowed_string: String = "output/" + app.exe_name().unwrap() + ".png";
    
        let directory  = "captures/".to_string();
        let app_name   = app.exe_name().unwrap().to_string();
        // let frame_num  = model.this_capture_frame.to_string();
        let extension  = ".png".to_string();

        let frame_num = format!("{:05}", model.this_capture_frame);

        let path = format!("{}{}{}", directory, frame_num, extension);

        app.main_window().capture_frame(path);
        
    }
}

fn display(vehicle: &Vehicle, draw: &Draw, app: &App, num:i32, hue:f32) {
    let Vehicle {
        history,
        position,
        velocity,
        ..
    } = vehicle;

    if history.len() > 1 {

        let theta = (velocity.angle() + PI / 2.0) * -1.0;

        let vertices1 = history
            .iter()
            .map(|v| pt2(v.x, v.y))
            .enumerate()
            .map(|(_, p)| {
                let color2 = hsv (0.0, 1.0, 0.0);
                (p, BLACK)
                // let color = hsva ( map_range( abs(app.time.sin() * 0.001 + (num*2) as f32), 0.4, 0.9, 0.3, 0.75), 1.0, 1.0, 1.0);
                
            });
        //draw.polyline().caps_round().weight(12.0).points_colored(vertices1);
        
        let vertices2 = history
            .iter()
            .map(|v| pt2(v.x, v.y))
            .enumerate()
            .map(|(_, p)| {
                let color = hsv (hue, 1.0, 1.0);
                // (p, color)
                (p, PALEGOLDENROD)
            });
            
        draw.polyline().caps_round().weight(5.0).points_colored(vertices2);

        
    }

}

fn mouse_pressed(_app: &App, model: &mut Model, _button: MouseButton) {
    model.debug = !model.debug;
}
