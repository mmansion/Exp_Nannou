use nannou::prelude::*;
use nannou::prelude::Point2;
// use std::ops::Range;
use nannou::noise::*;
// use nannou::Draw;
// use std::collections::VecDeque;
use std::time::Duration;

//--------------------------------------------------------

static CAPTURE  : bool = true; // capture to image sequence
static WIDTH    : f32 = 800.0;
static HEIGHT   : f32 = 800.0; 

static RESOLUTION: f32 = 10.0;
static ITERATIONS: i32 = 3;
static POINT_SIZE: f32 = 20.0;

//--------------------------------------------------------

pub mod lib;

// module tree
use crate::lib::grid::Grid as Grid;
use crate::lib::vehicles::Vehicle as Vehicle;

fn main() {
    nannou::app(model).update(update).run();
}

struct Anim {
    position : Vector2,
    size     : f32,
    min_size : f32,
    max_size : f32,
    pub animate  : bool, 
    speed    : f32,   
    angles   : i32,
}
impl Anim {

    pub fn new(position:Vector2, speed:f32, size:f32, max_size:f32) -> Self {

        let position    = position;
        let speed       = speed;
        let mut size        = size;
        let min_size    = size;
        let max_size    = max_size;
        let animate     = false;
        
        let angles = 360;

        Anim {
            position,
            size,
            min_size,
            max_size,
            animate,
            speed,
            angles,
        }

        
    }

    pub fn update(&mut self) {

        self.size += self.speed;
        if self.size > self.max_size {
            self.animate = false;
            self.size = self.min_size;
        }
    }

    pub fn display(&self, draw: &Draw) {

        let pts = (0..self.angles + 1).map(|i| {

            let inc =  ( (360 / self.angles * i) as f32).to_radians();
                    
            let x = inc.cos() * self.size; 
            let y = inc.sin() * self.size;

            pt2(x, y)
        });

        if self.animate {
            draw
            .xy(self.position)
            .scale(1.0)
            .polygon()
            .no_fill()
            .stroke(rgba(1.0,1.0,1.0, map_range(
                self.size, self.min_size, self.max_size, 1.0, 0.0)))
            .stroke_weight(map_range(
                self.size, self.min_size, self.max_size, 0.0, 10.0))
            .points(pts)
            ;
        } 
    }
}

struct Model {
    this_capture_frame : i32,
    last_capture_frame : i32,
    last_calc : Duration,
    touched_points: Vec<bool>,
    vehicles  : Vec<Vehicle>,
    anims     : Vec<Anim>,
    grid      : Grid,
    noise     : Perlin,
    noiseGen  : Point2,
    xOff      : f32, 
    yOff      : f32,
}

// ----------------------------------------------------------------------
fn model(app: &App) -> Model {

    let rect = Rect::from_w_h( WIDTH, HEIGHT );

    //app.set_loop_mode(LoopMode::loop_once());
    
    // app.set_loop_mode(LoopMode::loop_once());
    // app.set_loop_mode(LoopMode::rate_fps(0.1));
    
    app.new_window()
        .size(800, 800)
        .view(view)
        .build()
        .unwrap();

    let grid = Grid::new(10.0, 10.0, 10, 10, &rect);

    let mut touched_points = Vec::new();
    let mut anims = Vec::new();

    for i in 0..grid.points.len() {
        touched_points.push(true);
        anims.push( Anim::new(grid.points[i], 0.1, POINT_SIZE, POINT_SIZE * 5.0));
    }
    

    //--------------------------------------------------------
    let mut vehicles = Vec::new();

    for i in 0..ITERATIONS {
        let randX    = random_f32() * (HEIGHT/2.0);
        let randY    = random_f32() * (WIDTH/2.0);
        let maxspeed = 5.0;
        let velocity = vec2(randX, randY);
        let length = 20;

        vehicles.push( Vehicle::new(0.0, 0.0, maxspeed, velocity.limit_magnitude(maxspeed), length) );
    }

    //--------------------------------------------------------
    let mut noise = Perlin::new();
    noise = noise.set_seed(1);
    let mut noiseGen = pt2(0.0, 0.0);

    let xOff = 0.0;
    let yOff = 0.0;

    //--------------------------------------------------------

    let mut last_calc = Duration::from_millis(0);
    //----------------------------------
    let mut this_capture_frame = 0;
    let mut last_capture_frame = 0;

    //----------------------------------

    Model {
        this_capture_frame, 
        last_capture_frame, 
        last_calc,
        // texture,
        anims,
        grid,
        noise,
        noiseGen,
        xOff,
        yOff,
        vehicles,
        touched_points,
    }
} 

