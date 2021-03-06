use nannou::image;
use nannou::image::RgbaImage;
use nannou::prelude::*;
use nannou::noise::*;
use nannou::Draw;
use std::path::{Path, PathBuf};
use std::collections::VecDeque;
use std::time::Duration;

pub mod lib;

// module tree
use crate::lib::points::Point as Point;
use crate::lib::vehicles::Vehicle as Vehicle;

//-------------------------------------------------------------------
static CAPTURE  : bool = false; // capture to image sequence
static WIDTH    : i32  = 800;
static HEIGHT   : i32  = 800; 
static ANGLES   : i32  = 32;
//-------------------------------------------------------------------

// The vertex type that we will use to represent a point on our triangle.
#[repr(C)]
#[derive(Clone, Copy)]
struct Vertex {
    position: [f32; 2],
}

// The vertices that make up the rectangle to which the image will be drawn.
const VERTICES: [Vertex; 4] = [
    Vertex {
        position: [-1.0, 1.0],
    },
    Vertex {
        position: [-1.0, -1.0],
    },
    Vertex {
        position: [1.0, 1.0],
    },
    Vertex {
        position: [1.0, -1.0],
    },
];

//-------------------------------------------------------------------

fn main() {
    nannou::app(model).update(update).run();
}

//-------------------------------------------------------------------
struct Model {
    points : Vec<Vector2>,
    incs   : Vec<f32>,
    noise  : Perlin,
    xOff   : f64, 
    yOff   : f64,
    this_capture_frame : i32,
    last_capture_frame : i32,
    new_frame : bool,
    last_calc : Duration,
    inc : f32,

    current_layer: f32,
    texture_array: wgpu::Texture,
    texture_view: wgpu::TextureView,
    sampler: wgpu::Sampler,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
}

// ----------------------------------------------------------------------
fn model(app: &App) -> Model {

    // Load the images.
    let sequence_path = app
    .assets_path()
    .unwrap()
    .join("images")
    .join("birds");

    println!("Loading images...");
    let (images, (img_w, img_h)) = load_images(&sequence_path);
    println!("Done!");

    let w_id = app
        .new_window()
        .size(WIDTH as u32, HEIGHT as u32)
        .view(view)
        .build()
        .unwrap();

    let window = app.window(w_id).unwrap();
    let device = window.swap_chain_device();
    let format = Frame::TEXTURE_FORMAT;
    let msaa_samples = window.msaa_samples();

    let mut noise = Perlin::new();
    noise = noise.set_seed(1);
    let mut xOff = 0.0;
    let mut yOff = 0.0;
    let mut points = Vec::new();
    let mut incs = Vec::new();
    let mut new_frame = false;
    let mut last_calc = Duration::from_millis(0);
    let mut inc = 0.0;
    //----------------------------------
    let mut this_capture_frame = 0;
    let mut last_capture_frame = 0;
    //----------------------------------

    // setup incs
    for i in 0..ANGLES+1 {
        incs.push(i as f32 * random_f32());
    }


    //----------------------------------
    

    let vs_mod = wgpu::shader_from_spirv_bytes(device, include_bytes!("shaders/vert.spv"));
    let fs_mod = wgpu::shader_from_spirv_bytes(device, include_bytes!("shaders/frag.spv"));

    let texture_array = {
        // The wgpu device queue used to load the image data.
        let mut queue = window.swap_chain_queue();
        // Describe how we will use the texture so that the GPU may handle it efficiently.
        let usage = wgpu::TextureUsage::SAMPLED;
        let iter = images.iter().map(|&(_, ref img)| img);
        wgpu::Texture::load_array_from_image_buffers(device, queue, usage, iter)
            .expect("tied to load texture array with an empty image buffer sequence")
    };
    let layer = 0;
    let texture_view = texture_array.view().layer(layer).build();

    // Create the sampler for sampling from the source texture.
    let sampler = wgpu::SamplerBuilder::new().build(device);

    let bind_group_layout = create_bind_group_layout(device, texture_view.component_type());
    let bind_group = create_bind_group(device, &bind_group_layout, &texture_view, &sampler);
    let pipeline_layout = create_pipeline_layout(device, &bind_group_layout);
    let render_pipeline = create_render_pipeline(
        device,
        &pipeline_layout,
        &vs_mod,
        &fs_mod,
        format,
        msaa_samples,
    );


    // IMAGE SEQUENCE STUFF
    // Create the vertex buffer.
    let vertices_bytes = vertices_as_bytes(&VERTICES[..]);
    let usage = wgpu::BufferUsage::VERTEX;
    let vertex_buffer = device.create_buffer_with_data(vertices_bytes, usage);

    Model {
        this_capture_frame, 
        last_capture_frame, 
        noise, points, 
        incs,
        xOff,
        yOff,
        new_frame,
        last_calc,
        inc,
        current_layer: 0.0,
        texture_array,
        texture_view,
        sampler,
        bind_group_layout,
        bind_group,
        vertex_buffer,
        render_pipeline,
    }
} 

