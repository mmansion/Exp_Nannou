use nannou::prelude::*;

mod _vehicle;

static CAPTURE  : bool = true; // capture to image sequence
static WIDTH    : i32 = 800;
static HEIGHT   : i32 = 800; 
static DIVS     : i32 = 16;
static MARGIN   : i32 = 50; 
static LINE_LEN : usize = 10;

fn main() {
    nannou::app(model).update(update).run();
}

// -------------------------------------------------------------------

struct Model {

}

fn model(app: &App) -> Model {

    let rect = Rect::from_w_h( WIDTH, HEIGHT );

    app.new_window()
        .size(rect.w() as u32, rect.h() as u32)
        .view(view)
        .build()
        .unwrap();

    Model {  }
}    

// do calculations here 
/*
have a &mut Model in update: that's where you can mutate your data. 
You can't do that in view, because it's only a reference, not a mutable one. 
This is a design choice from nannou where you can't mutate things when you are drawing them. 
Coming from processing it might be hard to adapt to this choice, but it makes things clearer.
*/
fn update(app: &App, m: &mut Model, _update: Update) {

 
}

// draw outputs here
fn view(app: &App, m: &Model, frame: Frame) {

    let win = app.window_rect();

    // get app time
    let time = app.time;

    // Begin drawing
    let draw = app.draw();

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
            .color(bg_col)
            ;
    }

    // --------------------------------------
    
    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
