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
    image: image::DynamicImage,
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

    // load an image
    // Find and return the absolute path to the project’s assets directory
    let assets = app.assets_path().unwrap();


    // build an image path
    let img_path = assets
        .join("images")
        .join("cat.jpg");
    

    //open and create an image buffer

    let image = image::open(img_path).unwrap();

    let img_path = assets
        .join("images")
        .join("cat.jpg");
    

    let texture = wgpu::Texture::from_path(app, img_path).unwrap();

    Model {
        image,
        texture
    }
}


fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    draw.background().color(WHITE);
   
   // draw.texture(&m.texture);

    let tile_count = 800;
    
    let mut colors = Vec::new();
    
    let rect_size = win.w() / tile_count as f32;
    for grid_y in 0..tile_count as usize {
        for grid_x in 0..tile_count as usize {
            let px = grid_x as f32 * rect_size + (rect_size / 2.0);
            let py = grid_y as f32 * rect_size + (rect_size / 2.0);

            // get pixel color (returns 8bit values 0-255)
            let c = m.image.get_pixel(px as u32, py as u32);
            
            //normalize values to 0.0-1.0
            let red   = c[0] as f32 / 255.0;
            let green = c[1] as f32 / 255.0;
            let blue  = c[2] as f32 / 255.0;
            let alpha = c[3] as f32 / 255.0;

            //depending on the value of the pixel, we quantize to either 0 or 1
            let quantized_r = (c[0] as f32 / 255.0).round() * 1.0;
            let quantized_g = (c[1] as f32 / 255.0).round() * 1.0;
            let quantized_b = (c[2] as f32 / 255.0).round() * 1.0;
            // let quantized_red = (c[0] as f32 / 255.0).round() * 1.0;
         

            colors.push(rgba(quantized_r, quantized_g, quantized_b, alpha));
        }
    }

    let mut i = 0;
    for grid_y in 0..tile_count as usize {
        for grid_x in 0..tile_count as usize {
            let pos_x = win.left() + grid_x as f32 * rect_size + (rect_size / 2.0);
            let pos_y = win.top() - grid_y as f32 * rect_size - (rect_size / 2.0);
            draw.rect()
                .x_y(pos_x, pos_y)
                .w_h(rect_size, rect_size)
                .color(colors[i]);
            i += 1;

            // draw.ellipse()
            //     .x_y(pos_x, pos_y)
            //     .w_h(rect_size, rect_size)
            //     .color(colors[i]);
            // i += 1;
        }
    }

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}