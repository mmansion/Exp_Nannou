use nannou::prelude::*;
use nannou::image;
use nannou::image::GenericImageView;

//--------------------------------------------------------
static FILENAME: &str = "image-basics";
static FRAME: bool = true; //hide window chrome when set to false
static SIZE: u32 = 800;

static TILE_SIZE: usize = 400;


fn main() {
    nannou::app(model).run();
}

struct Model {
    image  : image::DynamicImage,
    pixels : Array
}
/*
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
 */

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


    let mut pixels = vec![vec!['#'; 800]; 800];

    Model {
        image,
        pixels
    }
}

// fn index(x:f32, y:f32) -> 

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    draw.background().color(WHITE);

    //TODO:
    //https://en.wikipedia.org/wiki/Floyd%E2%80%93Steinberg_dithering
    //https://www.youtube.com/watch?v=0L2n8Tg2FwI


    // floyd-steinburg algorithm
    /*
    for each y from top to bottom do
    for each x from left to right do
        oldpixel := pixels[x][y]
        newpixel := find_closest_palette_color(oldpixel)
        pixels[x][y] := newpixel
        quant_error := oldpixel - newpixel
        pixels[x + 1][y    ] := pixels[x + 1][y    ] + quant_error × 7 / 16
        pixels[x - 1][y + 1] := pixels[x - 1][y + 1] + quant_error × 3 / 16
        pixels[x    ][y + 1] := pixels[x    ][y + 1] + quant_error × 5 / 16
        pixels[x + 1][y + 1] := pixels[x + 1][y + 1] + quant_error × 1 / 16
    */

    let tile_count = 10;
    let rect_size = win.w() / tile_count as f32;
    let mut colors = Vec::new();


    for grid_y in 0..tile_count as usize { //from top to bottom
        for grid_x in 0..tile_count as usize {
            let px = (grid_x as f32 * rect_size + (rect_size / 2.0)) as u32;
            let py = (grid_y as f32 * rect_size + (rect_size / 2.0)) as u32;

            //old pixel
            let old_pixel = m.image.get_pixel(px, py);

            let old_r = old_pixel[0] as f32;
            let old_g = old_pixel[1] as f32;
            let old_b = old_pixel[2] as f32;
            let old_a = old_pixel[3] as f32;

            let factor = 1.0;

            let new_r = (factor * old_r / 255.0).round() * (1.0 / factor);
            let new_g = (factor * old_g / 255.0).round() * (1.0 / factor);
            let new_b = (factor * old_b / 255.0).round() * (1.0 / factor);
            let new_a = (factor * old_a / 255.0).round() * (1.0 / factor);

            let error_r = old_r - new_r;
            let error_g = old_g - new_g;
            let error_b = old_b - new_b;

            // let quantized_r = (old_pixel[0] as f32 / 255.0).round() * 1.0;
            // let quantized_g = (old_pixel[1] as f32 / 255.0).round() * 1.0;
            // let quantized_b = (old_pixel[2] as f32 / 255.0).round() * 1.0;
            // let quantized_a = (old_pixel[3] as f32 / 255.0).round() * 1.0;
         

            // colors.push(rgba(quantized_r, quantized_g, quantized_b, quantized_a));
            
            colors.push(rgba(new_r, new_g, new_b, new_a));
            // let quantized_g = (c[1] as f32 / 255.0).round() * 1.0;
            // let quantized_b = (c[2] as f32 / 255.0).round() * 1.0;

        }
    }



    
    // let rect_size = win.w() / tile_count as f32;
    // for grid_y in 0..tile_count as usize {
    //     for grid_x in 0..tile_count as usize {
    //         let px = grid_x as f32 * rect_size + (rect_size / 2.0);
    //         let py = grid_y as f32 * rect_size + (rect_size / 2.0);

    //         // get pixel color (returns 8bit values 0-255)
    //         let c = m.image.get_pixel(px as u32, py as u32);
            
    //         //normalize values to 0.0-1.0
    //         let red   = c[0] as f32 / 255.0;
    //         let green = c[1] as f32 / 255.0;
    //         let blue  = c[2] as f32 / 255.0;
    //         let alpha = c[3] as f32 / 255.0;

    //         //depending on the value of the pixel, we quantize to either 0 or 1
    //         let quantized_r = (c[0] as f32 / 255.0).round() * 1.0;
    //         let quantized_g = (c[1] as f32 / 255.0).round() * 1.0;
    //         let quantized_b = (c[2] as f32 / 255.0).round() * 1.0;
    //         // let quantized_red = (c[0] as f32 / 255.0).round() * 1.0;
         

    //         colors.push(rgba(quantized_r, quantized_g, quantized_b, alpha));
    //     }
    // }

    let mut i = 0;
    for grid_y in 0..tile_count as usize {
        for grid_x in 0..tile_count as usize {
            let pos_x = win.left() + grid_x as f32 * rect_size + (rect_size / 2.0);
            let pos_y = win.top() - grid_y as f32 * rect_size - (rect_size / 2.0);
            println!("{}, {}", pos_x, pos_y);
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