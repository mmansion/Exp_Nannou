use std::os::unix::process;

use nannou::{prelude::*, color::white_point::F2};

//--------------------------------------------------------
static FILENAME: &str = "n-0109";
static CAPTURE: bool = true;
static FRAME: bool = true; //hide window chrome when set to false

static SIZE: u32 = 800;

fn main() {
    nannou::app(model).run();
}

struct Model {
    window_id: WindowId,
    outer_circle_points: Vec<Vec2>,
    inner_circle_points: Vec<Vec2>,
    center_circle_points: Vec<Vec2>,
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(SIZE, SIZE)
        .decorations(FRAME) //creates a borderless window
        .view(view)
        .build()
        .unwrap();
    

    app.set_loop_mode(LoopMode::loop_once());


    let mut center_circle_points = Vec::new();
    let mut inner_circle_points = Vec::new();
    let mut outer_circle_points = Vec::new();

    let s = SIZE as f32 * 0.25;
    let n = 360;


    for i in 0..n {
        let degree = i * (360/n);
        let x = ((degree as f32).to_radians()).cos() * s;
        let y = ((degree as f32).to_radians()).sin() * s;
        center_circle_points.push(pt2(x, y));

        let x_outer = ((degree as f32).to_radians()).cos() * (s * 2.0);
        let y_outer = ((degree as f32).to_radians()).sin() * (s * 2.0);
        outer_circle_points.push(pt2(x_outer, y_outer));

        let x_inner = ((degree as f32).to_radians()).cos() * (s * 0.5);
        let y_inner = ((degree as f32).to_radians()).sin() * (s * 0.5);
        inner_circle_points.push(pt2(x_inner, y_inner));
    }

    Model {
        window_id,
        center_circle_points,
        inner_circle_points,
        outer_circle_points
    }
}

fn view(app: &App, m: &Model, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();
    let win = app.window_rect();
    
    draw.background().color(WHITE);
    // let draw = draw.rotate(-PI/4.0);
    

    for i in 0..m.center_circle_points.len() {
        // println!( "{},{}", m.inner_circle_points[i].x, m.inner_circle_points[i].y );

        // draw.line()
        //     .start(pt2(m.inner_circle_points[i].x, m.inner_circle_points[i].y))
        //     .end(pt2(m.outer_circle_points[i].x, m.outer_circle_points[i].y))
        //     .color(BLACK)
        //     .weight(0.5);

        // get random points along the line
        for r in 0..1000 {

            // weight the random value based on how far away from center_circle_point
            
            // let random = random_range(0.0, 1.0);
            let random = weighted_random(0.0, 1.0, 3);
            
            
            // let random = rng.gen_range(0.0, 1.0);
            let x = map_range(random, 0.0, 1.0, m.inner_circle_points[i].x, m.outer_circle_points[i].x);
            let y = map_range(random, 0.0, 1.0, m.inner_circle_points[i].y, m.outer_circle_points[i].y);
            let pt = pt2(x, y);
            
            // get the distance from the center
            let total_dist = m.center_circle_points[i].distance(m.outer_circle_points[i]);
            let dist = m.center_circle_points[i].distance(pt);
            
            //create a color based on the dist 
            let color = rgba(0.0, 0.0, 0.0, map_range(dist, 0.0, total_dist, 9.0, 0.0));
            let stipple_size = map_range(dist, 0.0, total_dist, 4.0, 0.0);

            draw.ellipse()
                .x_y(x, y)
                .w_h(stipple_size, stipple_size)
                .color(color);
        }
    }

    

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();

    if CAPTURE {      
        let directory  = "captures/".to_string();
        // let app_name   = app.exe_name().unwrap().to_string();
        let extension  = ".png".to_string();
        let filename   = FILENAME.to_string();

        let path = format!("{}{}{}", directory, filename, extension);
        app.main_window().capture_frame(path);
    }
}

//https://stackoverflow.com/questions/30492259/get-a-random-number-focused-on-center
fn weighted_random(min:f32, max:f32, weight:u32) -> f32 {
    let mut num = 0.0;
    for i in 0..weight {
        num += random_range(min, max) * (max/weight as f32);
    }  
    return num;
}