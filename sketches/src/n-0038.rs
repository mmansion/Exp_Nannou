use nannou::prelude::*;
use nannou::Draw;
use std::collections::VecDeque;

pub mod lib;

// module tree
use crate::lib::points::Point as Point;
use crate::lib::vehicles::Vehicle as Vehicle;

static CAPTURE  : bool = true; // capture to image sequence
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
        .build()
        .unwrap();

    let debug = false;

    let mut vehicles = Vec::new();
    for i in 0..9 {
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
            let size = 3.0;
            points.push(Point::new(x, y, 1.0, size));
            
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
        
        m.vehicles[v].boundaries2(&app.window_rect(), MARGIN + 20);
        m.vehicles[v].update();
    }

}

fn view(app: &App, model: &Model, frame: Frame) {

    // get canvas to draw on
    let draw = app.draw();
    let win = app.window_rect();

    let bg = rgba(0.0, 0.0, 0.2, 0.4);
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

    let color1 = hsva ( 0.4, 1.0, 1.0, 1.0);
    let color2 = hsva ( 1.0, 1.0, 1.0, 0.0);
    let color3 = hsva ( 0.2, 1.0, 1.0, 0.8);

    

    for i in 0..model.points.len() {

        draw.ellipse()
        .xy(model.points[i].position)
        .radius( model.points[i].size )
        .color(color1); 

        // draw.rect()
        // .no_fill()
        // .xy(model.points[i].position)
        // .w_h(model.points[i].size, model.points[i].size)
        // .stroke(WHITE); 
    }

    // ------------------------------------------------

   
   

    // ------------------------------------------------
    let mut pts1 = Vec::new();
    let mut pts2 = Vec::new();
    let mut pts3 = Vec::new();

    for v in 0..model.vehicles.len() {

        let points = (0..=360).map(|i| {    
            
            let radian = deg_to_rad(i as f32); 
            let x = radian.sin() * 50.0;
            let y = radian.cos() * 50.0;
            pt2(x,y)              
         });


         draw
        .polygon()
        .stroke_weight(6.0)
        .caps_round()
        .stroke(color1)
        // .color(color3)
        .no_fill()
        .points(points)
        ;

         {
            let points = (0..=360).map(|i| {    
                let radian = deg_to_rad(i as f32); 
                let x = radian.sin() * 25.0;
                let y = radian.cos() * 25.0;
                pt2(x,y)              
             });
             draw
            .polyline() 
            .xy(model.vehicles[v].position)
            .weight(3.0)
            .points(points)
            .color(color3)
            ; 
            if v < 3 {
                pts1.push(model.vehicles[v].position);
            } else if v < 6 {
                pts2.push(model.vehicles[v].position);
                    
            } else if v < 9 {
                pts3.push(model.vehicles[v].position);
            }
         }

        
    }

    

    draw
    .polygon()
    .stroke_weight(2.0)
    .caps_round()
    .stroke(color1)
    .color(color2)
    .points(pts1)
    ;

    draw
    .polygon()
    .stroke_weight(2.0)
    .caps_round()
    .stroke(color1)
    .color(color2)
    .points(pts2)
    ;

    draw
    .polygon()
    .stroke_weight(2.0)
    .caps_round()
    .stroke(color1)
    .color(color2)
    .points(pts3)
    ;


    // ------------------------------------------------
    let mut frame_points = Vec::new();

    frame_points.push(vec2( (-WIDTH/2 + MARGIN) as f32, (-HEIGHT/2 + MARGIN) as f32 ));
    frame_points.push(vec2( (WIDTH/2 - MARGIN) as f32, (-HEIGHT/2 + MARGIN) as f32 ));
    frame_points.push(vec2( (WIDTH/2 - MARGIN) as f32, (HEIGHT/2 - MARGIN) as f32));
    frame_points.push(vec2( (-WIDTH/2 + MARGIN) as f32, (HEIGHT/2 - MARGIN) as f32));

    draw
    .polygon()
    .stroke_weight(2.0)
    .caps_round()
    .no_fill()
    .stroke(color1)
    .points(frame_points)
    ;
    
    

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