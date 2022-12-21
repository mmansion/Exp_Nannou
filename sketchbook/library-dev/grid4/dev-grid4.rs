/*
development file for grid4 lib
*/

// use nannou::lyon::path::AttributeStore;
use nannou::lyon::geom::euclid::num::Ceil;
use nannou::prelude::*;

use nannou_touchosc::TouchOscClient as TouchOscClient;

// use nannou::geom::*;
// use nannou::geom::Point2;
// use std::ops::Range;
// use nannou::Draw;
use std::time::Duration;

use library::colors::Palette;
// use library::line::Line;
use library::grid4::Grid4 as Grid;

// beginning of touch library for nannou
// use library::touchosc3::TouchOscClient;

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
    touchosc: TouchOscClient,
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

    let mut touchosc = TouchOscClient::new(6555);

    touchosc.add_fader("/grid/rows", 2.0, (WIDTH/10) as f32, 20.0);
    touchosc.add_fader("/grid/cols", 2.0, (HEIGHT/10) as f32, 20.0);

    touchosc.add_fader("/grid/cols-rows", 2.0, (WIDTH/10) as f32, 10.0);

    touchosc.add_button("/toggle/corner-points", false);
    touchosc.add_button("/toggle/cell-points", false);
    touchosc.add_button("/toggle/lines", false);
    touchosc.add_button("/toggle/arrows", true);

    // touchosc.add_fader("/rect/width");
    // touchosc.add_fader("/rect/height");
    // touchosc.add_fader("/rect/weight");
    // touchosc.add_fader("/rect/rotation");

    //--------------------------------------------------------
    let mut grid = Grid::new(10, 10, WIDTH, HEIGHT);

    // setup flowfield angles
    // TODO: change grid to use multi-dim vector
    for row in 0..grid.cell_points.len() {
        for col in 0..grid.cell_points[row].len() {
            let rows = grid.rows as f32;
            let this_row = row as f32;
            let angle = this_row / rows * PI;
            grid.cell_angles[row][col] = angle;       
        }
    }

    //--------------------------------------------------------

    Model {
        window_id,
        this_capture_frame,
        last_capture_frame,
        last_calc,
        colors,
        redraw,
        last_redraw,
        touchosc,
        grid,
    }
}

fn update(app: &App, m: &mut Model, _update: Update) {

    let win = app.window_rect();
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
    m.touchosc.update(); //update vals


    m.grid.show_cell_points = m.touchosc.button("/toggle/cell-points");
    m.grid.show_corner_points = m.touchosc.button("/toggle/corner-points");
    m.grid.show_lines  = m.touchosc.button("/toggle/lines");
    m.grid.show_arrows = m.touchosc.button("/toggle/arrows");
    // println!("{}", fader_rows);

    // println!("{}, {}", n_rows, n_cols);
    // let n_rows = m.touchosc.fader("/grid/rows") as usize;
    // let n_cols = m.touchosc.fader("/grid/cols") as usize;

    let resolution = (win.w() * 0.08); 
    let n_cols = ( win.w()  / resolution) as usize; 
    let n_rows = ( win.h() / resolution) as usize;

    m.grid.set_rows(n_rows);
    m.grid.set_cols(n_cols);

    for row in 0..m.grid.cell_points.len() {
        for col in 0..m.grid.cell_points[row].len() {
            let rows = m.grid.rows as f32;
            let this_row = row as f32;
            let angle = this_row / rows * PI;
            m.grid.cell_angles[row][col] = angle;       
        }
    }

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
        let num_steps = 10;
        let start = pt2(-300.0, 300.0);
        let step = pt2(10.0, -10.0);
        let mut x = start.x;
        let mut y = start.y;

        let left_x = win.left();
        let top_y = win.top();

        let resolution = (win.w() * 0.01); 

        for n in 0..num_steps {

            draw.ellipse()
            .x_y(x, y)
            .radius(5.0)
            .color(WHITE);

            let x_offset = x - left_x;
            let y_offset = y - top_y;

            let x_index = (x_offset / resolution) as usize;
            let y_index = (y_offset / resolution) as usize;

            print!("{}, {}", x_index, y_index);

            
//             let x_off = win.w() / m.grid.cols as f32;
//             let y_off = -win.h() / m.grid.rows as f32;

//             let step = pt2(x + x_off, y + y_off);
// ;           let angle = m.grid.get_nearest_cell_angle(s);

//             // println!("{}", x_off);

//             x = p.x;
//             y = p.y;
            // print!("{}, {}", x, y);
        }

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
