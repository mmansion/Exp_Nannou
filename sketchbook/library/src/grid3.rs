use nannou::prelude::*;

pub struct Grid3 {
    pub width     : i32,
    pub height    : i32,
    pub points    : Vec <Vec2>,
    pub pt_size   : f32,
    pub angles    : Vec<Vec2>,
    row_spacing   : i32,
    col_spacing   : i32,
    row_start_pts : Vec <Vec2>,
    col_start_pts : Vec <Vec2>,
}

impl Grid3 {

    pub fn new(rows: i32, cols: i32, width: i32, height:i32) -> Self {
        
        let width    = width;
        let height   = height;

        let mut points    = Vec::new();
        let mut row_start_pts = Vec::new();
        let mut col_start_pts = Vec::new();
        
        let pt_size  = 5.0;

        let row_spacing = width/rows;
        let col_spacing = height/rows;

        //--------------------------------------------------------
        let mut angles = Vec::new();

        //--------------------------------------------------------

        //offset for coord sys
        let y_off = -height/2;
        let x_off = -width/2;

        for row in 0..(rows+1) {
            let y =  (height / rows * row + y_off) as f32;
            
            for col in 0..(cols+1) {
                let x =  (width / cols  * col + x_off) as f32;
                
                points.push(pt2(x, y));

                angles.push(vec2(1.0, 1.0));

                if row == 0 {
                    col_start_pts.push(pt2(x, y));
                }
                if col == 0 {
                    row_start_pts.push(pt2(x,y));
                }
            } 
        }

        //--------------------------------------------------------

        Grid3 {
            width,
            height,
            points,
            pt_size,
            row_spacing,
            col_spacing,
            row_start_pts,
            col_start_pts,
            angles
        }
    }

    pub fn set_angle(&mut self, index:usize, angle:Vec2) {
        self.angles[index] = angle;
    }

    pub fn draw(&self, draw: &Draw) {


        // draw row lines
        for r in 0..self.row_start_pts.len() {
   
            let start_pt = self.row_start_pts[r];

            let end_pt = pt2(
                self.row_start_pts[r].x + self.width as f32, 
                self.row_start_pts[r].y );

            draw
            .line()
            .color(GRAY)
            .points(start_pt, end_pt)
            ;
        }

        // draw col lines
        for c in 0..self.col_start_pts.len() {

            let start_pt = self.col_start_pts[c];

            let end_pt = pt2(
                self.col_start_pts[c].x, 
                self.col_start_pts[c].y + self.height as f32 );

            draw
            .line()
            .color(GRAY)
            .points(start_pt, end_pt)
            ;
        }

        // draw points

        for p in 0..self.points.len() {
            draw.ellipse()
            .xy(self.points[p])
            .radius( self.pt_size )
            .color(BLACK); 
        }
        
    }
}