use nannou::prelude::*;

pub struct Grid {
    pub position  : Point2,
    pub width     : i32,
    pub height    : i32,
    pub points    : Vec <Vector2>,
    pub ptSize : f32,
}

impl Grid {

    pub fn new(rows: f32, cols: f32, rowSpacing: i32, colSpacing: i32, rect: &Rect) -> Self {
        
        let width    = rect.w() as i32;
        let height   = rect.h() as i32;
        let position = vec2(0.0, 0.0);
        let points   = Vec::new();
        let ptSize   = 10.0;

        //--------------------------------------------------------

        for row in 0..( rowSpacing + 1 ) {
            let y =  ((height / rowSpacing * row) + (-height/2)) as f32;
    
            for col in 0..(colSpacing + 1) {
                let x =  ( (width/colSpacing  * col) + (-width/2) ) as f32;
                
                points.push(pt2(x, y));
            } 
        }

        //--------------------------------------------------------

        Grid {
            position,
            width,
            height,
            points,
            ptSize
        }
    }

    pub fn display(&self, draw: &Draw) {

        for i in 0..self.points.len() {
            draw.ellipse()
            .xy(self.points[i])
            .radius( self.ptSize )
            .color(GRAY); 
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