use nannou::geom::Point2;
use nannou::geom::*;
use nannou::prelude::*;
use nannou::Draw;
use std::ops::Range;
use std::time::Duration;

use library::bezier::Bezier;
use library::grid;

//--------------------------------------------------------
static CAPTURE: bool = true; // capture to image sequence
static WIDTH: f32 = 800.0;
static HEIGHT: f32 = 800.0;

//--------------------------------------------------------
fn main() {
    nannou::app(model).update(update).run();
}

//--------------------------------------------------------
struct Model {
    this_capture_frame: i32,
    last_capture_frame: i32,
    last_calc: Duration,

    curves: Vec<Bezier>,
    incs: Vec<f32>,
    texture: wgpu::Texture,
}

//--------------------------------------------------------
fn model(app: &App) -> Model {
    let rect = Rect::from_w_h(WIDTH, HEIGHT);

    // app.set_loop_mode(LoopMode::loop_once());
    // app.set_loop_mode(LoopMode::rate_fps(0.1));

    app.new_window().size(800, 800).view(view).build().unwrap();

    //--------------------------------------------------------

    // load image
    let assets = app.assets_path().unwrap();
    let img_path = assets.join("images").join("apollo08_earthrise.jpg");
    let texture = wgpu::Texture::from_path(app, img_path).unwrap();

    //--------------------------------------------------------

    let mut last_calc = Duration::from_millis(0);

    //--------------------------------------------------------
    let mut this_capture_frame = 0;
    let mut last_capture_frame = 0;

    //--------------------------------------------------------

    let mut curves = Vec::new();
    let mut incs = Vec::new();
    // let mut vec =  Vec<T>::new();
    let mut inner_circle: Vec<Point2> = Vec::new();

    let angles = 9;
    let radius = 600.0;

    let inner_radius = 50.0;

    for i in 0..angles {
        let inc = ((360 / angles * i) as f32).to_radians();

        let x = inc.cos() * inner_radius;
        let y = inc.sin() * inner_radius;

        inner_circle.push(pt2(x, y));
    }

    for i in 0..angles {
        let inc = ((360 / angles * i) as f32).to_radians();

        let x = inc.cos() * radius;
        let y = inc.sin() * radius;

        let offX = inner_circle[i].x;
        let offY = inner_circle[i].y;

        let p1 = pt2(offX, offY);
        let cp1 = pt2(x * 0.25 + offX, y * 0.25 + offY + app.time.cos());
        let cp2 = pt2(x * 0.75 + offX, y * 0.75 + offY + app.time.sin());
        // let p2  = pt2(x + offX, y+ offY);
        let p2 = pt2(0.0, 0.0);

        curves.push(Bezier::new(p1, cp1, cp2, p2));

        curves[i].show_handles = false;
        curves[i].stroke_weight(6.0);
    }

    // setup incs
    for i in 0..angles {
        incs.push(i as f32 * random_f32());
    }

    //--------------------------------------------------------

    Model {
        this_capture_frame,
        last_capture_frame,
        last_calc,
        texture,
        curves,
        incs,
    }
}

fn update(app: &App, m: &mut Model, _update: Update) {
    // ref:
    //https://doc.rust-lang.org/nightly/core/time/struct.Duration.html
    //let millis = Duration::from_millis(100).as_millis();
    let since_last_calc = _update.since_start.as_millis() - m.last_calc.as_millis();
    if since_last_calc > 10 {
        //time interval
        m.last_calc = _update.since_start;

        //increment incs
        for inc in m.incs.iter_mut() {
            *inc += 0.5;
        }
    }

    if m.this_capture_frame != m.last_capture_frame {
        m.last_capture_frame = m.this_capture_frame;
    }

    if CAPTURE {
        m.this_capture_frame += 1;
    }

    //--------------------------------------------------------
    let rot_radius = 600.0;
    for i in 0..m.curves.len() {
        let t = app.time;
        let x = (t.cos() * 0.2) * rot_radius;
        let y = (t.sin() * 0.2) * rot_radius;
        //let y = 0.0;

        m.curves[i].end_point = pt2(x, y);

        // let xOff = (m.incs[i]).cos() * 0.5;
        // let yOff = (m.incs[i]).sin() * 0.5;

        // m.curves[i].control_point_2 = pt2(
        //     m.curves[i].control_point_2.x + xOff,
        //     m.curves[i].control_point_2.y +yOff
        // );
    }
    //--------------------------------------------------------
}

