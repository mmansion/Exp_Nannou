use nannou::prelude::*;

// Carry Over Notes: 

// [] upgrade and learn ab frame cap -> https://nannou.cc/posts/nannou_v0.13

static _WIDTH_       : f32 = 800.0;
static _HEIGHT_      : f32 = 800.0;
static _VEC_SIZE_    : i32 = 90;
static _VEC_SCALE_   : f32 = 1.75; 

fn main() {
    nannou::app(model).update(update).run();
}

// -------------------------------------------------------------------

struct Model {
    pbin_1 : Vec<Vector2>, // points bin no.1
    pbin_2 : Vec<Vector2>,
    ibin_1 : Vec<f32>,
    ibin_2 : Vec<f32>
}

fn model(app: &App) -> Model {

    let rect = Rect::from_w_h( _WIDTH_, _HEIGHT_ );

    app.new_window()
        .size(rect.w() as u32, rect.h() as u32)
        .view(view)
        .build()
        .unwrap();

    let mut pbin_1  = Vec::new();
    let mut pbin_2  = Vec::new();
    let mut ibin_1  = Vec::new();
    let mut ibin_2  = Vec::new();

    for i in 0.._VEC_SIZE_ {

        let x = random_range(-rect.w()/2.0 * _VEC_SCALE_, rect.w()/2.0 * _VEC_SCALE_);
        let y = random_range(-rect.h()/2.0 * _VEC_SCALE_, rect.h()/2.0 * _VEC_SCALE_);

        //space the increments out numerically on init
        ibin_1.push(i as f32 * random_f32()); 
        ibin_2.push(i as f32 * random_f32());

        pbin_1.push(pt2(x, y));
        pbin_2.push(pt2(y, x)); //flip
    }

    Model { pbin_1, pbin_2, ibin_1, ibin_2 }
}    

// do calculations here 
/*
have a &mut Model in update: that's where you can mutate your data. 
You can't do that in view, because it's only a reference, not a mutable one. 
This is a design choice from nannou where you can't mutate things when you are drawing them. 
Coming from processing it might be hard to adapt to this choice, but it makes things clearer.
*/
fn update(app: &App, m: &mut Model, _update: Update) {

    for inc in m.ibin_1.iter_mut() {
        *inc += 0.001;
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
    
    let draw = draw.rotate(time * -0.02);

    // clear the bg
    let mut bg_col = rgba(0.0, 0.0, 0.0, 0.1);
    let mut fg_col = rgba(1.0, 1.0, 1.0, 0.1);

    if time < 0.1 {

        draw.background().color(BLACK);

    } else {
        //background
        draw.rect()
            .x_y(0.0, 0.0)
            .w_h(win.w()*2.0, win.w()*2.0)
            // .color(bg_col)
            .color(BLACK)
            ;
    }
    // --------------------------------------
    // drawing vars

    let radius = 600.0;

    // --------------------------------------
    // let circle_col = rgba(0.0,1.0,1.0,0.1);
    // draw.ellipse()
    //     .x_y(0.0,0.0)
    //     .radius(radius)
    //     .color(circle_col);

    // --------------------------------------

    let pbin_1_iter = m.pbin_1.iter();
    let pbin_2_iter = m.pbin_2.iter();

    let mut ix1 = 0;
    let mut ix2 = 0;

    // --------------------------------------
    // POINTS BIN NO.1

    for p in pbin_1_iter {

        let inc:f32 = m.ibin_1[ix1];

        let transform = inc.sin() * 10.0;

        let xOff = (inc).cos() * radius;
        let yOff = (inc).sin() * radius;

        ix1+=1; //bump to next inc in vec

        let color = hsv(time * 0.05, 1.0, 1.0);

        draw.line()
        .start(pt2(yOff, xOff))
        .end(pt2(xOff * random_f32(), yOff* random_f32()))
        .weight(1.4)
        .color(color)
        //.color(bg_col)
        ;

        let points = (0..=144).map(|i| {

    
            // Convert each degree to radians.
            let radian = deg_to_rad(i as f32);

            // Get  sine of the radian to find the x co-ordinate of this point of the circle
            // and multiply it by the radius.
            let x = radian.sin() * xOff;

            // Do the same with cosine to find the y co-ordinate.
            let y = radian.cos() * radius;

            // Construct and return a point object with a color.
            (pt2(x,y), BLACK) // speed of color transition

        });
    
        //println!("{}, {}", p.x, p.y);
        draw.polyline()
            .weight(random_f32())
            .rotate(transform)
            .points_colored(points)
            ;
        
    
        let circle_col = rgba(0.0,1.0,1.0,0.01);
        draw.ellipse()
            .x_y(xOff, yOff)
            .radius(radius)
            .stroke(BLACK)
            .color(circle_col)
        ;
        draw.rect()
            .x_y(yOff, xOff)
            .w(xOff * ix1 as f32)
            .color(BLACK)
            ;
    }

    // --------------------------------------
    // POINTS BIN NO.2

    for p in pbin_2_iter {

        let r = 200.0;

        let inc:f32 = m.ibin_1[ix2];

        let transform = inc.sin() * 10.0;

        let xOff = (inc * ix2 as f32).cos() * r;
        let yOff = (inc * ix2 as f32).sin() * r;

        ix2+=1; //bump to next inc in vec

        let color = hsv(time * 0.1, 1.0, 1.0);

        draw.line()
            .start(pt2(xOff, yOff))
        // .end(pt2(xOff * random_f32(), yOff* random_f32()))
            .end(pt2(yOff,xOff))
            .weight(1.4)
            .color(color)
        //.color(bg_col)
        ;

        let pbin_1 = (0..=144).map(|i| {

            // Convert each degree to radians.
            let radian = deg_to_rad(i as f32);

            // Get  sine of the radian to find the x co-ordinate of this point of the circle
            // and multiply it by the radius.
            let x = radian.sin() * radius + xOff;

            // Do the same with cosine to find the y co-ordinate.
            let y = radian.cos() * radius;

            // Construct and return a point object with a color.
            (pt2(x,y), color) // speed of color transition

        });
    
        // Create an `ngon` of points.
        let n_points = 5;
        let radius = 80.0;
        let points = (0..n_points).map(|i| {
            let fract = i as f32 / n_points as f32;
            let phase = fract;
            let x = radius * (TAU * phase * ix2 as f32).cos() * _VEC_SCALE_;
            let y = radius * (TAU * phase * ix2 as f32).sin() * _VEC_SCALE_;
            pt2(x, y)
        });

        draw.polygon()
            // .x_y(yOff * ix2 as f32, yOff * ix2 as f32)
            .no_fill()
            // .color(WHITE)
            .rotate(transform)
            .stroke(fg_col)
            .stroke_weight(3.0)
            .join_round()
            .points(points);

        // draw.rect()
        //     .x_y(xOff, yOff)
        //     .w(xOff)
        //     .rotate(transform * -1.0)
        //     .color(fg_col)
        //     ;
    }

    //---------------------------------------------------------



    //----------------------------------------------

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();

    //capture
    // if app.keys.down.contains(&Key::S) {
    //     app.main_window()
    //         .capture_frame(app.exe_name().unwrap() + ".png");
    // }
}
