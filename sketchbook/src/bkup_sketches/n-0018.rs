use nannou::prelude::*;

// Carry Over Notes: 

// [] upgrade and learn ab frame cap -> https://nannou.cc/posts/nannou_v0.13


// -----------------------------------------------------
// CONSTANTS
static _WIDTH_      : f32 = 800.0;
static _HEIGHT_     : f32 = 800.0;
static _VEC_SIZE_   : i32 = 12;
static _VEC_SCALE_  : f32 = 1.0; 

static NUM_POINTS  : i32 = 60;

static SHAPE_SIZE   :f32  = 100.0;

// -----------------------------------------------------
fn main() {
    nannou::app(model).update(update).run();
}

// -----------------------------------------------------
// MODEL

struct Model {

    shape_points : Vec<Vector2>, // points bin no.1
    offsets : Vec <Vector2>,
    pbin_2  : Vec <Vector2>,
    ibin_1  : Vec <f32>,
    ibin_2  : Vec <f32>,

    rect_points : [Vector2; 5],

    clicked: bool,
    clear_background: bool,
    paused : bool,

    
}

// returns instantiated Model
fn model(app : &App) -> Model {

    let rect = Rect::from_w_h( _WIDTH_, _HEIGHT_ );

    app.new_window()
        .size(rect.w() as u32, rect.h() as u32)
        .view(view)
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .key_pressed(key_pressed)
        .key_released(key_released)
        .build()
        .unwrap();

    let mut shape_points = Vec::new();
    let mut offsets = Vec::new();

    let mut pbin_2  = Vec::new();
    let mut ibin_1  = Vec::new();
    let mut ibin_2  = Vec::new();

    let mut rect_points = [
        pt2(0.0, 0.0),
        pt2(-SHAPE_SIZE, 0.0),
        pt2(-SHAPE_SIZE, SHAPE_SIZE),
        pt2(0.0, SHAPE_SIZE),
        pt2(0.0, 0.0)
    ];

    for i in 0..NUM_POINTS {

        let a = (360 / NUM_POINTS) * i;

        let x = (a as f32).cos() * SHAPE_SIZE;
        let y = (a as f32).cos() * SHAPE_SIZE;

        offsets.push( pt2(0.0, 0.0));
        ibin_1.push(a as f32);

        shape_points.push( pt2(x, y) );
    }

    Model { 
        shape_points, 
        offsets, 
        pbin_2, 
        ibin_1, 
        ibin_2, 
        rect_points,
        clicked: false,
        clear_background: false,
        paused : false,
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
fn update(app: &App, model: &mut Model, _update: Update) {

    // for inc in m.ibin_1.iter_mut() {
    //     *inc += 0.008;
    // }
    // for inc in m.ibin_2.iter_mut() {
    //     *inc += 0.002;
    // }

    for i in 0 .. model.offsets.len() {

        let xOff = (model.ibin_1[i] as f32).cos() * 300.0;
        let yOff = (model.ibin_1[i] as f32).sin() * 100.0;

        model.ibin_1[i] -= 0.02;

        model.offsets[i].x = xOff;
        model.offsets[i].y = yOff;
        // println!("{}", i);
    }

    
}

// draw outputs here
fn view(app: &App, model: &Model, frame: Frame) {

    let win = app.window_rect();

    // get app time
    let time = app.time;

    // Begin drawing
    let draw = app.draw();

    //let rotate = (time * 0.1).sin() * (m.ibin_1[0]).cos();
    let draw = draw.rotate(time * app.time.sin() * 0.1);

    // -----------------------------------------------------
    // BACKGROUND

    let mut bg_col = rgba(0.0, 0.0, 0.0, 0.02);
    let col1 = hsv(time, 1.0, 1.0);
    let col2 = hsva(app.time.sin(), 1.0, 1.0, 0.02 );

    if time < 0.1 {
        draw.background().color(BLACK);

    } else {
        //background
        draw.rect()
            .x_y(0.0, 0.0)
            .w_h(win.w()*2.0, win.w()*2.0)
            .color(bg_col)
            //.color(col2)
            // .color(BLACK)
            ;
    }
    // -----------------------------------------------------
    
    let circle_resolution = map_range( abs(app.time.sin()), 0. , 1. ,3.0 , 12.0) as i32;
    //let radius = app.mouse.x - win.left();
    let radius = model.offsets[0].x + SHAPE_SIZE;
    let angle = TAU / circle_resolution as f32;

    
    if app.elapsed_frames() == 1 || model.clear_background {
        draw.background().color(BLACK);
    }

    let mut points_1 = Vec::new();
    let mut points_2 = Vec::new();
    let mut points_3 = Vec::new();

    for i in 0..circle_resolution {
        let x = (angle * i as f32).cos() * radius;
        let y = (angle * i as f32).sin() * radius;
        points_1.push(pt2(x, y));

        let x = (angle * i as f32).cos() * radius;
        let y = (angle * i as f32).sin() * radius;
        points_2.push(pt2(x, y));

        let x = (angle * model.ibin_1[0] ).cos() * radius/2.0;
        let y = (angle * model.ibin_1[0] ).sin() * radius/2.0;
        points_3.push(pt2(x, y));
    }



    // if  app.time.sin() > 0.0  {
        draw.scale(0.75).polygon()
        //.stroke(rgba(0.0, 0.0, 0.0, 0.1))
        .stroke( col1 )
        .stroke_weight( model.offsets[0].x )
        .no_fill()
        .points(points_1);
    // }


        draw.scale(0.5).polygon()
        //.stroke(rgba(0.0, 0.0, 0.0, 0.1))
        .stroke( BLACK )
        .stroke_weight( model.offsets[0].y )
        .no_fill()
        .points(points_2);

        draw.scale(1.0).polygon()
        //.stroke(rgba(0.0, 0.0, 0.0, 0.1))
        .stroke( BLACK )
        .stroke_weight( model.offsets[0].y )
        .no_fill()
        .points(points_3);


        draw.ellipse()
        .x_y(0.0, 0.0)
        .radius(win.w() * 0.225 * time.sin())
        .no_fill()
        .stroke(BLACK)
        .stroke_weight(10.0)
        ;
    

    // if model.clicked {
    //     draw.polygon()
    //         //.stroke(rgba(0.0, 0.0, 0.0, 0.1))
    //         .stroke( col1 )
    //         .stroke_weight(2.0)
    //         .no_fill()
    //         .points(points);
    // }

    // println!("{}", app.elapsed_frames());

    // println!("{}", app.time.sin());


    //----------------------------------------------

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();

    //capture
    // if app.keys.down.contains(&Key::S) {
    //     app.main_window()
    //         .capture_frame(app.exe_name().unwrap() + ".png");
    // }
}

fn mouse_pressed(_app: &App, model: &mut Model, _button: MouseButton) {
    model.clicked = true;
}
fn mouse_released(_app: &App, model: &mut Model, _button: MouseButton) {
    model.clicked = false;
}
fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Space => {
            model.clear_background = true;
        }
        Key::S => {
            app.main_window()
                .capture_frame(app.exe_name().unwrap() + ".png");
        }
        _other_key => {}
    }
}
fn key_released(_app: &App, model: &mut Model, key: Key) {
    if key == Key::Space {
        model.clear_background = false;
    }
}
