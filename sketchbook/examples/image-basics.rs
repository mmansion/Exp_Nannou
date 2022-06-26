use nannou::prelude::*;
use nannou::image;
use nannou::image::GenericImageView;

//--------------------------------------------------------
static FILENAME: &str = "image-basics";
static FRAME: bool = true; //hide window chrome when set to false
static SIZE: u32 = 800;

fn main() {
    nannou::app(model).run();
}

struct Model {
    texture: wgpu::Texture,
}

fn model(app: &App) -> Model {
    app
    .new_window()
    .size(SIZE, SIZE)
    .decorations(FRAME) //creates a borderless window
    .view(view)
    .build()
    .unwrap();

    let win = app.window_rect();

    app.set_loop_mode(LoopMode::loop_once());
    //--------------------------------------------------------

    // Find and return the absolute path to the project’s assets directory
    let assets = app.assets_path().unwrap();

    //construct the path
    let img_path = assets.join("images").join("cat.jpg");
    
    //load the texture/image
    let texture = wgpu::Texture::from_path(app, img_path).unwrap();

    Model {
        texture
    }
}


fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();

    draw.background().color(WHITE);
   
    draw.texture(&m.texture);

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}