use nannou::{prelude::*, color::white_point::F2};

//--------------------------------------------------------
static FILENAME: &str = "n-0108";
static CAPTURE: bool = false;
static FRAME: bool = false; //hide window chrome when set to false

static SIZE: u32 = 800;

fn main() {
    nannou::app(model).run();
}

struct Model {
    window_id: WindowId,
    wave_points: Vec<Vec2>,
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

    let mut wave_points = Vec::new();
    let s = SIZE as i32;
    let start = -s/2;
    let end = s/2;
    for i in start..end {
        let degree = map_range(i, start, end, -720, 720) as i32;
        let x = i as f32;
        let y = ((degree as f32).to_radians()).sin() * (SIZE/2) as f32;
        wave_points.push(pt2(x, y));
    }

    Model {
        window_id,
        wave_points
    }
}

fn view(app: &App, m: &Model, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();
    let win = app.window_rect();
    
    draw.background().color(WHITE);
    // let draw = draw.rotate(-PI/4.0);
    
    let stipple_size = 1.0;

    //iterate through sine wave points
    for wave_point in m.wave_points.iter() { 

        // set min/max ranges for y, based on window height
        let min_range = -win.h()/2.0;
        let max_range = win.h()/2.0;

        // set density of points based on current y value of sine wave
        let density:i32 = map_range(wave_point.y, min_range, max_range, 100, 1000);

        // choose a number of random y values based on density
        for d in 0..density {
            let rand_y = random_range(min_range, max_range);

            // draw a point at the random y value and x value of current sine wave point
            draw.ellipse().x_y(wave_point.x, rand_y).w_h(stipple_size, stipple_size).color(GRAY);

        }
    

        // draw.ellipse()
        //     .x_y(wave_point.x, wave_point.y)
        //     .w_h(2.0, 2.0)
        //     .color(BLACK);
    }

    draw.polyline()
        .weight(5.0)
        .points(m.wave_points.clone())
        .color(WHITE);
    

  

    // //draw a polyline sine wave
    // let mut points = Vec::new();
    // let s = SIZE as i32;
    // let start = -s/2;
    // let end = s/2;
    // for i in start..end {
    //     let degree = map_range(i, start, end, -360, 360) as i32;
    //     let x = i as f32;
    //     let y = ((degree as f32).to_radians()).sin() * (SIZE/4) as f32;
    //     points.push(pt2(x, y));
    // }
    // draw.polyline()
    //     .weight(1.0)
    //     .points(points)
    //     .color(BLACK);
    

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}
