use nannou::prelude::*;

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
    // for i in 0..count {
    //     let angle = random::<f32>() * PI * 2.0;
    //     let r = 1.0 - random::<f32>() * random::<f32>() * random::<f32>();
    //     let x = (radius*scale) * angle.cos() * r + pos.x;
    //     let y = (radius*scale) * angle.sin() * r + pos.y;
    //     draw.ellipse().x_y(x, y).w_h(stipple_size, stipple_size).color(GRAY);
    // }

    for a in 0..angles {
        let degree = (360/angles*a) as f32;
        let radian = deg_to_rad(degree);

        let reduced_points = map_range(a, 0, angles, points, 0);
        let reduced_point_size = map_range(a, 0, angles, point_size, 0.0);
        let reduced_alpha = map_range(a, 0, angles, 0.1, 1.0);
        let reduced_radius = map_range(a, 0, angles, radius, radius*0.2);

        for p in 0..reduced_points {

            //recursive-random factor
            let rrand = 1.0 - random::<f32>() * random::<f32>() * random::<f32>() * random::<f32>();

            // let x = radian.cos() * radius * rrand + pos.x;
            // let y = radian.sin() * radius * rrand + pos.y;
            let x = radian.cos() * reduced_radius * rrand + pos.x;
            let y = radian.sin() * reduced_radius * rrand + pos.y;

            // let start = vec2(0.0, 0.0);
            // let end = vec2(x, y);
            let c = rgba(0.0, 0.0, 0.0, reduced_alpha);

            draw.ellipse().x_y(x, y).w_h(reduced_point_size, reduced_point_size).color(c);
        }
    }

    for a in 0..angles {
        let degree = (360/angles*a*-1) as f32;
        let radian = deg_to_rad(degree)-PI;

        let reduced_points = map_range(a, 0, angles, points, 0);
        let reduced_point_size = map_range(a, 0, angles, point_size, 0.0);
        let reduced_alpha = map_range(a, 0, angles, 0.1, 1.0);

        for p in 0..reduced_points {

            //recursive-random factor
            let rrand = 1.0 - random::<f32>() * random::<f32>() * random::<f32>() * random::<f32>();

            let x = radian.cos() * radius * rrand + pos.x;
            let y = radian.sin() * radius * rrand + pos.y;

            let c = rgba(0.0, 0.0, 0.0, reduced_alpha);

            draw.ellipse().x_y(x, y).w_h(reduced_point_size, reduced_point_size).color(c);
        }
    }

    
    //  draw.rect()
    //     .x_y(win.w()*0.25,-win.h()*0.25)
    //     .w_h(win.w()*0.5, win.h()*0.5)
    //     .stroke_weight(100.0)
    //     .stroke_color(WHITE)
    //     .color(rgba(0.0,0.0, 0.0, 0.0))
    //     ;

    for i in 0..count*2 {
        let angle = random::<f32>() * PI * 2.0;
        let r = 1.0 - random::<f32>() * random::<f32>() * random::<f32>();
        let x = (radius*scale) * angle.cos() * r + pos.x;
        let y = (radius*scale) * angle.sin() * r + pos.y;
        draw.ellipse().x_y(x, y).w_h(stipple_size, stipple_size).color(GRAY);
    }


    for i in 0..count*10 {
        let rrand = win.h() * random::<f32>() * random::<f32>() * random::<f32>();
        let x = map_range(i as f32, 0.0, count as f32, win.h() / 2.0, -win.h() / 2.0);
        let y = 0.0- rrand;
        draw.rotate(PI)
            .ellipse()
            .x_y(x, y+win.h()*0.5)
            .w_h(1.0, 1.0)
            .color(BLACK);
        // for ii in 0..3 {
        //     let y_off = 50.0 * ii as f32;
        // }
    }

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();
}
