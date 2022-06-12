use nannou::prelude::*;

static WIDTH: i32 = 800;
static HEIGHT: i32 = 800;
static DIVS: i32 = 10;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    points: Vec<Vector2>, // points bin no.1
    this_capture_frame: i32,
    last_capture_frame: i32,
}

fn model(app: &App) -> Model {
    let rect = Rect::from_w_h(WIDTH, HEIGHT);

    app.new_window()
        .size(rect.w() as u32, rect.h() as u32)
        .view(view)
        .build()
        .unwrap();

    let mut points = Vec::new();
    let mut this_capture_frame = 0;
    let mut last_capture_frame = 0;

    for row in 0..(DIVS + 1) {
        let y = (HEIGHT / DIVS * row) as f32;

        for col in 0..(DIVS + 1) {
            let x = (WIDTH / DIVS * col) as f32;

            points.push(pt2(x, y));
        }
    }

    Model {
        points,
        this_capture_frame,
        last_capture_frame,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    if model.this_capture_frame != model.last_capture_frame {
        model.last_capture_frame = model.this_capture_frame;
    }

    if app.keys.down.contains(&Key::S) {
        model.this_capture_frame += 1;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // get canvas to draw on
    let draw = app.draw();

    // set background to blue
    draw.background().color(rgb(0.2, 0.0, 0.8));

    let draw = draw.x_y((-WIDTH / 2) as f32, (-HEIGHT / 2) as f32);

    let t = app.time;

    for i in 0..model.points.len() {
        // println!( "{},{}", model.points[i].x, model.points[i].y );
        let col = hsv((t * 0.001 * i as f32).sin(), 1.0, 1.0);
        draw.ellipse()
            .x_y(model.points[i].x, model.points[i].y)
            .radius(((t) + i as f32).sin() * 20.0)
            .color(col);
    }

    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();

    if model.this_capture_frame != model.last_capture_frame {
        // let mut owned_string: String = "hello ".to_owned();
        // let borrowed_string: String = "output/" + app.exe_name().unwrap() + ".png";

        let directory = "captures/21/".to_string();
        let app_name = app.exe_name().unwrap().to_string();
        // let frame_num  = model.this_capture_frame.to_string();
        let extension = ".png".to_string();

        let frame_num = format!("{:05}", model.this_capture_frame);

        let path = format!("{}{}{}", directory, frame_num, extension);

        app.main_window().capture_frame(path);
    }
}
