use nannou::prelude::*;

//--------------------------------------------------------
// CAPTURE
static FILENAME: &str = "np-002";
static CAPTURE_DIR: &str = "nature-pix/exports";
static CAPTURE: bool = true;
//--------------------------------------------------------


static SIZE: u32 = 800;

fn main() {
    nannou::app(model).update(update).exit(exit).run();
}

struct Model {
    window_id: WindowId,
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
    texture_reshaper: wgpu::TextureReshaper 
}

fn model(app: &App) -> Model {
    // Lets write to a 4K UHD texture.
    let texture_size = [SIZE, SIZE];

    // Create the window.
    let [win_w, win_h] = [800, 800];
    let window_id = app
        .new_window()
        .size(win_w, win_h)
        .title(FILENAME)
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
        let dir     = CAPTURE_DIR; //"captures/".to_string();
        let sub_dir = FILENAME.to_string();
        let path = format!("{}/{}", dir, sub_dir);
        // app.main_window().capture_frame(path);
        std::fs::create_dir_all(path).unwrap();
    }

    let captured_complete = false;

    Model { 
        window_id,
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
        let win_rect = geom::Rect::from_w_h(w as f32, h as f32);
    
        // Use the frame number to animate, ensuring we get a constant update time.
        let elapsed_frames = app.main_window().elapsed_frames();
        let t = elapsed_frames as f32 / 60.0;
    
        // Draw like we normally would in the `view`.
        //--------------------------------------------------------
        draw.background().color(WHITE);
        // let draw = draw.rotate(-PI/4.0);

        // DO DRAWING HERE
        // let draw = draw.rotate(PI / 2.0);

        let count = 20000;
        let point_size = 2.0;
        let rect_width = win_rect.w() / 2.0;

        for i in 0..count {
            let rrand = rect_width * random::<f32>() * random::<f32>();
            let y = map_range(i as f32, 0.0, count as f32, win_rect.h() / 2.0, -win_rect.h() / 2.0);
            let x = rrand;
            draw.rotate(PI)
                .ellipse()
                .x_y(x, y)
                .w_h(point_size, point_size)
                .color(BLACK);
            // for ii in 0..3 {
            //     let y_off = 50.0 * ii as f32;
            // }
        }

        draw.rect()
            .x_y(0.0, 0.0)
            .w_h(rect_width, rect_width * 1.5)
            .color(WHITE)
            .stroke_weight(1.0)
            .stroke_color(rgba(0.0, 0.0, 0.0, 0.4));

        draw.rect()
            .x_y(0.0, 0.0)
            .w_h(rect_width * 0.5, rect_width)
            .color(WHITE)
            .stroke_weight(1.0)
            .stroke_color(rgba(0.0, 0.0, 0.0, 1.0));

        for i in 0..count / 2 {
            let rrand = (rect_width * 0.25) * random::<f32>() * random::<f32>();
            let x = -rect_width / 4.0 + rrand;
            let y = map_range(
                i as f32,
                0.0,
                (count / 2) as f32,
                -rect_width / 2.0,
                rect_width / 2.0,
            );
            draw.ellipse()
                .x_y(x, y)
                .w_h(point_size / 2.0, point_size / 2.0)
                .color(BLACK);
        }

        // END DRAWING
        
        //--------------------------------------------------------
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
    let dir     = CAPTURE_DIR;//"captures".to_string();
    let sub_dir = FILENAME.to_string();
    let path = format!("{}/{}", dir, sub_dir);

    print!("path: {}\n", path);
    print!("--------------\n");

    app.project_path()
        .expect("could not locate project_path")
        .join(path)
}