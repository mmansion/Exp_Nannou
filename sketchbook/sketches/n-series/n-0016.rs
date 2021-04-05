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
}

fn model(app : &App) -> Model {

    let rect = Rect::from_w_h( _WIDTH_, _HEIGHT_ );

    app.new_window()
        .size(rect.w() as u32, rect.h() as u32)
        .view(view)
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

    Model { shape_points, offsets, pbin_2, ibin_1, ibin_2, rect_points }
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



    for i in 0..m.offsets.len() {

        let xOff = (m.ibin_1[i] as f32).cos() +  m.shape_points[i].x * 3.0;
        let yOff = (m.ibin_1[i] as f32).sin() +  m.shape_points[i].y;

        m.ibin_1[i] -= 0.1;

        m.offsets[i].x = xOff;
        m.offsets[i].y = yOff;
        // println!("{}", i);
    }

    
}

// draw outputs here
fn view(app: &App, m: &Model, frame: Frame) {

    let win = app.window_rect();

    // get app time
    let time = app.time;

    // Begin drawing
    let draw = app.draw();

    //let rotate = (time * 0.1).sin() * (m.ibin_1[0]).cos();
    let draw = draw.rotate(time * 0.1);

    // -----------------------------------------------------
    // BACKGROUND

    let mut bg_col = rgba(0.0, 0.0, 0.0, 0.002);
    let col1 = hsv(1.0, time, 1.0);

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
    let deadZoneRadius = 10.0;

    for i in 0..m.shape_points.len() {

        let x = m.shape_points[i].x + m.offsets[i].x;
        let y = m.shape_points[i].y + m.offsets[i].y;
        let c = hsv(time * 1.1, 1.0, 1.0);

        //let draw = draw.rotate(time * random_f32() * 1.1);

        if(abs(x) > deadZoneRadius && abs(y) > deadZoneRadius) {
            // draw.ellipse()
            // .x_y(x, y * random_f32())
            // .radius(win.w() * time.sin() * 0.01)
            // .rotate(time * -0.9)
            // .color(c);

            draw.scale(win.w() * time.sin() * 0.01).rect()
            .x_y(x + y, y * random_f32())
            .w(win.w() * time.cos() * 0.01)
            .h(win.h() * time.sin() * 0.01)
            .stroke_weight(1.0)
            .color(c);
        }

        // ----------------------------
        
        let rect_tuples = (0..m.rect_points.len()).map(|i|{
            let c = hsv(time * 2.1, 1.0, 1.0);
            let x = m.rect_points[i].x;
            let y = m.rect_points[i].y;
            ( pt2(x, y), c)
        });
    
        draw.scale(0.25)
            .polyline()
            .weight(1.0)
            .rotate(time)
            //.points(points)
            .points_colored(rect_tuples)
            ;

        // ----------------------------

        let rect_tuples = (0..m.rect_points.len()).map(|i|{
            let c = hsv(time * 0.1, 1.0, 0.1);
            let x = m.rect_points[i].x;
            let y = m.rect_points[i].y;
            ( pt2(x, y), c)
        });
        
    
        // draw.scale(1.0)
        //     .polyline()
        //     .weight(4.0)
        //     .rotate(time * TAU)
        //     .weight(2.0)
        //     //.points(points)
        //     .points_colored(rect_tuples)
        //     ;
        // ----------------------------
        
    }



    draw.ellipse()
        .x_y(0.0, 0.0)
        .radius(200.0)
        .color(BLACK);

    // -----------------------------------------------------

    let point_color_tuples = (0..m.shape_points.len()).map( |i| {

        let c = hsv(1.0, 1.0, 1.0);
    
        // let x = m.shape_points[i].x + m.offsets[i].x;
        // let y = m.shape_points[i].x + m.offsets[i].y;
        
        let scale = 0.5;
        let x = scale * (map_range(i, 0, NUM_POINTS as usize - 1, win.left(), win.right()) + m.offsets[i].x);
        let fract = i as f32 / NUM_POINTS as f32;
        let amp = (time + fract * TAU).sin();
        let y = scale * (map_range(amp, -1.0, 1.0, win.bottom() * 0.75, win.top() * 0.75) + m.offsets[i].y);
        
        ( pt2( -x, -y ), col1 )
        //( pt2(m.shape_points[i].x, m.shape_points[i].y), c)

    });

   

    // draw.scale(0.9)
    //     .polyline()
    //     .weight(2.0)
    //     //.points(points)
    //     .points_colored(point_color_tuples)
    //     ;


    // -----------------------------------------------------

    let draw = draw.rotate(time * -0.9);

    // for i in 0..m.offsets.len() {

    //     let c = hsv(time * 0.01, 0.2, 0.1);
    //     let t = app.time;

    //     let x = m.offsets[i].x;
    //     let y = m.offsets[i].y;
        

    //     draw.ellipse()
    //         .x_y(x, y)
    //         .radius(win.w() * 0.125 * t.sin())
    //         .color(c);
    // }
  
    // let point_color_tuples = (0..m.shape_points.len()).map( |i| {

    //     let c = hsv(time * 0.1, 1.0, 1.0);
    
    //     // let x = m.shape_points[i].x + m.offsets[i].x;
    //     // let y = m.shape_points[i].x + m.offsets[i].y;
        
    //     let scale = 0.5;
    //     let x = scale * (map_range(i, 0, NUM_POINTS as usize + 1, win.right(), win.left()) + m.offsets[i].x);
    //     let fract = i as f32 / NUM_POINTS as f32;
    //     let amp = (time + fract * TAU).sin();
    //     let y = scale * (map_range(amp, -1.0, 1.0, win.top() * 0.75, win.bottom() * 0.75) + m.offsets[i].y);
        
    //     ( pt2( -x, -y ), col1 )
    //     //( pt2(m.shape_points[i].x, m.shape_points[i].y), c)

    // });


    // draw.scale(0.5)
    //     .polyline()
    //     .weight(1.0)
    //     //.points(points)
    //     .points_colored(point_color_tuples)
    //     ;


    // -----------------------------------------------------
    /*
    let rect_tuples = (0..m.rect_points.len()).map(|i|{
        let c = hsv(time * 1.1, 1.0, 1.0);
        let x = m.rect_points[i].x + m.offsets[i].x;
        let y = m.rect_points[i].y + m.offsets[i].y;
        ( pt2(x, y), c)
    });

    draw.scale(0.5)
        .polyline()
        .weight(1.0)
        //.points(points)
        .points_colored(rect_tuples)
        ;

   
    */

    // -----------------------------------------------------




    //----------------------------------------------

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();

    //capture
    // if app.keys.down.contains(&Key::S) {
    //     app.main_window()
    //         .capture_frame(app.exe_name().unwrap() + ".png");
    // }
}
