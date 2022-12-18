use nannou::{prelude::*, color::white_point::F2};

//--------------------------------------------------------
static FILENAME: &str = "n-0104";
static CAPTURE: bool = true;
static FRAME: bool = false; //hide window chrome when set to false

static SIZE: u32 = 800;

fn main() {
    nannou::app(model).run();
}

struct Model {
    window_id: WindowId,
}

fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(SIZE, SIZE)
        .decorations(FRAME) //creates a borderless window
        .view(view)
        .build()
        .unwrap();
    

    app.set_loop_mode(LoopMode::loop_once());

    Model {
        window_id,
    }
}

fn view(app: &App, m: &Model, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();
    let win = app.window_rect();

    draw.background().color(WHITE);
    // let draw = draw.rotate(-PI/4.0);
   
    let pos = pt2(0.0, 0.0);
    let radius = 300.0;
    let angles = 360;
    let points = 300;
    let point_size = 4.0;

    let count = 20000;
    let scale = 1.0;
    let stipple_size = 0.9;

    let rect_size = 600.0;
    let steps = 10;
    for x in 0..steps+1 {
        let x_step = radius / steps as f32 * x as f32;
        draw
        .translate( pt3( -radius, 0.0, 0.0) )
        .line()
        .points( pt2(x_step, 0.0),pt2(x_step, win.bottom()) )
        .stroke_weight(1.0)
        .color(rgba(0.0,0.0,0.0,0.5))
        ;

         draw
        .translate( pt3( 0.0, 0.0, 0.0) )
        .line()
        .points( pt2(x_step, 0.0),pt2(x_step, win.top()) )
        .stroke_weight(1.0)
        .color(rgba(0.0,0.0,0.0,0.5))
        ;
    }

    draw.ellipse()
    .x_y(0.0, 0.0)
    .w_h(radius*2.0, radius*2.0)
    .color(WHITE)
    ;
    
     for i in 0..count*2 {
        let angle = random::<f32>() * PI * 2.0;
        let r = 1.0 - random::<f32>() * random::<f32>() * random::<f32>();
        let x = (radius*scale) * angle.cos() * r + pos.x;
        let y = (radius*scale) * angle.sin() * r + pos.y;
        draw.ellipse().x_y(x, y).w_h(stipple_size, stipple_size).color(GRAY);
    }

    for i in 0..count*10 {
        let rrand = (win.h()*0.5) * random::<f32>() * random::<f32>() * random::<f32>();
        let x = map_range(i as f32, 0.0, count as f32, win.h() / 2.0, -win.h() / 2.0);
        let y = 0.0- rrand;
        draw.rotate(PI)
            .ellipse()
            .x_y(x, y+win.h()*0.5)
            .w_h(1.0, 1.0)
            .color(BLACK);
    }
    for i in 0..count {
        let angle = random::<f32>() * PI * 2.0;
        let r = 1.0 - random::<f32>() * random::<f32>() * random::<f32>();
        let x = (radius*scale) * angle.cos() * r + pos.x;
        let y = (radius*scale) * angle.sin() * r + pos.y;
        draw.ellipse().x_y(x, y).w_h(stipple_size, stipple_size).color(GRAY);
    }
    
    let max_radius = 300;
    let angles = 360;
    let deg_step = 1;
    let scale = 1.0;
    let steps = 360;
    let step_size = 10;
    let mut spiral_points = Vec::new();
    for i in 0..steps {
       // let inc = (360/step_size * i) as f32;
        let inc =  ( (steps/step_size * i) as f32).to_radians();
        let r = scale * i as f32;
        let x = inc.cos() * r;
        let y = inc.sin() * r;
        let max = scale*steps as f32;
        let s = map_range(r, 0.0, max, 0.0, 1.0);
        let bs = map_range(r, 0.0, max, 1.0, 0.0);
        let col = rgba(0.0, 0.0, 0.0, bs);
        let bcol = rgba(0.0, 0.0, 0.0, s);
        let point_size = s*20.0;
        spiral_points.push(pt2(x, y));

        if i > 0 && i < 300 {
            draw.line()
            .color(bcol)
            .stroke_weight(bs*1.5)
            .points(spiral_points[i-1], spiral_points[i])
            ;
        }
    }

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}