//-------------------------------------------------------------------
fn update(app: &App, m: &mut Model, _update: Update) {

    // ref:
    //https://doc.rust-lang.org/nightly/core/time/struct.Duration.html
    //let millis = Duration::from_millis(100).as_millis();

    let since_last_calc = _update.since_start.as_millis() - m.last_calc.as_millis();

    if since_last_calc > 100  {

        m.last_calc = _update.since_start;
        m.inc += 1.5;
        m.new_frame = true;

        for inc in m.incs.iter_mut() {
            *inc += 0.05;
        }
        if m.this_capture_frame != m.last_capture_frame {
            m.last_capture_frame = m. this_capture_frame;
        }
        if CAPTURE {
            m.this_capture_frame += 1;
        }
        
    } else {
        m.new_frame = false;
    }
}

//-------------------------------------------------------------------
fn view(app: &App, m: &Model, frame: Frame) {

    // get canvas to draw on
    let draw  = app.draw();
    let win   = app.window_rect();
    let t     = app.time;

    // draw frame ---------------------------------------------------------
    if m.new_frame  {

        let bg = rgba(1.0, 1.0, 1.0, 0.01);
        
        if m.inc < 10.0 {
            draw.background().color(BLACK);
        } else {
            //background
            draw.rect().x_y(0.0, 0.0).w_h(win.w()*2.0, win.w()*2.0).color(bg);
        }
        draw.background().color(BLACK);

        for z in 0..2 {

            // let randX = WIDTH as f32 * random_f32();
            // let randY = HEIGHT as f32 * random_f32();
            let randX = random_range(-WIDTH/2, WIDTH/2) as f32;
            let randY = random_range(-HEIGHT/2, HEIGHT/2) as f32;

            let randPos = vec3(randX, randY, 0.0);

            let draw = draw.translate(randPos);

            for n in (0..10).rev() { 
                let atten = random_f32() * 0.25;
                let scale = (n as f32) * atten;
                let mut xStore = 0.0;
                let mut yStore = 0.0;

                let rad_a = 60.0;
                let rad_b = 10.0;
                let num_cusps = 12.0;

                let pts = (0..ANGLES + 1).rev().map(|i| {

                    let inc =  ( (360 / ANGLES * i) as f32).to_radians();
                    let ix  = i as usize;
                    
                    let x = inc.cos() * rad_a; 
                    //let y = (inc.sin() * rad_b;
                    let y = ( (num_cusps-1.0) * inc.sin() * rad_b) + (( (num_cusps -1.0) * inc).sin() * rad_b );

                    let r = 2.0;

                    let mut xOff = 0.0;
                    let mut yOff = 0.0;

                    //let mut xOff = (m.incs[ix] + inc).cos() * r; 
                    //let mut yOff = (m.incs[ix] + inc).sin() * r;

                    if i == 0 {
                        
                        xStore = xOff;
                        yStore = yOff;
                    } 

                    if i == ANGLES {
                    
                        xOff = xStore;
                        yOff = yStore;
                    }
                    // let n = (m.noise.get([x as f64, y as f64]) * 10.0) as f32;
            
                    pt2(x + xOff, y + yOff)


                });  
                
                // let color = hsva( t.sin() * 0.01, 1.0, 1.0, 1.0);
                let color = rgba(0.0, 0.0, 0.0, 1.0);
                // let draw = draw.rotate( (t.sin() * n as f32) * 0.0001);
                let draw = draw.rotate( 3.0*PI/2.0 );


                if n % 2 == 0 {
                    draw
                    .scale(scale)
                    .polygon()
                    //.color(BLUE)
                    .no_fill()
                    .stroke(rgba(0.0, 0.5, 0.5, 1.0))
                    .stroke_weight(0.9 + (0.9 * scale))
                    .points(pts)
                    ;
                } else {
                    draw
                    .scale(scale)
                    .polygon()
                    // .color(BLUE)
                    // .no_fill()
                    .stroke(rgba(1.0, 0.5, 0.8, 0.5))
                    .stroke_weight(0.5 + (0.5 * scale))
                    .points(pts)
                    ;
                }

            }
        }
    
        // put everything on the frame
        draw.to_frame(app, &frame).unwrap();

    } 


    // end draw frame ---------------------------------------------------------

    
    if m.this_capture_frame != m.last_capture_frame {
            
        // let mut owned_string: String = "hello ".to_owned();
        // let borrowed_string: String = "output/" + app.exe_name().unwrap() + ".png";
    
        let directory  = "captures/".to_string();
        let app_name   = app.exe_name().unwrap().to_string();
        // let frame_num  = m.this_capture_frame.to_string();
        let extension  = ".png".to_string();

        let frame_num = format!("{:05}", m.this_capture_frame);

        let path = format!("{}{}{}", directory, frame_num, extension);

        app.main_window().capture_frame(path);
        
    }
}

