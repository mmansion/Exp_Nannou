use nannou::prelude::*;

const TOTAL_COLORS: usize = 10;
pub struct Palette {
    pub mango: Rgba8,
    pub orange_pantone: Rgba8,
    pub winter_sky: Rgba8,
    pub blue_violet: Rgba8,
    pub azure: Rgba8,
    pub black: Rgba8,
    pub vapor_red: Rgba8,
    pub vapor_pink: Rgba8,
    pub vapor_purple: Rgba8,
    pub vapor_blue: Rgba8,
    pub col_arr: [Rgba8; TOTAL_COLORS],
}

impl Palette {
    pub fn new() -> Self {
        let mango = rgba8(255, 190, 11, 255);
        let orange_pantone = rgba8(251, 86, 7, 255);
        let winter_sky = rgba8(255, 0, 110, 255);
        let blue_violet = rgba8(131, 56, 236, 255);
        let azure = rgba8(58, 134, 255, 255);
        let black = rgba8(0, 0, 0, 255);
        let vapor_red = rgba8(217, 4, 61, 255);
        let vapor_pink = rgba8( 242, 68, 132, 255);
        let vapor_purple = rgba8(169, 156, 217, 255);
        let vapor_blue = rgba8(107, 204, 242, 255);

        let col_arr: [Rgba8; TOTAL_COLORS] =
            [
                mango, 
                orange_pantone, 
                winter_sky, 
                blue_violet, 
                azure, 
                vapor_red,
                vapor_pink,
                vapor_purple,
                vapor_blue,
                black
            ];

        Palette {
            mango,
            orange_pantone,
            winter_sky,
            blue_violet,
            azure,
            vapor_red,
            vapor_pink,
            vapor_purple,
            vapor_blue,
            black,
            col_arr,
        }
    }
    pub fn get_random(&self) -> Rgba8 {
        let i = random_range(0, self.col_arr.len());
        return self.col_arr[i];
    }
}
