/*
development file for grid4 lib
*/

// use nannou::lyon::path::AttributeStore;

use nannou::lyon;
// use nannou::lyon::geom::euclid::num::Ceil;
use nannou::prelude::*;
use nannou_touchosc::TouchOscClient as TouchOscClient;
use std::time::Duration;

use library::colors::Palette;
use library::grid5::Grid5 as Grid;

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
    mouse_pressed: bool,
}

//--------------------------------------------------------
fn model(app: &App) -> Model {
    let window_id = app
        .new_window()
        .size(WIDTH as u32, HEIGHT as u32)
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .mouse_moved(mouse_moved)
        .key_pressed(key_pressed)
        .decorations(FRAME) //creates a borderless window
        .view(view)
        .build()
        .unwrap();

    let win = app.window_rect();
    // app.set_loop_mode(LoopMode::loop_once());
    // app.set_loop_mode(LoopMode::rate_fps(0.1));

    let mut last_calc = Duration::from_millis(0);

    //--------------------------------------------------------
    let mut this_capture_frame = 0;
    let mut last_capture_frame = 0;

    //--------------------------------------------------------
    let mut redraw = false;
    let mut last_redraw = 0;

    let mut builder = nannou::geom::path::Builder::new().with_svg();

    //--------------------------------------------------------

    let colors = Palette::new();

    let mut touchosc = TouchOscClient::new(6555);

    touchosc.add_fader("/grid/resolution", win.w() * 0.1, win.w() * 0.01, win.w() * 0.05);

    // touchosc.add_fader("/line-length", 0.001, 200.0, 50.0);

    // let line_length = touchosc.fader("/line-length");

    touchosc.add_fader("/grid/rows", 2.0, (WIDTH/10) as f32, 20.0);
    touchosc.add_fader("/grid/cols", 2.0, (HEIGHT/10) as f32, 20.0);
    touchosc.add_fader("/grid/rotate", 0.0, PI*2.0, 0.0);
    touchosc.add_fader("/grid/scale", 0.1, 1.0, 1.0);

    touchosc.add_fader("/grid/cols-rows", 2.0, (WIDTH/10) as f32, 10.0);
    touchosc.add_fader("/angle/rotate", 0.0, 1.0, 0.5);
    touchosc.add_radial("/angle/rotate-selected", 0.0, PI*2.0, 0.0);

    touchosc.add_button("/toggle/corner-points", false);
    touchosc.add_button("/toggle/cell-points", true);
    touchosc.add_button("/toggle/lines", true);
    touchosc.add_button("/toggle/arrows", false);
    // touchosc.add_button("/toggle/curves", true);
    // touchosc.add_button("/toggle/edit-mode", false);

    // touchosc.add_radar("/color/background", (0.0, 1.0, 1.0), (0.0, 1.0, 0.1));

    //--------------------------------------------------------

    // grid_pos, grid_width, grid_height, num_rows, num_cols
    let mut grid = Grid::new(pt2(0.0, 0.0), WIDTH as f32, HEIGHT as f32, 10, 10);

    grid.set_rect_mode(library::grid5::RectMode::Center);
    // grid.set_rect_mode(library::grid5::RectMode::Corner);

    grid.set_line_color(rgba( 169.0/255.0, 156.0/255.0, 217.0/255.0, 255.0/255.0));

    grid.on_resize = |grid| {

        println!("resizing grid to {},{}", grid.rows, grid.cols);
        //let angle_rotate = touchosc.fader("/angle/rotate");
        let angle_rotate = 0.0;
        let closure = |v:Vec2, rows:usize, cols:usize| -> f32 {   
            (v.x / rows as f32) * PI + (angle_rotate * cols as f32)
        };
        grid.set_angles_by_index(closure);
    };


    //--------------------------------------------------------
    let mouse_pressed = false;

    Model {
        window_id,
        this_capture_frame,
        last_capture_frame,
        last_calc,
        colors,
        redraw,
        last_redraw,
        // line_length,
        touchosc,
        grid,
        // curve_starting_points,
        mouse_pressed,
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

    // let resolution = (win.w() * 0.08); 
    let resolution = m.touchosc.fader("/grid/resolution");
    let rotation = m.touchosc.fader("/grid/rotate");
    let scale = m.touchosc.fader("/grid/scale");

    let n_cols = ( win.w()  / resolution) as usize; 
    let n_rows = ( win.h() / resolution) as usize;

    m.grid.set_rotation(rotation);
    m.grid.set_scale(scale);
    m.grid.set_rows(n_rows);
    m.grid.set_cols(n_cols);
    m.grid.set_rows_cols(n_rows, n_cols);


    let angle_rotate = m.touchosc.fader("/angle/rotate");
        
    let closure = |v:Vec2, rows:usize, cols:usize| -> f32 {   
        (v.x / rows as f32) * PI + (angle_rotate * cols as f32)
    };
    m.grid.set_angles_by_index(closure);


}

//--------------------------------------------------------
// DRAW

fn view(app: &App, m: &Model, frame: Frame) {
    if (m.redraw) {
        // get canvas to draw on
        let draw = app.draw();
        let win = app.window_rect();
        let time = app.time;

        //--------------------------------------------------------
        // background
        // let bg = m.colors.get_random();
        let bg = m.colors.vapor_blue;

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

fn mouse_pressed(app: &App, m: &mut Model, button: MouseButton) {
    m.mouse_pressed = true; 


    println!("{}", m.grid.get_cell(0,0));
   // m.grid.toggle_editable_cell( vec2(app.mouse.x, app.mouse.y));
    
}

fn mouse_released(app: &App, m: &mut Model, button: MouseButton) {
    m.mouse_pressed = false;

    
}

fn key_pressed(app: &App, m: &mut Model, key: Key) {
     if key == Key::Space {
     }
}

fn mouse_moved(app: &App, m: &mut Model, pos: Point2) {
    if m.mouse_pressed {
        // if m.grid.enable_edit_mode {
        //     m.grid.toggle_editable_cell( vec2(app.mouse.x, app.mouse.y) );
        // } else {
        //     m.curve_starting_points.push(pos);
        // }
    }
}

// fn on_grid_resized() {
//     println!("grid resized");
// }
// fn on_grid_resized(grid: &mut Grid4) {
//     println!("grid resized");
    
//     // m.curve_starting_points.clear();`
// }