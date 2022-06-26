use nannou::prelude::*;
use nannou::image;

fn main() {

    let mut array = [[0u8; 10]; 10];

    println!("array length = {}", array.len());

    array[0][0] = 42;
    array[3][3] = 9;

    println!("array[0][0] = {}", array[0][0]);

    match array.get_mut(3*3) { //check if out of bounds
        Some(x) => { 
            println!("array[3][3] = {}", array[3][3]);
        }
        None => { println!("oops, out of bounds"); }
    }

    //--------------------------------------------------------

    nannou::sketch(view)
        .size(800, 800)
        .loop_mode(LoopMode::loop_once())
        .run();
}


fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    draw.to_frame(app, &frame).unwrap();
}
