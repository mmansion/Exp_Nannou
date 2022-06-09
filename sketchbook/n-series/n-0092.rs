use nannou::prelude::*;
use nannou_touchosc::TouchOscClient;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    win: window::Id,
    touchosc: TouchOscClient,
}

fn model(app: &App) -> Model {
    let win = app.new_window().size(400, 400).view(view).build().unwrap();
    let mut touchosc = TouchOscClient::new(6555);

    touchosc.add_radio("/invert_bg", 2, 0); //add radio button with 2 options, initialized at zero index

    touchosc.add_grid("/grid", 2, 3.0, 24.0, 10.0);

    touchosc.add_encoder("/grid/rotate", 0.0, PI*2.0, 0.0);

    touchosc.add_fader("/color_r", 0.0, 1.0, 1.0);
    touchosc.add_fader("/color_g", 0.0, 1.0, 0.0);
    touchosc.add_fader("/color_b", 0.0, 1.0, 1.0);
    touchosc.add_fader("/color_a", 0.0, 1.0, 1.0);

    // add button inputs to the client
    touchosc.add_button("/shape1", true);   //toggle button, initialized as true
    touchosc.add_button("/shape2", false);  //toggle button, initialized to false

    


    touchosc.add_fader("/polyline/points", 3.0, 8.0, 4.0);
    
    touchosc.add_xy("/rect/width_height",10.0, 200.0, 10.0);
    touchosc.add_radial("/rect/rotate_off", 0.0, 10.0, 0.0);
    touchosc.add_fader("/rect/stroke_width", 1.0, 10.0, 2.0);

    touchosc.add_radar("/rect/scale_rotate", (0.1, 1.0, 1.0), (0.0, PI/2.0, 0.0));

    touchosc.add_grid("/rgb",3, 0.0, 1.0, 1.0);
    
    Model { win, touchosc }
}

fn update(a: &App, m: &mut Model, _update: Update) {
    m.touchosc.update(); //receive touchosc messages
}

fn view(app: &App, m: &Model, frame: Frame) {
    let draw = app.draw();

    let invert = m.touchosc.radio("/invert_bg");

    if invert > 0 {
        draw.background().color(BLACK);
    } else {
        draw.background().color(WHITE);
    }

    
    let win_w = app.window_rect().w();
    let win_h = app.window_rect().h();

    let shape1 = m.touchosc.button("/shape1");
    let shape2 = m.touchosc.button("/shape2");

    let grid_margin = 100.0;
    let grid_rotate = m.touchosc.encoder("/grid/rotate");

    let rows = m.touchosc.grid("/grid/1");
    let cols = m.touchosc.grid("/grid/2");
   
    let x_space = (win_w - grid_margin) / cols;
    let y_space = (win_h - grid_margin) / rows;
    let x_off = -win_w/2.0 + grid_margin/2.0;
    let y_off = -win_h/2.0 + grid_margin/2.0;
    
    let r_stroke  = m.touchosc.fader("/rect/stroke_width");
    let r_scale   = m.touchosc.radar("/rect/scale_rotate").x;
    let r_rotate  =  m.touchosc.radar("/rect/scale_rotate").y;
    let r_off     = m.touchosc.radial("/rect/rotate_off");
    let r_width   = m.touchosc.xy("/rect/width_height").x;
    let r_height  = m.touchosc.xy("/rect/width_height").y;

    let steps = m.touchosc.fader("/polyline/points").round() as usize;

    let color = rgba(
        m.touchosc.fader("/color_r"),
        m.touchosc.fader("/color_g"),
        m.touchosc.fader("/color_b"),
        m.touchosc.fader("/color_a")
    );

    
    // let draw = draw.rotate(PI/3.0);
    let draw = draw.rotate(grid_rotate).translate(pt3(x_off, y_off, 0.0));
    
    for c in 1..cols as i32 {
        
        for r in 1..rows as i32 {
            let n = rows * cols;
            let f = (c * r) as f32 / n;
            // let w = r_off.sin() * (r_off + PI * 2.0 * f as f32).cos();

            let w = (r_rotate).sin() * (r_rotate + f * PI * 2.0).cos();

            let rotate = r_rotate + (w * r_off);

            let x = x_space * c as f32;
            let y = y_space * r as f32;

        

            if shape1 {
                // let w = r_width * r_scale;
                // let h = r_height * r_scale;
    
                // let r_pts = [ //rectangle vertices
                //     pt2(-w / 2.0,  h / 2.0),
                //     pt2( w / 2.0,  h / 2.0),
                //     pt2( w / 2.0, -h / 2.0),
                //     pt2(-w / 2.0, -h / 2.0),
                //     pt2(-w / 2.0,  h / 2.0),
                // ];
    
                // draw.polyline()
                // .rotate(rotate)
                // .xy(pt2(x, y))
                // .stroke_weight(r_stroke)
                // .color(r_rgb)
                // .points_closed(r_pts);

                let radius = 20.0;
                let points = (0..=360).step_by(360/steps).map(|i| {
                    let radian = deg_to_rad(i as f32);
                    let x = radian.sin() * radius;
                    let y = radian.cos() * radius;
                    pt2(x,y)
                });
                draw.translate(pt3(x, y, 0.0))
                    .polygon()
                    .stroke_weight(r_stroke)
                    .color(color)
                    .points(points);
            }

             if shape2 {

                let radius = 20.0;
                let points = (0..=360).step_by(360/steps).map(|i| {
                    let radian = deg_to_rad(i as f32);
                    let x = radian.sin() * radius;
                    let y = radian.cos() * radius;
                    pt2(x,y)
                });
                draw.translate(pt3(x, y, 0.0))
                    .polygon()
                    .stroke_weight(r_stroke)
                    .color(color)
                    .points(points);
            }

        }
    }



    // if m.touchosc.button("/my-toggle") {
    //     draw.ellipse().color(STEELBLUE);
    // }

    

    // let angles = 12;
    // let radius = 100.0;
    // let x_off = 0.0;
    // let y_off = 0.0;

    // let circle_radius = 10.0;

    // let ring_spacing = 10.0;

    // for i in 0..angles {

    //     let inc =  ( (360 / angles * i) as f32).to_radians();
                
    //     let x = inc.cos() * radius; 
    //     let y = inc.sin() * radius;

    //     draw
    //     .line()
    //     .stroke_weight(1.0)
    //     .caps_round()
    //     .color(BLUE)
    //     .points(pt2(x_off, y_off), pt2(x, y))
    //     ;

    //     draw
    //     .ellipse()
    //     .w_h(circle_radius,circle_radius)
    //     .stroke_weight(1.0)
    //     .color(BLACK)
    //     .x_y(x, y)
    //     ;

    //     //------------------------------------------------------
    //     let spacing = ring_spacing * i as f32;

    //     let r1 = m.touchosc.radar("/my-radar").x;
    //     let r2 = m.touchosc.radar("/my-radar").y;
        
    //     let mut inc = 0.0;

    //     let ring_points = (0..360 + 1).map(|j| {
            
    //         let inc =  ((360 / r2.ceil() as i32 * j) as f32).to_radians();
                
    //         let x = inc.cos() * (i as f32 * r1); 

    //         let y = inc.sin() * (i as f32 * r1);

    //         // inc = inc + r2;

    //         pt2(x, y)
    //     });

    //     draw
    //     .polyline()
    //     .stroke_weight(1.0)
    //     .caps_round()
    //     .color(rgba(0.0, 0.0, 0.0, 1.0))
    //     .points(ring_points)
    //     ;
    // }


    draw.to_frame(app, &frame).unwrap();
}
