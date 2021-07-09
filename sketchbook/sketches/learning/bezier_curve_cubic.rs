/*
* bezier_curve_cubic
*
* example of how to draw a cubic bezier curve in nannou
*
* mikhail mansion 2021
*/

use nannou::prelude::*;

//--------------------------------------------------------
static WIDTH      : u32 = 400;
static HEIGHT     : u32 = 400; 
static POINT_SIZE : f32 = 5.0;

//--------------------------------------------------------
fn main() {
    nannou::app(model).update(update).run();
}

//--------------------------------------------------------
struct Model {
    p1 : Point2,
    p2 : Point2,
    cp1 : Point2,
    cp2 : Point2,
    point_active : i32
}

//--------------------------------------------------------
fn model(app: &App) -> Model {

    app.new_window()
        .size(WIDTH, HEIGHT)
        .view(view)
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .build()
        .unwrap();

    let p1  = pt2(-100.0, 0.0);
    let cp1 = pt2(-50.0, 100.0);
    let cp2 = pt2(50.0, 100.0);
    let p2  = pt2(100.0, 0.0);

    let point_active = -1;

    Model { p1, p2, cp1, cp2, point_active }
} 

fn update(app: &App, m: &mut Model, _update: Update) {

    match m.point_active {

        // point 1
        1 => m.p1 = pt2(app.mouse.x, app.mouse.y),
        
        // control point 1
        2 => m.cp1 = pt2(app.mouse.x, app.mouse.y),

        // control point 2
        3 => m.cp2 = pt2(app.mouse.x, app.mouse.y),

        // point 2
        4 => m.p2 = pt2(app.mouse.x, app.mouse.y),

        // Handle the rest of cases
        _ => (),
    }
}

fn view(app: &App, m: &Model, frame: Frame) {

    // get canvas to draw on
    let draw  = app.draw();

    draw.background().color(WHITE);

    
    let builder = nannou::geom::path::Builder::new();

    let path = builder
        .move_to( m.p1 )
        .cubic_bezier_to(m.cp1, m.cp2, m.p2).build();

    // draw the bezier curve path
    draw.path()
        .stroke()
        .weight(2.0)
        .rgba(0.0, 0.0, 0.0, 1.0)
        .events(path.iter());

    // draw line from start to control point 1

    draw.line()
        .weight(1.0)
        .caps_round()
        .color(GRAY)
        .points(m.p1, m.cp1);

    // draw line from start to control point 2

    draw.line()
        .weight(1.0)
        .caps_round()
        .color(GRAY)
        .points(m.cp2, m.p2);

    // draw starting point
    draw.ellipse()
        .xy(m.p1)
        .radius(POINT_SIZE)
        .color(RED);

    // draw control point 1
    draw.ellipse()
        .xy(m.cp1)
        .radius(POINT_SIZE)
        .color(BLUE);

    // draw control point 2
    draw.ellipse()
        .xy(m.cp2)
        .radius(POINT_SIZE)
        .color(BLUE);

    // draw end point
    draw.ellipse()
        .xy(m.p2)
        .radius(POINT_SIZE)
        .color(RED);


    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}

fn mouse_pressed(app: &App, m: &mut Model, btn: MouseButton) {
    
    // println!("({},{})", app.mouse.x, app.mouse.y);

    let mouse = vec2(app.mouse.x, app.mouse.y);

    let p1_v  = mouse - m.p1;
    let p2_v  = mouse - m.p2;
    let cp1_v = mouse - m.cp1;
    let cp2_v = mouse - m.cp2;

    let dist_p1  = p1_v.length();
    let dist_p2  = p2_v.length();
    let dist_cp1 = cp1_v.length();
    let dist_cp2 = cp2_v.length();

    if dist_p1 < POINT_SIZE {
        m.point_active = 1;
    } else 

    if dist_cp1 < POINT_SIZE {
        m.point_active = 2;
    } else 
        
    if dist_cp2 < POINT_SIZE {
        m.point_active = 3;
    } else 

    if dist_p2 < POINT_SIZE {
        m.point_active = 4;
    }
}

fn mouse_released(app: &App, m: &mut Model, btn: MouseButton) {
    m.point_active = -1;
}
