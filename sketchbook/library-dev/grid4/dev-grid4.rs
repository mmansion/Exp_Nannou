/*
development file for grid4 lib
*/

// use nannou::lyon::path::AttributeStore;

use library::grid4;
use nannou::lyon;
// use nannou::lyon::geom::euclid::num::Ceil;
use nannou::prelude::*;

use nannou_touchosc::TouchOscClient as TouchOscClient;
use winit::event_loop;

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

use winit::{
        event::{Event, WindowEvent},
        event_loop::EventLoopBuilder,
        window::WindowBuilder,
    };

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

pub fn flowfield_1(v:Vec2, rows:usize, cols:usize) -> f32 {
    v.x.sin() * v.y.cos()
}
pub fn flowfield_2(v:Vec2, rows:usize, cols:usize) -> f32 {
    (v.x.cos() + v.y.sin()) * PI
}

pub fn flowfield_3(v:Vec2, rows:usize, cols:usize) -> f32 {    
    (v.x / rows as f32) * PI
}


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
    line_length: f32,
    grid: Grid,
    curve_starting_points: Vec<Point2>,
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

    let mut curve_starting_points = Vec::new();

    //--------------------------------------------------------

    let colors = Palette::new();

    let mut touchosc = TouchOscClient::new(6555);

    touchosc.add_fader("/grid/resolution", win.w() * 0.1, win.w() * 0.01, win.w() * 0.05);

    touchosc.add_fader("/line-length", 0.001, 200.0, 50.0);

    let line_length = touchosc.fader("/line-length");

    touchosc.add_fader("/grid/rows", 2.0, (WIDTH/10) as f32, 20.0);
    touchosc.add_fader("/grid/cols", 2.0, (HEIGHT/10) as f32, 20.0);

    touchosc.add_fader("/grid/cols-rows", 2.0, (WIDTH/10) as f32, 10.0);
    touchosc.add_fader("/angle/rotate", 0.0, 1.0, 0.5);
    touchosc.add_radial("/angle/rotate-selected", 0.0, PI*2.0, 0.0);

    touchosc.add_button("/toggle/corner-points", false);
    touchosc.add_button("/toggle/cell-points", true);
    touchosc.add_button("/toggle/lines", true);
    touchosc.add_button("/toggle/arrows", false);
    touchosc.add_button("/toggle/curves", true);
    touchosc.add_button("/toggle/edit-mode", false);

    touchosc.add_radar("/color/background", (0.0, 1.0, 1.0), (0.0, 1.0, 0.1));


    //--------------------------------------------------------

    // grid_pos, grid_width, grid_height, num_rows, num_cols
    let mut grid = Grid::new(pt2(-400.0, 400.0), WIDTH, HEIGHT, 10, 10);

    // grid.rect_mode = RectMode::Center;



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
        line_length,
        touchosc,
        grid,
        curve_starting_points,
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
    
    // m.grid.edit_mode = m.touchosc.button("/toggle/edit-mode");
    // println!("{}", fader_rows);

    // println!("{}, {}", n_rows, n_cols);
    // let n_rows = m.touchosc.fader("/grid/rows") as usize;
    // let n_cols = m.touchosc.fader("/grid/cols") as usize;

    // let resolution = (win.w() * 0.08); 
    let resolution = m.touchosc.fader("/grid/resolution");
    let n_cols = ( win.w()  / resolution) as usize; 
    let n_rows = ( win.h() / resolution) as usize;

    m.grid.set_rows(n_rows);
    m.grid.set_cols(n_cols);
    m.grid.set_rows_cols(n_rows, n_cols);


    let angle_rotate = m.touchosc.fader("/angle/rotate");
        
    let closure = |v:Vec2, rows:usize, cols:usize| -> f32 {   
        (v.x / rows as f32) * PI + (angle_rotate * cols as f32)
    };
    m.grid.set_angles_by_index(closure);

   

    //let angle_rotate_selected = m.touchosc.radial("/angle/rotate-selected");
    //println!("{}", angle_rotate_selected);
    //m.grid.set_editable_cells_angle(angle_rotate_selected);

    m.line_length = m.touchosc.fader("/line-length");

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
        // let bg = m.colors.vapor_blue;
        let radar = m.touchosc.radar("/color/background");
        let bg = hsva(radar.y, radar.x, radar.x, 1.0);
        

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


        /*
        for n in 0..50 {
            // let draw = draw.translate(vec3(-win.w()*0.5, win.h()*0.5, 0.0));
            draw.ellipse()
            .x_y(x, y)
            .radius(5.0)
            .color(WHITE);

            let angle = m.grid.get_nearest_cell_angle( pt2(x, y) );

            let x_step = step_len * angle.cos();
            let y_step = step_len * angle.sin();

             if n > 0 {
                draw.line()
                .start_pos(pt2(x, y))
                .end(last_xy)
                .color(WHITE);
            }

            last_xy = pt2(x, y);
d
            x = x + x_step;
            y = y + y_step;
        }
        */

        //--------------------------------------------------------
        if m.touchosc.button("/toggle/curves") {
            for i in 0..m.curve_starting_points.len() {

                let start_pos = m.curve_starting_points[i];
                let mut x = start_pos.x;
                let mut y = start_pos.y;
                let mut last_pt = pt2(x, y);
        
                for n in 0..50 {
                    let pt = pt2(x, y);
                    let angle = m.grid.get_nearest_cell_angle(pt2(x, y));
                    let x_step = m.line_length * angle.cos();
                    let y_step = m.line_length * angle.sin();

                    if n > 0 {
                        let mut builder = nannou::geom::path::Builder::new().with_svg();
                        //builder.line_to(lyon::math::point(last_pt.x, last_pt.y));

                        builder.line_to(lyon::math::point(last_pt.x, last_pt.y));

                        builder.quadratic_bezier_to(
                            lyon::math::point(last_pt.x, last_pt.y),
                            lyon::math::point(last_pt.x, last_pt.y)
                        );

                        builder.quadratic_bezier_to(
                            lyon::math::point(pt.x, pt.y),
                            lyon::math::point(pt.x, pt.y)
                        );

                        // end control point
                        builder.move_to(lyon::math::point(pt.x, pt.y));
                        builder.close();

                        let path = builder.build();

                        draw.path()
                        .stroke()
                        .color(WHITE)
                        .events(path.iter());
                    }

                    last_pt = pt2(x, y);

                    x = x + x_step;
                    y = y + y_step;
                }
            }            
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

fn mouse_pressed(app: &App, m: &mut Model, button: MouseButton) {
    m.mouse_pressed = true; 

   // m.grid.toggle_editable_cell( vec2(app.mouse.x, app.mouse.y));
    
}

fn mouse_released(app: &App, m: &mut Model, button: MouseButton) {
    m.mouse_pressed = false;

    
}

fn key_pressed(app: &App, m: &mut Model, key: Key) {
     if key == Key::Space {
        m.curve_starting_points.clear();
     }
}

fn mouse_moved(app: &App, m: &mut Model, pos: Point2) {
    if m.mouse_pressed {
         m.curve_starting_points.push(pos);
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