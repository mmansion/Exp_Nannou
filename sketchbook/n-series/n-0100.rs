use nannou::prelude::*;

//--------------------------------------------------------
static FILENAME: &str = "n-0100";
static CAPTURE: bool = true;
static FRAME: bool = true; //hide window chrome when set to false

static SIZE: u32 = 800;

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
        .unwrap();

    let win = app.window_rect();

    app.set_loop_mode(LoopMode::loop_once());

    Model {
        window_id
    }
}


//translated from js:http://jsfiddle.net/mmansion/2hqf1kgv/6/
fn point_in_triangle(pt: Point2, A: Point2, B: Point2, C: Point2) -> bool {
    let a = 0.5 * (-B.y * C.x + A.y * (-B.x + C.x) + A.x * (B.y - C.y) + B.x * C.y);
    let sign = match () {
        _ if a < 0.0 => -1.0,
        _ => 1.0,
    };
    let s = (A.y * C.x - A.x * C.y + (C.y - A.y) * pt.x + (A.x - C.x) * pt.y) * sign;
    let t = (A.x * B.y - A.y * B.x + (A.y - B.y) * pt.x + (B.x - A.x) * pt.y) * sign;

    return s > 0.0 && t > 0.0 && (s+t) < 2.0 * a * sign;
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    draw.background().color(WHITE);

    //--------------------------------------------------------
    
    draw.line()
    .color(BLACK)
    .points(pt2(0.0, win.top()), pt2(0.0, win.bottom()))
    ;

    let n = 4;
    let offset = 100.0;
    let stipple_size = 2.0;
    let count = 10000;
    let draw = draw.translate( pt3(0.0, win.bottom(), 0.0) );
    for i in 0..n {
        let size = win.h()/n as f32;
        let half_size = size * 0.5;
        let y_pos = size * i as f32 + (size * 0.5);
        let mut dir = 1.0;
        if i%2 == 0 {
            dir = -1.0;
        }
        let x_pos = offset*dir;

        draw.rect()
        .x_y(x_pos, y_pos)
        .w_h(size, size)
        .stroke_weight(1.0)
        .stroke_color(BLACK)
        .color(rgba(1.0,1.0, 1.0, 0.1))
        ;

        for i in 0..count {
            let rrand = half_size * random::<f32>() * random::<f32>() * random::<f32>();
            let y = half_size - rrand;
            let x = map_range(
                i as f32,
                0.0,
                count as f32,
                x_pos - half_size,
                x_pos + half_size,
            );
            draw.ellipse()
                .x_y(x, y_pos - y)
                .w_h(stipple_size, stipple_size)
                .color(BLACK);
        }
    } 

    let count2 = 20000;
    for i in 0..count2 {
        let rrand = win.h() * 0.5 * random::<f32>() * random::<f32>() * random::<f32>();
        let y = win.w() * 0.5 - rrand;
        let x = map_range(i as f32, 0.0, count2 as f32, win.left(), win.right());
        draw.rotate(0.0)
            .ellipse()
            .x_y(x, y)
            .w_h(stipple_size, stipple_size)
            .color(BLACK);
    }

    //--------------------------------------------------------

    

    

    
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