fn view(app: &App, m: &Model, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();
    let win = app.window_rect();
    let time = app.time;

    //--------------------------------------------------------
    // background

    // let bg = rgba(1.0, 1.0, 1.0, 0.03);
    let bg = hsva(app.time * 0.1, 1.0, 1.0, 1.0);
    //let bg = rgba(0.54, 0.6, 0.99, 0.05);

    if app.elapsed_frames() == 1 {
        draw.background().color(bg);
    } else {
        draw.rect()
            .x_y(0.0, 0.0)
            .w_h(win.w() * 2.0, win.w() * 2.0)
            .color(bg);
    }

    //--------------------------------------------------------

    //draw.scale(0.75).rotate(0.0).translate(pt3(0.0, 20.0, 0.0)).texture(&m.texture);

    //--------------------------------------------------------

    let w = win.w();
    let h = win.h();
    let rows = 100;

    draw.rect().w(w).h(h / 2.0).x_y(0.0, -h / 4.0).color(BLACK);
    //--------------------------------------------------------

    //--------------------------------------------------------
    let red = rgba(1.0, 0.26, 0.25, 0.5);
    let col = hsva(app.time * 0.1, 1.0, 1.0, 1.0);
    let red_solid = rgba(1.0, 0.26, 0.25, 1.0);
    // let white = rgba(1.0, 1.0, )

    let toggle = (app.time % 1.0).round();

    let draw2 = draw.rotate(app.time.cos() * 1.8);
    let draw3 = draw.rotate(app.time * 0.1);

    draw.ellipse().x_y(0.0, 0.0).radius(30.0).color(red_solid);

    for i in 0..rows {
        let r = rows as f32;
        let n = i as f32;
        let mut y = h / r * n;

        y += random_f32();

        draw.line()
            .stroke_weight(1.0)
            .color(red)
            .points(pt2(-w / 2.0, y), pt2(w / 2.0, y));
    }

    // println!("{}", toggle);
    for i in 0..m.curves.len() {
        m.curves[i].draw(&draw3, red);

        m.curves[i].draw(&draw3, rgba(1.0, 1.0, 1.0, 0.1));

        // draw.ellipse()
        // .x_y(m.curves[i].control_point_2.x, m.curves[i].control_point_2.y)
        // .radius(10.0)
        // .color(BLACK);

        draw2
            .ellipse()
            .x_y(m.curves[i].control_point_1.x, m.curves[i].control_point_1.y)
            .radius(10.0)
            .color(WHITE);

        // draw3.ellipse()
        // .x_y(m.curves[i].end_point.x, m.curves[i].end_point.y)
        // .radius(20.0)
        // .color(BLACK);
    }

    //--------------------------------------------------------

    let points_arr_1 = [
        pt2(0.0, 0.0),
        pt2(win.w() / 2.0, 0.0),
        pt2(0.0, -win.h() / 2.0),
        pt2(0.0, 0.0),
    ];

    let points_arr_2 = [
        pt2(0.0, 0.0),
        pt2(-win.w() / 2.0, 0.0),
        pt2(0.0, win.h() / 2.0),
        pt2(0.0, 0.0),
    ];

    let points_arr_3 = [
        pt2(0.0, 0.0),
        pt2(win.w() / 2.0, 0.0),
        pt2(0.0, win.h() / 2.0),
        pt2(0.0, 0.0),
    ];

    let points_arr_4 = [
        pt2(0.0, 0.0),
        pt2(-win.w() / 2.0, 0.0),
        pt2(0.0, -win.h() / 2.0),
        pt2(0.0, 0.0),
    ];

    let mut points_1 = (0..points_arr_1.len()).map(|i| points_arr_1[i]);
    let mut points_2 = (0..points_arr_2.len()).map(|i| points_arr_2[i]);
    let mut points_3 = (0..points_arr_3.len()).map(|i| points_arr_3[i]);
    let mut points_4 = (0..points_arr_4.len()).map(|i| points_arr_4[i]);

    let dist = 250.0;
    let draw_top_left = draw.translate(pt3(-dist, dist, 0.0));
    let draw_btm_right = draw.translate(pt3(dist, -dist, 0.0));
    let draw_btm_left = draw.translate(pt3(-dist, -dist, 0.0));
    let draw_top_right = draw.translate(pt3(dist, dist, 0.0));
    let scale = app.time.sin() * 2.0;

    let col = hsva(app.time.sin(), app.time.sin(), 1.0, 1.0);

    // draw_top_left.scale(scale)
    //     .polyline()
    //     .weight(2.0)
    //     .color(col)
    //     .points(points_1)
    //     ;

    // draw_btm_right.scale(scale)
    //     .polyline()
    //     .weight(2.0)
    //     .color(col)
    //     .points(points_2)
    //     ;

    // draw_btm_left.scale(scale)
    //     .polyline()
    //     .weight(2.0)
    //     .color(col)
    //     .points(points_3)
    //     ;

    // draw_top_right.scale(scale)
    //     .polyline()
    //     .weight(2.0)
    //     .color(col)
    //     .points(points_4)
    //     ;

    //--------------------------------------------------------
    // draw frame

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();

    //--------------------------------------------------------
    // capture frame

    if m.this_capture_frame != m.last_capture_frame {
        let directory = "captures/".to_string();
        let app_name = app.exe_name().unwrap().to_string();
        let extension = ".png".to_string();
        let frame_num = format!("{:05}", m.this_capture_frame);

        let path = format!("{}{}{}", directory, frame_num, extension);
        app.main_window().capture_frame(path);
    }
}