// Load a directory of images and returns them sorted by filename alongside their dimensions.
// This function assumes all the images have the same dimensions.
fn load_images(dir: &Path) -> (Vec<(PathBuf, RgbaImage)>, (u32, u32)) {
    let mut images = vec![];
    let mut dims = (0, 0);
    for entry in std::fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let image = match image::open(&path) {
            Ok(img) => img.into_rgba(),
            Err(err) => {
                eprintln!("failed to open {} as an image: {}", path.display(), err);
                continue;
            }
        };
        let (w, h) = image.dimensions();
        dims = (w, h);
        images.push((path, image));
    }
    images.sort_by_key(|(path, _)| path.clone());
    (images, dims)
}

fn create_bind_group_layout(
    device: &wgpu::Device,
    texture_component_type: wgpu::TextureComponentType,
) -> wgpu::BindGroupLayout {
    wgpu::BindGroupLayoutBuilder::new()
        .sampled_texture(
            wgpu::ShaderStage::FRAGMENT,
            false,
            wgpu::TextureViewDimension::D2,
            texture_component_type,
        )
        .sampler(wgpu::ShaderStage::FRAGMENT)
        .build(device)
}

fn create_bind_group(
    device: &wgpu::Device,
    layout: &wgpu::BindGroupLayout,
    texture: &wgpu::TextureView,
    sampler: &wgpu::Sampler,
) -> wgpu::BindGroup {
    wgpu::BindGroupBuilder::new()
        .texture_view(texture)
        .sampler(sampler)
        .build(device, layout)
}

fn create_pipeline_layout(
    device: &wgpu::Device,
    bind_group_layout: &wgpu::BindGroupLayout,
) -> wgpu::PipelineLayout {
    let desc = wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[&bind_group_layout],
    };
    device.create_pipeline_layout(&desc)
}

fn create_render_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    vs_mod: &wgpu::ShaderModule,
    fs_mod: &wgpu::ShaderModule,
    dst_format: wgpu::TextureFormat,
    sample_count: u32,
) -> wgpu::RenderPipeline {
    wgpu::RenderPipelineBuilder::from_layout(layout, vs_mod)
        .fragment_shader(fs_mod)
        .color_format(dst_format)
        .add_vertex_buffer::<Vertex>(&wgpu::vertex_attr_array![0 => Float2])
        .sample_count(sample_count)
        .primitive_topology(wgpu::PrimitiveTopology::TriangleStrip)
        .build(device)
}

// See the `nannou::wgpu::bytes` documentation for why this is necessary.
fn vertices_as_bytes(data: &[Vertex]) -> &[u8] {
    unsafe { wgpu::bytes::from_slice(data) }
}