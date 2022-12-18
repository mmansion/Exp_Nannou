/*
development file for grid4 lib
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
use library::grid4::Grid4 as Grid;

// beginning of touch library for nannou
use library::touchosc3::TouchOscClient;

//--------------------------------------------------------
static CAPTURE: bool = false; // capture to image sequence (or use obs)
static FRAME: bool = true; //hide window chrome when set to false
static WIDTH: i32 = 800;
static HEIGHT: i32 = 800;
static BORDER: f32 = 10.0;
static WAIT: u128 = 100;

static NUM_FADERS: usize = 4; //num of sliders used

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
    osc_client: TouchOscClient,

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

    let mut osc_client = TouchOscClient::new(6555);

    osc_client.add_fader("/grid/rows");
    osc_client.add_fader("/grid/cols");

    osc_client.add_button("/toggle/points");
    osc_client.add_button("/toggle/lines");
    osc_client.add_button("/toggle/arrows");

    // osc_client.add_fader("/rect/width");
    // osc_client.add_fader("/rect/height");
    // osc_client.add_fader("/rect/weight");
    // osc_client.add_fader("/rect/rotation");

    //--------------------------------------------------------
    let mut grid = Grid::new(10, 10, WIDTH, HEIGHT);

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
        osc_client,
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
    m.osc_client.update(); //update vals

    //GRID
    let fader_rows = m.osc_client.fader("/grid/rows").value;
    let fader_cols = m.osc_client.fader("/grid/cols").value;

    let toggle_points = (m.osc_client.button("/toggle/points").value == 1.0) as bool;
    let toggle_lines  = (m.osc_client.button("/toggle/lines").value  == 1.0) as bool;
    let toggle_arrows = (m.osc_client.button("/toggle/arrows").value == 1.0) as bool;

    //println!("{}, {}, {}", toggle_points, toggle_lines, toggle_arrows);

    m.grid.show_points = toggle_points;
    m.grid.show_lines  = toggle_lines;
    m.grid.show_arrows = toggle_arrows;
    // println!("{}", fader_rows);

    let n_rows = map_range(fader_rows, 0.0, 1.0, 2, WIDTH/10);
    let n_cols = map_range(fader_cols, 0.0, 1.0, 2, WIDTH/10);

    // println!("{}, {}", n_rows, n_cols);
    m.grid.set_rows(n_rows);
    m.grid.set_cols(n_cols);

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
