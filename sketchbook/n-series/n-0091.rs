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
use library::grid3::Grid3 as Grid;

// beginning of touch library for nannou
use library::touchosc5::TouchOscClient as TouchOscClient;

//--------------------------------------------------------
static CAPTURE  : bool = false; // capture to image sequence (or use obs)
static FRAME    : bool = true; //hide window chrome when set to false
static WIDTH    : i32 = 800;
static HEIGHT   : i32 = 800; 
static WAIT     : u128 = 100;

static NUM_FADERS  : usize  = 4; //num of sliders used 

const OSC_PORT_RX: u16 = 6555;

//--------------------------------------------------------
fn main() {
    nannou::app(model).update(update).run();
}

//--------------------------------------------------------
struct Model {
    window_id: WindowId,
    this_capture_frame : i32,
    last_capture_frame : i32,
    last_calc : Duration,
    colors:Palette,
    redraw:bool,
    last_redraw: u128,
    touchosc: TouchOscClient,
    points: Vec<Point2>,
    grid: Grid
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
        .unwrap()
        ;
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
    let mut touchosc = TouchOscClient::new(OSC_PORT_RX);

    //grid controls
    touchosc.add_fader("/grid/rows", 0.0, 12.0, 6.0);
    touchosc.add_fader("/grid/cols", 0.0, 12.0, 6.0);
    touchosc.add_radial("/grid/rotation", 0.0, PI*2.0, 0.0);

    // rect controls
    touchosc.add_button("/rect/toggle", true);
    touchosc.add_encoder("/rect/rotation", 0.0, PI*2.0, 0.0);
    touchosc.add_xy("/rect/width_height", 50.0, 300.0, 50.0);

    touchosc.add_radar("/rect/scale_rotation", (1.0, 2.0, 1.0), (0.0, PI*2.0, 0.0));
    
    //touchosc.add_grid("/rect/rgba", 3, 0.0, 1.0, 1.0);

    touchosc.add_fader("/rect/weight", 1.0, 10.0, 1.0);

    touchosc.add_radio("/radio", 3, 1);
    

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
        touchosc,
        points,
        grid
    }
} 

fn update(app: &App, m: &mut Model, _update: Update) {

    // ref:
    //https://doc.rust-lang.org/nightly/core/time/struct.Duration.html
    //let millis = Duration::from_millis(100).as_millis();
    let since_last_calc = _update.since_start.as_millis() - m.last_calc.as_millis();
    if since_last_calc > 10  { //time interval
        m.last_calc = _update.since_start;
    }

    if m.this_capture_frame != m.last_capture_frame {
        m.last_capture_frame = m. this_capture_frame;
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
    m.touchosc.update();//update vals

    let num_rows = m.touchosc.fader("/grid/rows") as i32;
    let num_cols = m.touchosc.fader("/grid/cols") as i32;
    
    // println!("{}, {}", num_rows, num_cols);

    m.grid.rows(num_rows);
    m.grid.cols(num_cols);

    m.grid.rotation( m.touchosc.radial("/grid/rotation"));


}

fn view(app: &App, m: &Model, frame: Frame) {

    if(m.redraw) {
        // get canvas to draw on
        let draw  = app.draw();
        let win   = app.window_rect();
        let time  = app.time;
    
        //--------------------------------------------------------
        // background
        // let bg = m.colors.get_random();
        let bg = m.colors.mango;
    
        if app.elapsed_frames() < 10 { //must clear render context once for fullscreen
            draw.background().color(BLACK);
        } else {
            draw.rect().x_y(0.0, 0.0).w_h(win.w()*2.0, win.w()*2.0).color(bg);
        }
        
        //--------------------------------------------------------
        m.grid.draw(&draw);

        if m.touchosc.button("/rect/toggle") {
        
            for i in 0..m.grid.points.len() {

                // let rotation = m.grid.angles[i];
                let pt = m.grid.points[i];
                let x = pt.x;
                let y = pt.y;

                let w  = m.touchosc.xy("/rect/width_height").x;
                let h  = m.touchosc.xy("/rect/width_height").y;
                let wt = m.touchosc.fader("/rect/weight");

                let scale = m.touchosc.radar("/rect/scale_rotation").x;
                let rotation = m.touchosc.radar("/rect/scale_rotation").y;

                // Return a new rotated draw instance.
                // This will rotate both the rect and text around the origin.
                let f = i as f32 / m.grid.points.len() as f32;
                let rotate = (rotation).sin() * (rotation + f * PI * 2.0).cos();
                let draw = draw.translate(pt3(x, y, 0.0));
                let draw = draw.rotate(rotate);
                let draw = draw.scale(scale);

  
                let pts = [
                    pt2(- w / 2.0, h / 2.0),
                    pt2(w / 2.0,  h / 2.0),
                    pt2(w / 2.0, - h / 2.0),
                    pt2(- w / 2.0, - h / 2.0),
                    pt2(- w / 2.0, h / 2.0),

                ];

                draw.polyline()
                .xy(pt2(0.0,0.0))
                .stroke_weight(wt)
                .color(BLACK)
                .points_closed(pts)
                ;

            }

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
            let directory  = "captures/".to_string();
            let app_name   = app.exe_name().unwrap().to_string();
            let extension  = ".png".to_string();
            let frame_num  = format!("{:05}", m.this_capture_frame);
    
            let path = format!("{}{}{}", directory, frame_num, extension);
            app.main_window().capture_frame(path);
        }
    }

}

fn mouse_pressed(_app: &App, m: &mut Model, _button: MouseButton) {

    
}