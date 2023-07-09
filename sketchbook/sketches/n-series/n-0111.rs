// A demonstration of drawing to a very large texture, capturing the texture in its original size
// as a PNG and displaying a down-scaled version of the image within the window each frame.

use nannou::prelude::*;
static FILENAME: &str = "n-0111";
static CAPTURE: bool = true;
// static SIZE: u32 = 16384;
static SIZE: u32 = 2160;

fn main() {
    nannou::app(model).update(update).exit(exit).run();
}

struct Model {
    window_id: WindowId,
    outer_circle_points: Vec<Vec2>,
    inner_circle_points: Vec<Vec2>,
    center_circle_points: Vec<Vec2>,
    captured_complete: bool,

    // The texture that we will draw to.
    texture: wgpu::Texture,
    // Create a `Draw` instance for drawing to our texture.
    draw: nannou::Draw,
    // The type used to render the `Draw` vertices to our texture.
    renderer: nannou::draw::Renderer,
    // The type used to capture the texture.
    texture_capturer: wgpu::TextureCapturer,
    // The type used to resize our texture to the window texture.
    texture_reshaper: wgpu::TextureReshaper,
}

fn model(app: &App) -> Model {
    // Lets write to a 4K UHD texture.
    let texture_size = [SIZE, SIZE];

    // Create the window.
    let [win_w, win_h] = [800, 800];
    let window_id = app
        .new_window()
        .size(win_w, win_h)
        .title("nannou")
        .view(view)
        .build()
        .unwrap();

  
    let window = app.window(window_id).unwrap();

    // Retrieve the wgpu device.
    let device = window.device();

    // Create our custom texture.
    let sample_count = window.msaa_samples();
    let texture = wgpu::TextureBuilder::new()
        .size(texture_size)
        // Our texture will be used as the RENDER_ATTACHMENT for our `Draw` render pass.
        // It will also be SAMPLED by the `TextureCapturer` and `TextureResizer`.
        .usage(wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING)
        // Use nannou's default multisampling sample count.
        .sample_count(sample_count)
        // Use a spacious 16-bit linear sRGBA format suitable for high quality drawing.
        .format(wgpu::TextureFormat::Rgba16Float)
        // Build it!
        .build(device);

    // Create our `Draw` instance and a renderer for it.
    let draw = nannou::Draw::new();
    let descriptor = texture.descriptor();
    let renderer =
        nannou::draw::RendererBuilder::new().build_from_texture_descriptor(device, descriptor);

    // Create the texture capturer.
    let texture_capturer = wgpu::TextureCapturer::default();

    // Create the texture reshaper.
    let texture_view = texture.view().build();
    let texture_sample_type = texture.sample_type();
    let dst_format = Frame::TEXTURE_FORMAT;
    let texture_reshaper = wgpu::TextureReshaper::new(
        device,
        &texture_view,
        sample_count,
        texture_sample_type,
        sample_count,
        dst_format,
    );

    // Make sure the directory where we will save images to exists.
    

    if CAPTURE {      
        let dir     = "captures/".to_string();
        let sub_dir = FILENAME.to_string();
        let path = format!("{}/{}", dir, sub_dir);
        // app.main_window().capture_frame(path);
        std::fs::create_dir_all(path).unwrap();
    }

    let captured_complete = false;

    //--------------------------------------------------------
    let mut center_circle_points = Vec::new();
    let mut inner_circle_points = Vec::new();
    let mut outer_circle_points = Vec::new();

    let s = SIZE as f32 * 0.25;
    let n = 360;


    for i in 0..360 {
        let degree = i * (360/n);
    
        let x = ((degree as f32).to_radians()).cos() * s;
        let y = ((degree as f32).to_radians()).sin() * s;
        center_circle_points.push(pt2(x, y));

        let x_outer = ((degree as f32).to_radians()).cos() * (s * 2.0);
        let y_outer = ((degree as f32).to_radians()).sin() * (s * 2.0);
        outer_circle_points.push(pt2(x_outer, y_outer));

        let x_inner = ((degree as f32).to_radians()).cos() * (s * 0.5);
        let y_inner = ((degree as f32).to_radians()).sin() * (s * 0.5);
        inner_circle_points.push(pt2(x_inner, y_inner));
        
    }

    Model {
        window_id,
        outer_circle_points,
        inner_circle_points,
        center_circle_points,
        texture,
        draw,
        renderer,
        texture_capturer,
        texture_reshaper,
        captured_complete
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {

    if model.captured_complete {
        return;
    } else {
        // First, reset the `draw` state.
        let draw = &model.draw;
        draw.reset();
    
        // Create a `Rect` for our texture to help with drawing.
        let [w, h] = model.texture.size();
        let r = geom::Rect::from_w_h(w as f32, h as f32);
    
        // Use the frame number to animate, ensuring we get a constant update time.
        let elapsed_frames = app.main_window().elapsed_frames();
        let t = elapsed_frames as f32 / 60.0;
    
        // Draw like we normally would in the `view`.
        //--------------------------------------------------------
        draw.background().color(WHITE);
        // let draw = draw.rotate(-PI/4.0);
    
        for i in 0..model.center_circle_points.len() {
            // println!( "{},{}", model.inner_circle_points[i].x, model.inner_circle_points[i].y );

            // draw.line()
            //     .start(pt2(model.inner_circle_points[i].x, model.inner_circle_points[i].y))
            //     .end(pt2(model.outer_circle_points[i].x, model.outer_circle_points[i].y))
            //     .color(BLACK)
            //     .weight(0.5);

            // get random points along the line
            for r in 0..1000 {

                // weight the random value based on how far away from center_circle_point
                
                // let random = random_range(0.0, 1.0);
                let random = weighted_random(0.0, 1.0, 3);
                
                // let random = rng.gen_range(0.0, 1.0);
                let x = map_range(random, 0.0, 1.0, model.inner_circle_points[i].x, model.outer_circle_points[i].x);
                let y = map_range(random, 0.0, 1.0, model.inner_circle_points[i].y, model.outer_circle_points[i].y);
                let pt = pt2(x, y);
                
                // get the distance from the center
                let total_dist = model.center_circle_points[i].distance(model.outer_circle_points[i]);
                let dist = model.center_circle_points[i].distance(pt);
                
                //create a color based on the dist 
                // let color = rgba(0.0, 0.0, 0.0, map_range(dist, 0.0, total_dist, 9.0, 0.0));
                let gray_scale = map_range(dist, 0.0, total_dist, 0.0, 5.0);
                let gray = rgba(gray_scale, gray_scale, gray_scale, 1.0);
                let stipple_size = map_range(dist, 0.0, total_dist, 140.0, 0.0);
                println!("{} ", stipple_size);

                draw.ellipse()
                    .x_y(x, y)
                    .w_h(stipple_size, stipple_size)
                    .color(gray);
                
            }
        }
    

    
        // Render our drawing to the texture.
        let window = app.main_window();
        let device = window.device();
        let ce_desc = wgpu::CommandEncoderDescriptor {
            label: Some("texture renderer"),
        };
        let mut encoder = device.create_command_encoder(&ce_desc);
        model
            .renderer
            .render_to_texture(device, &mut encoder, draw, &model.texture);
    
        // Take a snapshot of the texture. The capturer will do the following:
        //
        // 1. Resolve the texture to a non-multisampled texture if necessary.
        // 2. Convert the format to non-linear 8-bit sRGBA ready for image storage.
        // 3. Copy the result to a buffer ready to be mapped for reading.
        let snapshot = model
            .texture_capturer
            .capture(device, &mut encoder, &model.texture);
    
        // Submit the commands for our drawing and texture capture to the GPU.
        window.queue().submit(Some(encoder.finish()));
    
        // Submit a function for writing our snapshot to a PNG.
        //
        // NOTE: It is essential that the commands for capturing the snapshot are `submited before we
        // attempt to read the snapshot - otherwise we will read a blank texture!
        let path = capture_directory(app)
            .join(elapsed_frames.to_string())
            .with_extension("png");
        snapshot
            .read( move |result| {
                let image = result.expect("failed to map texture memory").to_owned();
                image.save(&path).expect("failed to save texture to png image");
                println!("Captured frame {} to {:?}", elapsed_frames, path);
            })
            .unwrap();
        model.captured_complete = true;
    }
   
}

// Draw the state of your `Model` into the given `Frame` here.
fn view(_app: &App, model: &Model, frame: Frame) {
    // Sample the texture and write it to the frame.
    let mut encoder = frame.command_encoder();
    model.texture_reshaper.encode_render_pass(frame.texture_view(), &mut *encoder);
}

// Wait for capture to finish.
fn exit(app: &App, model: Model) {
    println!("Waiting for PNG writing to complete...");
    let window = app.main_window();
    let device = window.device();
    model
        .texture_capturer
        .await_active_snapshots(&device)
        .unwrap();
    println!("Done!");
}

// The directory where we'll save the frames.
fn capture_directory(app: &App) -> std::path::PathBuf {
    let dir     = "captures/".to_string();
    let sub_dir = FILENAME.to_string();
    let path = format!("{}/{}", dir, sub_dir);

    app.project_path()
        .expect("could not locate project_path")
        .join(path)
}
//https://stackoverflow.com/questions/30492259/get-a-random-number-focused-on-center
fn weighted_random(min:f32, max:f32, weight:u32) -> f32 {
    let mut num = 0.0;
    for i in 0..weight {
        num += random_range(min, max) * (max/weight as f32);
    }  
    return num;
}

