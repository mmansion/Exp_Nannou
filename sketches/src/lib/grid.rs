use nannou::prelude::*;

pub struct Grid {
    pub position  : Point2,
    pub width     : i32,
    pub height    : i32,
    pub points    : Vec <Vector2>,
    pub pt_size   : f32,

    row_spacing : i32,
    col_spacing : i32,
    row_start_pts : Vec <Vector2>,
    col_start_pts : Vec <Vector2>,
}

impl Grid {

    pub fn new(rows: f32, cols: f32, row_spacing: i32, col_spacing: i32, rect: &Rect) -> Self {
        
        let width    = rect.w() as i32;
        let height   = rect.h() as i32;

        let mut position  = vec2(0.0, 0.0);
        let mut points    = Vec::new();
        let mut row_start_pts = Vec::new();
        let mut col_start_pts = Vec::new();
        
        let pt_size  = 3.0;

        let row_spacing = row_spacing;
        let col_spacing = col_spacing;


        //--------------------------------------------------------

        for row in 0..( row_spacing + 1 ) {
            let y =  ((height / row_spacing * row) + (-height/2)) as f32;
            
            for col in 0..(col_spacing + 1) {
                let x =  ( (width/col_spacing  * col) + (-width/2) ) as f32;
                
                points.push(pt2(x, y));

                if row == 0 {
                    col_start_pts.push(pt2(x, y));
                }
                if col == 0 {
                    row_start_pts.push(pt2(x,y));
                }
            } 
        }

        //--------------------------------------------------------

        Grid {
            position,
            width,
            height,
            points,
            pt_size,
            row_spacing,
            col_spacing,
            row_start_pts,
            col_start_pts
        }
    }

    pub fn draw(&self, draw: &Draw) {

        for p in 0..self.points.len() {
            draw.ellipse()
            .xy(self.points[p])
            .radius( self.pt_size )
            .color(BLACK); 
        }

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

     



        
    }
}

//--------------------------------------------------------

// pub struct GridPoint {
//     pub position: Point2,
//     pub size : f32,
//     pub show : bool
// }

// impl GridPoint {
//     pub fn new(x: f32, y: f32, s: f32) -> Self {
//         let position = pt2(x, y);
//         let size = s;
//         let show = false;

//         GridPoint {
//             position,
//             size,
//             show
//         }
//     }
//     pub fn display(&self, draw: &Draw) {

//         if(self.show) {
//             draw.ellipse()
//             .xy(self.position)
//             .radius( self.size )
//             .color(GRAY);
//         }
//     }
// }