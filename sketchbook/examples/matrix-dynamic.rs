use nannou::{prelude::*, draw::background::new};
use std::ops::Index;

pub struct Matrix<T> {
    data: Vec<Vec<T>>,
}

impl<T> Matrix<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        Self { data }
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

fn main() {

    // let matrix = Matrix::new(Vec::new());
    let mut matrix: Matrix<Point2> = Matrix::new( Vec::new() );

    for col in 0..3 {

        let x = 0.0;
        let y = col as f32;

        let row = vec![
            pt2(x, y),
            pt2(x+1.0, y),
            pt2(x+2.0, y),
            pt2(x+3.0, y),
        ];

        matrix.data.push(row);
    }


    for col in 0..matrix.data.len() {
        for row in 0..matrix.data[col].len() {
            println!("{}", matrix.data[col][row]);
        }

    }

    //--------------------------------------------------------

    nannou::sketch(view)
        .size(800, 800)
        .loop_mode(LoopMode::loop_once())
        .run();
}


fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    draw.background().color(WHITE);

    draw.to_frame(app, &frame).unwrap();
}
