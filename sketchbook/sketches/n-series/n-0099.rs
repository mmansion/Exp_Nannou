use nannou::prelude::*;

//--------------------------------------------------------
static FILENAME: &str = "n-0099";
static CAPTURE: bool = true;
static FRAME: bool = false; //hide window chrome when set to false

static SIZE: u32 = 800;

fn main() {
    nannou::app(model).run();
}

struct Model {
    window_id: WindowId,
    triangle_a: Point2,
    triangle_b: Point2,
    triangle_c: Point2,
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(SIZE, SIZE)
        .decorations(FRAME) //creates a borderless window
        .mouse_pressed(mouse_pressed)
        .view(view)
        .build()
        .unwrap();

    let win = app.window_rect();

    app.set_loop_mode(LoopMode::loop_once());

    Model {
        window_id,
        triangle_a: pt2(0.0, 0.0),
        triangle_b: pt2(-win.w() * 0.25, -win.h() * 0.35),
        triangle_c: pt2(win.w() * 0.25, -win.h() * 0.35),
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
    let count = 50000;
    let point_size = 1.0;
    let rect_width = 500.0;
    let rect_height = 500.0;
    let rect_pos = pt2(-rect_width*0.5, rect_width*0.5);

    

    let s = win.h()*0.5;
    for i in 0..100 {
        let y = i as f32 * 10.0 - s;
        let alpha = map_range(i, 0, 100, 0.75, 0.0);
        draw.line()
            .color(rgba(0.0, 0.0, 0.0, alpha))
            .stroke_weight(1.0)
            .points( 
                pt2(-rect_width*0.5, y), 
                pt2(rect_width*0.5, y));
                // pt2(win.left(), y), 
                // pt2(win.right(), y));

    }

    draw.tri()
    .stroke_weight(2.0)
    .stroke_color(BLACK)
    .points(m.triangle_a,m.triangle_b,m.triangle_c)
    ;

     for i in 0..count {
        let rrand = rect_height * random::<f32>() * random::<f32>() * random::<f32>();
        let y = rect_height - rrand;
        let x = map_range(
            i as f32,
            0.0,
            count as f32,
            rect_pos.x,
            rect_pos.x + rect_width,
        );
        draw.ellipse()
            .x_y(x, y)
            .w_h(point_size, point_size)
            .color(BLACK);
        // draw.rotate(PI).ellipse()
        //     .x_y(x, y)
        //     .w_h(point_size, point_size)
        //     .color(BLACK);
    }
    

    draw.line()
    .color(rgba(0.0, 0.0, 0.0, 0.4))
    .stroke_weight(1.0)
    .points(
        pt2(-rect_width*0.5, win.top()), 
        pt2(-rect_width*0.5, win.bottom())
    );

    draw.line()
    .color(rgba(0.0, 0.0, 0.0, 0.4))
    .stroke_weight(1.0)
    .points(
        pt2(rect_width*0.5, win.top()), 
        pt2(rect_width*0.5, win.bottom())
    );

    let stipple_height = 200.0;
    let num_stipples = 10000;
    let stipple_size = 2.0;
    
    for i in 0..num_stipples {
        let rrand = stipple_height * random::<f32>() * random::<f32>() * random::<f32>();
        let y = -win.h()*0.35 + rrand;
        let x = map_range(
            i as f32,
            0.0,
            num_stipples as f32,
            rect_pos.x,
            rect_pos.x + rect_width,
        );

        let is_inside = point_in_triangle(pt2(x, y), m.triangle_a, m.triangle_b, m.triangle_c);

        if is_inside {
            draw.ellipse()
                .x_y(x, y)
                .w_h(stipple_size, stipple_size)
                .color(BLACK);
        }
    }

    
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

fn mouse_pressed(_app: &App, _model: &mut Model, _button: MouseButton) {
    let pt = pt2(_app.mouse.x, _app.mouse.y);
    let inside_tri = point_in_triangle(pt, _model.triangle_a, _model.triangle_b, _model.triangle_c);
    println!("({}, {}) => {}", _app.mouse.x, _app.mouse.y, inside_tri)
}
