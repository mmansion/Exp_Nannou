use nannou::prelude::*;

// Carry Over Notes:

// [] upgrade and learn ab frame cap -> https://nannou.cc/posts/nannou_v0.13

// -----------------------------------------------------
// CONSTANTS
static _WIDTH_: f32 = 800.0;
static _HEIGHT_: f32 = 800.0;
static _VEC_SIZE_: i32 = 12;
static _VEC_SCALE_: f32 = 1.0;

static NUM_POINTS: i32 = 12;

static SHAPE_SIZE: f32 = 200.0;

// -----------------------------------------------------
fn main() {
    nannou::app(model).update(update).run();
}

// -----------------------------------------------------
// MODEL

struct Model {
    shape_points: Vec<Vector2>, // points bin no.1
    offsets: Vec<Vector2>,
    pbin_2: Vec<Vector2>,
    ibin_1: Vec<f32>,
    ibin_2: Vec<f32>,

    points: [Vector2; 5],
}

fn model(app: &App) -> Model {
    let rect = Rect::from_w_h(_WIDTH_, _HEIGHT_);

    app.new_window()
        .size(rect.w() as u32, rect.h() as u32)
        .view(view)
        .build()
        .unwrap();

    let mut shape_points = Vec::new();
    let mut offsets = Vec::new();

    let mut pbin_2 = Vec::new();
    let mut ibin_1 = Vec::new();
    let mut ibin_2 = Vec::new();

    let mut points = [
        pt2(0.0, 0.0),
        pt2(-SHAPE_SIZE, 0.0),
        pt2(-SHAPE_SIZE, SHAPE_SIZE),
        pt2(0.0, SHAPE_SIZE),
        pt2(0.0, 0.0),
    ];

    for i in 0..NUM_POINTS {
        let x = (i as f32).cos() * SHAPE_SIZE;
        let y = (i as f32).cos() * SHAPE_SIZE;

        shape_points.push(pt2(x, y));
    }

    Model {
        shape_points,
        offsets,
        pbin_2,
        ibin_1,
        ibin_2,
        points,
    }
}

// -----------------------------------------------------

// do calculations here
/*
have a &mut Model in update: that's where you can mutate your data.
You can't do that in view, because it's only a reference, not a mutable one.
This is a design choice from nannou where you can't mutate things when you are drawing them.
Coming from processing it might be hard to adapt to this choice, but it makes things clearer.
*/
fn update(app: &App, m: &mut Model, _update: Update) {

    // for inc in m.ibin_1.iter_mut() {
    //     *inc += 0.008;
    // }
    // for inc in m.ibin_2.iter_mut() {
    //     *inc += 0.002;
    // }
}

