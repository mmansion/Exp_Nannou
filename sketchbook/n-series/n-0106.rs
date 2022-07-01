use nannou::prelude::*;

//--------------------------------------------------------
static FILENAME: &str = "n-0106";
static CAPTURE: bool = true;
static FRAME: bool = false; //hide window chrome when set to false

static SIZE: u32 = 800;

use library::math::point_inside_triangle as point_in_triangle;
use library::texture::StippleRect as StippleRect;


fn main() {
    nannou::app(model).run();
}

struct Model {
    window_id: WindowId,
    stippleRects:Vec<StippleRect>,
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

    let side = win.w() * 0.25;
    let side2 = win.w() * 0.35;

    //--------------------------------------------------------
    let angles = 8;
    let radius = 200.0;
    let rect_size =143.0;

    let mut stippleRects = Vec::new();

    // for i in 0..angles {
    //     let angle = deg_to_rad((365 / angles * i) as f32);

    //     let x = angle.cos() * radius;
    //     let y = angle.sin() * radius;

    //     stippleRects.push(
    //         StippleRect {
    //         position: pt2(x, y),
    //         c1 : pt2(-rect_size, rect_size),
    //         c2 : pt2(rect_size, -rect_size),
    //     })
    // }

    stippleRects.push(
        StippleRect {
            position: pt2(-rect_size, 0.0),
            c1: pt2(-rect_size, rect_size),
            c2: pt2(rect_size, -rect_size)
        });

    stippleRects.push(
        StippleRect {
            position: pt2(rect_size, 0.0),
            c1: pt2(-rect_size, rect_size),
            c2: pt2(rect_size, -rect_size)
        });

    Model {
        window_id,
        stippleRects
    }
}


fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    draw.background().color(WHITE);
    // let draw = draw.rotate(-PI/2.0);

    //--------------------------------------------------------


    let angles = 181;
    let radius = 30.0;
    let rect_size = 299.0;
    
    for a in 0..angles {
        let angle = deg_to_rad((365 / angles * a) as f32);

        let x = angle.cos() * radius;
        let y = angle.sin() * radius;

        draw
        .translate(pt3(0.0, 0.0, -a as f32))
        .rotate(angle)
        .rect()
        .x_y(x, y)
        .w_h(rect_size-a as f32, rect_size-a as f32)
        .stroke_weight(1.0)
        ;
    }
        let s = win.w()*0.5;
    for i in 0..100 {
        let x = i as f32 * 10.0 - s;
        let alpha = map_range(abs(x), 10.0, win.w()*0.5, 0.0, 0.45);
        let weight = map_range(abs(x), 10.0, win.w()*0.5, 0.0, 10.0);
        draw
            .rotate(PI)
            .translate(pt3(0.0, 0.0, 0.0))
            .line()
            .color(rgba(0.0, 0.0, 0.0, alpha))
            .stroke_weight(weight)
            .points( 
                pt2(x, win.h() * 0.5), 
                pt2(x, -win.w() * 0.5));
                // pt2(win.left(), y), 
                // pt2(win.right(), y));

    }
    for i in 0..m.stippleRects.len() {
        let _stippleRect = &m.stippleRects[i];
        // stippleRect.outline(&draw, 0.6, true);
        // stippleRect.left(&draw, 50000, 1.0);
        if i % 2 == 0 {
            // _stippleRect.outline(&draw, 0.6, true);
            _stippleRect.left(&draw, 50000, 1.0);
        } else {
            // _stippleRect.outline(&draw, 0.6, true);
            _stippleRect.right(&draw, 50000, 1.0);
        }
        //stippleRect.right(&draw, 50000, 1.0);
    }

    
    // m.stipple.outline(&draw, 0.25, true);
    // m.stipple.left(&draw, 50000, 1.0);


    // // m.stipple.outline(&draw, 0.6, true);
    // m.stipple2.left(&draw, 10000, 1.0);



    // let points = [
    //     pt2(-win.w()*0.35, win.h() * 0.35),
    //     pt2(win.w()*0.35, win.h() * 0.35),
    //     pt2(win.w()*0.35, -win.h() * 0.35),
    //     pt2(-win.w()*0.35, -win.h() * 0.35),
    // ];

    // draw
    // .polyline()
    // .stroke_weight(0.6)
    // .caps_round()
    // .color(rgba(0.0, 0.0, 0.0, 1.0))
    // .points_closed(points)
    // ;
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