fn update(app: &App, m: &mut Model, _update: Update) {

    // ref:
    //https://doc.rust-lang.org/nightly/core/time/struct.Duration.html
    //let millis = Duration::from_millis(100).as_millis();
    let since_last_calc =  _update.since_start.as_millis() - m.last_calc.as_millis();
    
    // println!("{}", since_last_calc);
    
    if since_last_calc > 10  {

        m.last_calc = _update.since_start;

        for i in 0..m.grid.points.len() {
            m.touched_points[i] = false; // reset 
        }

        for v in 0..m.vehicles.len() {

            for i in 0..m.grid.points.len() {
                if m.vehicles[v].hasCollision(m.grid.points[i], POINT_SIZE) {
                    m.touched_points[i] = true; 
                    m.anims[i].animate = true;
                }
            }
        }
    }
    

    if m.this_capture_frame != m.last_capture_frame {
        m.last_capture_frame = m. this_capture_frame;
    }

    if CAPTURE {
        m.this_capture_frame += 1;
    }

    //--------------------------------------------------------
    //calculations here

    // noise
    m.noiseGen.x+=0.1;
    m.noiseGen.y+=0.1;

    m.xOff = m.noise.get([m.noiseGen.x as f64, m.noiseGen.y as f64]) as f32;
    m.yOff = m.noise.get([m.noiseGen.x as f64, m.noiseGen.y as f64]) as f32;
    //--------------------------------------------------------
    
    //vehicles

    for v in 0..m.vehicles.len() {

        for i in 0..m.grid.points.len() {
            
            m.vehicles[v].redirect2( m.grid.points[i], POINT_SIZE );

            m.anims[i].update();
        }

        // for i in 0..m.points.len() {

        //     m.vehicles[v].redirect( &m.points[i] );
        //     // let steer = force.limit_magnitude(m.vehicles[v].max_force);
        //     // m.vehicles[v].apply_force(steer);
        // }
        
        m.vehicles[v].boundaries(&app.window_rect());
        m.vehicles[v].update();
    }

}

fn get_value(x:f32, y:f32) -> f32 {
    return (x*1.1).cos() +  ((y*0.1).sin() * (PI * 2.0));
    // return (x*0.01).cos() +  ((y*0.001).sin() * (PI * 2.0));
}

fn view(app: &App, m: &Model, frame: Frame) {

    // get canvas to draw on
    let draw  = app.draw();
    let win   = app.window_rect();
    let t     = app.time;

    // draw background----------------------------------------------------------

    let bg = rgba(0.0, 0.0, 0.2, 0.1);

    if app.elapsed_frames() == 1 { //initial background pass
        draw.background().color(rgba(0.0, 0.0, 0.0, 0.1));
    } else {
        draw.rect().x_y(0.0, 0.0).w_h(win.w()*2.0, win.w()*2.0).color(bg);
    }

    //--------------------------------------------------------
    m.grid.draw(&draw);
    // let draw = draw.rotate(t * 0.8);

    // let draw = draw.scale(0.5);
    // let draw = draw.translate(pt3(-WIDTH/2.0, -HEIGHT/2.0, 0.0));


    for i in 0..ITERATIONS {
       
        let x = random_f32() * WIDTH;
        let y = random_f32() * HEIGHT;
        let r = get_value(x * (t.cos() * 0.3), y * (t.sin() * 1.1));

        let start_pt = pt2(0.0, 0.0);
        let end_pt   = pt2(RESOLUTION + (t.sin() * 0.1), 0.0);

        // draw
        // .line()
        // .x_y(x, y)
        // .rotate(r)
        // .stroke_weight(map_range(y, 0.1, HEIGHT, 1.0, 100.0))
        // .color(hsv(
        //     map_range(y + t.sin(), 0.0, WIDTH, 0.1, 0.4), 
        //     1.0, 
        //     map_range(y + t.sin(), 0.0, WIDTH, 0.1, 0.4)))
        // .points(start_pt, end_pt)
        // ;

    }

    // draw
    // .x_y(WIDTH/2.0, HEIGHT/2.0)
    // .ellipse()
    // .color(BLACK)
    // .radius(200.0)
    // ;

    //--------------------------------------------------------

    let mut pts = Vec::new();
    for v in 0..m.vehicles.len() {

        //display(&model.vehicles[v], &draw, &app, v as i32, 0.4);
        pts.push(m.vehicles[v].position);
    }

    let color = hsva (  t.sin() * 0.1, 1.0, 1.0, 1.0);

    draw.polygon()
        .stroke_weight(1.5)
        .caps_round()
        .stroke(color)
        .no_fill()
        .points(pts)
        ;
    
    for i in 0..m.grid.points.len() {


        if m.touched_points[i] { 
            draw
            .ellipse()
            .xy(m.grid.points[i])
            .color( WHITE )
            .radius(POINT_SIZE)
            ;
        } else {
            draw
            .ellipse()
            .xy(m.grid.points[i])
            .color( color )
            .radius(POINT_SIZE)
            ;
        }
        

        m.anims[i].display(&draw);
    }


    //--------------------------------------------------------
    
    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();

    // capture fram -----------------------------------------------------------

    if m.this_capture_frame != m.last_capture_frame {
            
        // let mut owned_string: String = "hello ".to_owned();
        // let borrowed_string: String = "output/" + app.exe_name().unwrap() + ".png";
    
        let directory  = "captures/".to_string();
        let app_name   = app.exe_name().unwrap().to_string();
        // let frame_num  = m.this_capture_frame.to_string();
        let extension  = ".png".to_string();

        let frame_num = format!("{:05}", m.this_capture_frame);

        let path = format!("{}{}{}", directory, frame_num, extension);

        app.main_window().capture_frame(path);
        
    }
}