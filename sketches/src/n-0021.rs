use nannou::prelude::*;

static CELL_SIZE  : f32 = 10.0;

fn main() {
    nannou::sketch(view).run();
}

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


    for i in 0.._VEC_SIZE_ {

       
    }

    Model {  }
} 

fn update(app: &App, m: &mut Model, _update: Update) {

  
}

fn view(app: &App, m: &Model, frame: Frame) {

    // get canvas to draw on
    let draw = app.draw();

    // set background to blue
    draw.background().color(BLUE);

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}