use nannou::prelude::*;

static WIDTH  : i32 = 800;
static HEIGHT : i32 = 800; 
static DIVS   : i32 = 16;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    points : Vec<Vector2>, // points bin no.1
    this_capture_frame : i32,
    last_capture_frame : i32,
}

fn model(app: &App) -> Model {

    let rect = Rect::from_w_h( WIDTH, HEIGHT );

    app.new_window()
        .size(rect.w() as u32, rect.h() as u32)
        .view(view)
        .build()
        .unwrap();

    let mut points  = Vec::new();
    let mut this_capture_frame = 0;
    let mut last_capture_frame = 0;

    for row in 0..(DIVS+1) {

        let y =  (HEIGHT/DIVS * row) as f32;

        for col in 0..(DIVS+1) {

            let x =  (WIDTH/DIVS  * col) as f32;
            
            points.push(pt2(x, y));
        }

        
    }

    Model { points, this_capture_frame, last_capture_frame }
} 

fn update(app: &App, model: &mut Model, _update: Update) {

    if model.this_capture_frame != model.last_capture_frame {
        model.last_capture_frame = model. this_capture_frame;
    }

    if app.keys.down.contains(&Key::S) {
        model.this_capture_frame += 1;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {

    // get canvas to draw on
    let draw = app.draw();

    // set background to blue
    let r = 10.0;
    //let bg = hsv ( map_range( app.time % r, 0.0 , r, 0.0 , 1.0), 1.0, 1.0);
    let bg = hsva ( 0.9, 1.0, 1.0, 1.0);
    //let bg = hsva ( map_range( app.time.sin(), 0.0, 1.0, 0.3, 0.75), 1.0, 1.0, 0.1);

    draw.background().color( BLACK);

    let draw = draw.x_y((-WIDTH/2) as f32, (-HEIGHT/2) as f32);

    let t = app.time;


    println!("{}", model.points.len());
    for i in 0..model.points.len() {

        // println!( "{},{}", model.points[i].x, model.points[i].y );
        // let color = hsv( (t * 0.001 * i as f32).sin(), 1.0, 1.0);
        let mut color = hsva ( map_range( i, 0 , model.points.len() , 0.4 , 0.9), 1.0, 1.0, 1.0);

        // if i > model.points.len() / 2  {
        //     color = hsva ( map_range( i, 0 , model.points.len() , 0.4 , 0.7), 1.0, 1.0, 1.0);   
        // } 
        draw.ellipse()
        .x_y(model.points[i].x, model.points[i].y)
        .radius( ( (t*0.9) + i as f32).sin() * model.points.len() as f32 )
        .color(color); 
        


        
    }


    // put everything on the frame
    draw.to_frame(app, &frame).unwrap();

    if model.this_capture_frame != model.last_capture_frame {
        
        // let mut owned_string: String = "hello ".to_owned();
        // let borrowed_string: String = "output/" + app.exe_name().unwrap() + ".png";
    
        let directory  = "captures/".to_string();
        let app_name   = app.exe_name().unwrap().to_string();
        // let frame_num  = model.this_capture_frame.to_string();
        let extension  = ".png".to_string();

        let frame_num = format!("{:05}", model.this_capture_frame);

        let path = format!("{}{}{}", directory, frame_num, extension);

        app.main_window().capture_frame(path);
        
    }
}