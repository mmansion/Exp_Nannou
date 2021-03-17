use nannou::prelude::*;

// Carry Over Notes: 

// [] upgrade and learn ab frame cap -> https://nannou.cc/posts/nannou_v0.13

static _WIDTH_  : f32  = 800.0;
static _HEIGHT_ : f32  = 800.0;

fn main() {

    nannou::app(model).update(update).run();
}

// -------------------------------------------------------------------

struct Model {

    points  : Vec<Vector2>,
    incs    : Vec<f32>
}

fn model(app: &App) -> Model {

    let rect = Rect::from_w_h( _WIDTH_, _HEIGHT_ );

    app.new_window()
        .size(rect.w() as u32, rect.h() as u32)
        .view(view)
        .build()
        .unwrap();

    let mut points = Vec::new();
    let mut incs   = Vec::new();

    for i in 0..10 {

        //let x = random_f32() * rect.w();
        // let y = random_f32() * rect.h();

        let x = random_range(-rect.w()/2.0, rect.w()/2.0);
        let y = random_range(-rect.h()/2.0, rect.h()/2.0);

      
        incs.push(i as f32 * random_f32()); //space the increments out numerically on init

        points.push(pt2(x, y))
    }

    Model { points, incs }
}    

// do calculations here 
/*
have a &mut Model in update: that's where you can mutate your data. 
You can't do that in view, because it's only a reference, not a mutable one. 
This is a design choice from nannou where you can't mutate things when you are drawing them. 
Coming from processing it might be hard to adapt to this choice, but it makes things clearer.
*/
fn update(app: &App, m: &mut Model, _update: Update) {

    for inc in m.incs.iter_mut() {
        *inc += 0.1;
    }
}

// draw outputs here
fn view(app: &App, m: &Model, frame: Frame) {

    let win = app.window_rect();

    // get app time
    let time = app.time;

    // Begin drawing
    let draw = app.draw();

    // clear the bg
    let mut bg_col = rgba(0.0, 0.0, 0.0, 0.01);
    let mut fg_col = rgba(1.0, 1.0, 1.0, 0.1);

    if time < 0.1 {
        println!("time: {}", time);
        draw.background().color(BLACK);

    } else {
        //background
        draw.rect()
            .x_y(0.0, 0.0)
            .w_h(win.w()*2.0, win.w()*2.0)
            .color(bg_col)
            ;
    }

    // --------------------------------------
    let p_iter = m.points.iter();
    let mut ix = 0;
    for p in p_iter {

        let inc:f32 = m.incs[ix];

        let transform = inc.sin() * 100.0;

        ix+=1;
        

        //println!("{}, {}", p.x, p.y);

        draw.rect()
            .x_y(p.x + transform, p.y)
            .w(transform)
            .rotate(transform)
            .hsv(time, 1.0, 1.0);
    }

    // --------------------------------------
    
    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();

    // capture
    // if app.keys.down.contains(&Key::S) {
    //     app.main_window()
    //         .capture_frame(app.exe_name().unwrap() + ".png");
    // }
}
