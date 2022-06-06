use nannou::prelude::*;

const TOTAL_COLORS: usize = 6;
pub struct Palette {
    pub mango: Rgb8,
    pub orange_pantone: Rgb8,
    pub winter_sky: Rgb8,
    pub blue_violet: Rgb8,
    pub azure: Rgb8,
    pub black: Rgb8,
    pub col_arr: [Rgb8; TOTAL_COLORS],
}

impl Palette {
    pub fn new() -> Self {
        let mango = rgb8(255, 190, 11);
        let orange_pantone = rgb8(251, 86, 7);
        let winter_sky = rgb8(255, 0, 110);
        let blue_violet = rgb8(131, 56, 236);
        let azure = rgb8(58, 134, 255);
        let black = rgb8(0, 0, 0);

        let col_arr: [Rgb8; TOTAL_COLORS] =
            [mango, orange_pantone, winter_sky, blue_violet, azure, black];

        Palette {
            mango,
            orange_pantone,
            winter_sky,
            blue_violet,
            azure,
            black,
            col_arr,
        }
    }
    pub fn get_random(&self) -> Rgb8 {
        let i = random_range(0, self.col_arr.len());
        return self.col_arr[i];
    }
}
