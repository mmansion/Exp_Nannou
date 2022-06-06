/*
FEATURES
> osc receive, vec of faders, matched and loaded osc float values,
*/
// use nannou::lyon::path::AttributeStore;

use nannou::lyon::geom::euclid::num::Ceil;
use nannou::prelude::*;

// use nannou::geom::*;
// use nannou::geom::Point2;
// use std::ops::Range;
// use nannou::Draw;
use std::time::Duration;

use library::colors::Palette;
// use library::line::Line;
use library::grid2::Grid2 as Grid;

// beginning of touch library for nannou
use library::touchosc2::TouchOscClient;

//--------------------------------------------------------
static CAPTURE: bool = false; // capture to image sequence (or use obs)
static FRAME: bool = true; //hide window chrome when set to false
static WIDTH: f32 = 800.0;
static HEIGHT: f32 = 800.0;
static BORDER: f32 = 10.0;
static WAIT: u128 = 100;

static NUM_SLIDERS: usize = 4; //num of sliders used

// Make sure this matches the `TARGET_PORT` in the `osc_sender.rs` example.
const PORT: u16 = 6555;

//--------------------------------------------------------
fn main() {
    nannou::app(model).update(update).run();
}

//--------------------------------------------------------
struct Model {
    window_id: WindowId,
    this_capture_frame: i32,
    last_capture_frame: i32,
    last_calc: Duration,
    colors: Palette,
    redraw: bool,
    last_redraw: u128,
    touchosc_client: TouchOscClient,

    points: Vec<Point2>,

    grid: Grid,
}

//--------------------------------------------------------
fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(WIDTH as u32, HEIGHT as u32)
        .mouse_pressed(mouse_pressed)
        .decorations(FRAME) //creates a borderless window
        .view(view)
        .build()
        .unwrap();

    // app.set_loop_mode(LoopMode::loop_once());
    // app.set_loop_mode(LoopMode::rate_fps(0.1));

    let mut last_calc = Duration::from_millis(0);

    //--------------------------------------------------------
    let mut this_capture_frame = 0;
    let mut last_capture_frame = 0;

    //--------------------------------------------------------
    let mut redraw = false;
    let mut last_redraw = 0;

    //--------------------------------------------------------

    let colors = Palette::new();

    let mut touchosc_client = TouchOscClient::new("/nannou".to_string(), 6555);

    for n in 0..NUM_SLIDERS {
        let path = format!("/fader{}", n + 1);
        touchosc_client.add_fader(path);
        //faders.push( Fader::new(format!("/fader{}", n+1), 0.0));
    }

    //--------------------------------------------------------
    let rect = Rect::from_w_h(WIDTH, HEIGHT);
    let mut grid = Grid::new(10, 10, 10, 10, &rect);

    //--------------------------------------------------------
    let mut points = Vec::new();

    points.push(pt2(-0.5, 0.0));
    points.push(pt2(0.5, 0.0));

    //--------------------------------------------------------

    Model {
        window_id,
        this_capture_frame,
        last_capture_frame,
        last_calc,
        colors,
        redraw,
        last_redraw,
        touchosc_client,
        points,
        grid,
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
    }

    if m.this_capture_frame != m.last_capture_frame {
        m.last_capture_frame = m.this_capture_frame;
    }

    if CAPTURE {
        m.this_capture_frame += 1;
    }

    //--------------------------------------------------------
    // redraw framerate workaround
    // change WAIT to increase interval

    if _update.since_start.as_millis() - m.last_redraw > WAIT {
        m.last_redraw = _update.since_start.as_millis();
        m.redraw = true;
    } else {
        m.redraw = false;
    }
    //--------------------------------------------------------

    //OSC

    m.touchosc_client.update();
}

fn view(app: &App, m: &Model, frame: Frame) {
    if (m.redraw) {
        // get canvas to draw on
        let draw = app.draw();
        let win = app.window_rect();
        let time = app.time;

        //--------------------------------------------------------
        // background
        // let bg = m.colors.get_random();
        let bg = m.colors.mango;

        if app.elapsed_frames() < 10 {
            //must clear render context once for fullscreen
            draw.background().color(BLACK);
        } else {
            draw.rect()
                .x_y(0.0, 0.0)
                .w_h(win.w() * 2.0, win.w() * 2.0)
                .color(bg);
        }

        //--------------------------------------------------------
        m.grid.draw(&draw);

        for i in 0..m.grid.points.len() {
            let rotation = m.grid.angles[i];
            let position = m.grid.points[i];

            //let d = draw.rotate( rotation.angle()  );
            let _draw = draw.translate(pt3(position.x, position.y, 0.0));

            _draw.line().points(pt2(0.0, 0.0), rotation);

            let circles = (*m.touchosc_client.touchosc_faders[0].arg() * 100.0).round() as i32;
            let circle_res = (*m.touchosc_client.touchosc_faders[1].arg() * 32.0).round() as i32;
            let scale_factor = *m.touchosc_client.touchosc_faders[2].arg();
            let angle = TAU / circle_res as f32;
            let rad = 100.0;

            for c in 0..circles {
                let draw2 = _draw.rotate(*m.touchosc_client.touchosc_faders[3].arg() * PI * 2.0);
                let points = (0..=circle_res + 3).map(|i| {
                    let x = (angle * i as f32).cos() * rad * 2.0;
                    let y = (angle * i as f32).sin() * rad * 2.0;
                    pt2(x, y)
                });
                draw2
                    .scale(c as f32 * scale_factor)
                    .polyline()
                    .color(m.colors.black)
                    .weight(2.0)
                    .points(points); // Submit our points.
            }

            // _draw.rect()
            // .w_h(
            //     *m.touchosc_client.touchosc_faders[0].arg()*100.0,
            //     *m.touchosc_client.touchosc_faders[0].arg()*100.0
            // )
            // .rotate(*m.touchosc_client.touchosc_faders[1].arg()*PI)
            // .xy( vec2(map_range( *m.touchosc_client.touchosc_faders[2].arg(), 0.0, 1.0, -100.0, 100.0),0.0) )
            // .stroke_weight(10.0)
            // .color(BLACK)
            // ;
        }

        //--------------------------------------------------------
        // Create a polyline builder. Hot-tip: polyline is short-hand for a path that is
        // drawn via "stroke" tessellation rather than "fill" tessellation.

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
}

fn mouse_pressed(_app: &App, m: &mut Model, _button: MouseButton) {}
