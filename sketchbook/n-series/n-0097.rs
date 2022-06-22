use nannou::prelude::*;

//--------------------------------------------------------
static FILENAME : &str = "n-0097";
static CAPTURE  : bool = false;
static FRAME    : bool = false; //hide window chrome when set to false
static SIZE : u32= 800; 

fn main() {
    nannou::app(model).run();
}

struct Model {
    window_id: WindowId
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(SIZE, SIZE)
        .decorations(FRAME) //creates a borderless window
        .view(view)
        .build()
        .unwrap()
        ;

    app.set_loop_mode(LoopMode::loop_once());

    Model {
        window_id
    }
}

fn view(app: &App, m: &Model, frame: Frame) {

    // app.decorations(false);

    // get canvas to draw on
    let draw = app.draw();
    let win = app.window_rect();

    draw.background().color(WHITE);

    // let draw = draw.rotate(PI);

    let radius = 200.0;
    let count = 10000;
    let count2 = 20000;
    let pos = pt2(0.0, 0.0);

    let y_off = 20.0;

    let s = radius + 20.0;

    let rect_height = win.h()/2.0;

    for i in 0..count2 {

       
        let recursive_random = rect_height * random::<f32>() * random::<f32>() * random::<f32>();
        let h = rect_height - recursive_random;

        let x = map_range(i as f32, 0.0, count2 as f32, -win.w()/2.0, win.w()/2.0);
        let y = h;

        draw.ellipse().x_y(x, y + y_off).w_h(2.0, 2.0).color(BLACK);
        // for ii in 0..3 {
        //     let y_off = 50.0 * ii as f32;
        // }
    }


    let rect_points1 = [
        pt2(-s, s),
        pt2(-s, -s),
        pt2(s, -s),
        pt2(s, s),
        pt2(-s, s),
    ];

    draw.rect()
    .x_y(0.0, 100.0)
    .w_h(s*2.0, s*2.0)
    .color(WHITE)
    .stroke_weight(1.0)
    .stroke_color(rgba(0.0, 0.0, 0.0, 0.9))
    ;


    // draw
    // .polyline()
    // .x_y(0.0, 100.0)
    // .stroke_weight(1.0)
    // .color(BLACK)
    // .points(rect_points1)
    // ;

    draw.ellipse()
    .w_h(radius*2.0, radius*2.0)
    .color(WHITE)
    ;

    
    for i in 0..count {

        let angle = map_range(i as f32, 0.0, count as f32, 0.0, PI);

        let r = 1.0 - random::<f32>() * random::<f32>() * random::<f32>() * random::<f32>();

        let x = radius * angle.cos() * r + pos.x;
        let y = radius * angle.sin() * r + pos.y;

        draw.ellipse().x_y(x, y).w_h(2.0, 2.0).color(BLACK);
    }

    
    for i in 0..count {

        let angle = map_range(i as f32, 0.0, count as f32, PI, PI*2.0);

        let r = 1.0 - random::<f32>() * random::<f32>() * random::<f32>() * random::<f32>();

        let x = radius * angle.cos() * r + pos.x;
        let y = radius * angle.sin() * r + pos.y;

        draw.ellipse().x_y(x, y).w_h(2.0, 2.0).color(BLACK);
    }
    
    

    let rw = radius * 1.5;

    let y = y_off + 20.0;
    
    let rect_points2 = [
        pt2(-win.w()/2.0, y),
        pt2(-rw, y),
        pt2(-rw, -rw),
        pt2(rw, -rw),
        pt2(rw, y),
        pt2(win.w()/2.0, y),
    ];

    draw
    .polyline()
    .stroke_weight(2.0)
    .color(BLACK)
    .points(rect_points2)
    ;



    //--------------------------------------------------------

    if CAPTURE {
        let directory = "captures/".to_string();
        let app_name = app.exe_name().unwrap().to_string();
        let extension = ".png".to_string();
        
        let path = format!("{}{}{}", directory, FILENAME, extension);
        println!("Capturing {}", path);
        app.main_window().capture_frame(path);
    }

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();

    
}