// draw outputs here
fn view(app: &App, m: &Model, frame: Frame) {
    let win = app.window_rect();

    // get app time
    let time = app.time;

    // Begin drawing
    let draw = app.draw();

    //let rotate = (time * 0.1).sin() * (m.ibin_1[0]).cos();
    let draw = draw.rotate(time * 0.8);

    // -----------------------------------------------------
    // BACKGROUND

    let mut bg_col = rgba(0.0, 0.0, 0.0, 0.01);

    if time < 0.1 {
        draw.background().color(BLACK);
    } else {
        //background
        draw.rect()
            .x_y(0.0, 0.0)
            .w_h(win.w()*2.0, win.w()*2.0)
            .color(bg_col)
            // .color(BLACK)
            ;
    }
    // -----------------------------------------------------

    // this works too
    // for i in 0..m.shape_points.len() {
    //     println!("{}", m.shape_points[i].x);
    // }

    let point_color_tuples =
        (0..m.shape_points.len()).map(|i| (pt2(m.shape_points[i].x, m.shape_points[i].y), WHITE));

    draw.polyline()
        .weight(2.5)
        //.points(points)
        .points_colored(point_color_tuples);

    // -----------------------------------------------------
    // let size = 250.0;
    // let points = [
    //     pt2(0.0, 0.0),
    //     pt2(-size, 0.0),
    //     pt2(-size, size),
    //     // pt2(0.0, size),
    //     pt2(0.0, 0.0)
    // ];

    // let point_color_tuples = (0..=3).map(|i|{
    //     (points[i], WHITE)
    // });

    // draw.polyline()
    //     .weight(2.5)
    //     //.points(points)
    //     .points_colored(point_color_tuples)
    //     ;

    // -----------------------------------------------------
    // let draw = draw.rotate(time * -1.0);

    // let size = 120.0;
    // let points = [
    //     pt2(0.0, 0.0),
    //     pt2(-size, 0.0),
    //     pt2(-size, size),
    //     // pt2(0.0, size),
    //     pt2(0.0, 0.0)
    // ];

    // let point_color_tuples = (0..=3).map(|i|{
    //     (points[i], BLACK)
    // });

    // draw.polyline()
    //     .weight(2.5)
    //     //.points(points)
    //     .points_colored(point_color_tuples)
    //     ;

    // let shape_points_iter = m.shape_points.iter();
    // let pbin_2_iter = m.pbin_2.iter();

    // let mut ix1 = 0;
    // let mut ix2 = 0;

    // // --------------------------------------
    // // POINTS BIN NO.1

    // let r = 50.0;
    // for p in shape_points_iter {

    //     let inc:f32  = m.ibin_1[ix2]; // get inc for this iteration
    //     let inc2:f32 = m.ibin_2[ix2];

    //     let xOff = (inc + ix2 as f32).cos() * r * -0.1;
    //     let yOff = (inc2 + inc + ix2 as f32).sin() * r + xOff * ix2 as f32;

    //     let xPos = p.x;
    //     let yPos = p.y;

    //     let c1 = hsv(time * 0.01 * ix2 as f32, 1.0, time * 0.1);
    //     let c2 = hsv(time * 0.01, 1.0, 0.5);
    //     let c3 = hsv(1.0, time * 0.1, 0.5);
    //     let c4 = hsv(1.0, time * 0.1, 1.0);

    //     let points_arr1 = [
    //         pt2(xPos, yPos),
    //         pt2(xPos + xOff, yPos),
    //         pt2(xPos + xOff, yPos + yOff * ix2 as f32),
    //         pt2(xPos, yPos + xOff),
    //         pt2(yPos + xOff + xPos, yPos)
    //     ];

    //     let points_arr2 = [
    //         pt2(-xPos, yPos),
    //         pt2(-xPos + yOff, yPos),
    //         pt2(-xPos + (inc + ix2 as f32).cos() * r * -0.1, yPos + yOff),
    //         pt2(-xPos * ix2 as f32, yPos + yOff),
    //         pt2(-yPos + yOff + yOff, yPos + (inc + ix2 as f32).cos() * r * -0.1)
    //     ];

    //     let tuples1 = (0..=4).map(|i|{
    //         (points_arr1[i], c2)
    //     });

    //     let tuples2 = (0..=4).map(|i|{
    //         (points_arr2[i], c2)
    //     });

    //     let fract = ix2 as f32 / _VEC_SIZE_ as f32 * (xOff * 0.5);

    //     draw.scale(fract).polyline()
    //         .weight(fract + 1.0)
    //         .rotate(0.0)
    //         //.points(points)
    //         .points_colored(tuples1)
    //         ;

    //     draw.polyline()
    //         .weight(2.5)
    //         .rotate(180.0)
    //         //.points(points)
    //         .points_colored(tuples2)
    //         ;

    //     ix1+=1; //bump to next inc in vec
    // }

    //----------------------------------------------

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();

    //capture
    // if app.keys.down.contains(&Key::S) {
    //     app.main_window()
    //         .capture_frame(app.exe_name().unwrap() + ".png");
    // }
}
