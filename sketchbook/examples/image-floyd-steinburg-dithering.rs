use library::matrix::Matrix;
use nannou::draw::properties::color;
use nannou::prelude::*;
use nannou::image;
use nannou::image::GenericImageView;
use library::matrix;

//--------------------------------------------------------
static FILENAME: &str = "image-basics";
static FRAME: bool = true; //hide window chrome when set to false
static SIZE: u32 = 800;

static TILE_COUNT: usize = 400;


fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    image  : image::DynamicImage,
    points : Matrix<Point2>,
    pixels : Matrix<Rgba>
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

    let rect_size = win.w() / TILE_COUNT as f32;

    let mut points: Matrix<Point2> = Matrix::new( Vec::new() );
    let mut pixels: Matrix<Rgba> = Matrix::new(Vec::new());

    for col in 0..TILE_COUNT {

        let mut pts = Vec::new();
        let mut cols = Vec::new();

        for row in 0..TILE_COUNT {
    
            let x = win.left() + row as f32 * rect_size + (rect_size / 2.0);
            let y = win.top() - col as f32 * rect_size - (rect_size / 2.0);

            pts.push(pt2(x, y));

            //create a color variable for each point
            let color:Rgba = Rgba::new(1.0, 1.0, 1.0, 1.0);

            cols.push(color);


        }
        points.data.push(pts);
        pixels.data.push(cols);
    }

   

    Model {
        image,
        points,
        pixels
    }
}

fn update(app: &App, m: &mut Model, _update: Update) {
    let win = app.window_rect();
    let rect_size = win.w() / TILE_COUNT as f32;
    // let mut colors = Vec::new();

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

    for col in 0..TILE_COUNT as usize { //from top to bottom
        for row in 0..TILE_COUNT as usize {
            let x = (row as f32 * rect_size + (rect_size / 2.0)) as u32;
            let y = (col as f32 * rect_size + (rect_size / 2.0)) as u32;
            let ux = x as usize;
            let uy = y as usize;

            //old pixel
            let old_pixel = m.image.get_pixel(x, y);

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
            let error_a = old_a - new_a;

            m.pixels.data[col][row] = rgba(new_r, new_g, new_b, new_a);

            if col + 1 < TILE_COUNT { //check boundaries
                
                //let color:Rgba = Rgba::new( m.pixels.data[col + 1][row]);
                let color = m.pixels.data[col + 1][row] as Rgba; //right
                let final_r = color.red   + error_r * 7.0/16.0;
                let final_g = color.green + error_g * 7.0/16.0;
                let final_b = color.blue  + error_b * 7.0/16.0;
                let final_a = color.alpha + error_a * 7.0/16.0;
                
                m.pixels.data[col + 1][row] = rgba(final_r, final_g, final_b, final_a)
                
            } 

            // 
           

            // let px2 = m.pixels.data[ux - 1][uy + 1];
            // let px3 = m.pixels.data[ux    ][uy + 1];
            // let px3 = m.pixels.data[ux + 1][uy + 1];

            // let quantized_r = (old_pixel[0] as f32 / 255.0).round() * 1.0;
            // let quantized_g = (old_pixel[1] as f32 / 255.0).round() * 1.0;
            // let quantized_b = (old_pixel[2] as f32 / 255.0).round() * 1.0;
            // let quantized_a = (old_pixel[3] as f32 / 255.0).round() * 1.0;
         

            // colors.push(rgba(quantized_r, quantized_g, quantized_b, quantized_a));
            

            // colors.push(rgba(new_r, new_g, new_b, new_a));
            // let quantized_g = (c[1] as f32 / 255.0).round() * 1.0;
            // let quantized_b = (c[2] as f32 / 255.0).round() * 1.0;

        }
    }
}
// fn index(x:f32, y:f32) -> 

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    draw.background().color(WHITE);



    let mut i = 0;
     let rect_size = win.w() / TILE_COUNT as f32;
    for col in 0..TILE_COUNT as usize {
        for row in 0..TILE_COUNT as usize {
            // let pos_x = win.left() + row as f32 * rect_size + (rect_size / 2.0);
            // let pos_y = win.top() - col as f32 * rect_size - (rect_size / 2.0);
            // println!("{}, {}", pos_x, pos_y);

    
            draw.rect()
                .xy(m.points.data[col][row])
                .w_h(rect_size, rect_size)
                .color(m.pixels.data[col][row]);
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