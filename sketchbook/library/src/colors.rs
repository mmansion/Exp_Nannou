use nannou::prelude::*;

pub struct Palette {
    pub mango : Rgb8,
    pub orange_pantone: Rgb8,
    pub winter_sky: Rgb8,
    pub blue_violet: Rgb8,
    pub azure: Rgb8,
}
 
impl Palette {
    pub fn new() -> Self {
        let mango = rgb8(255, 190, 11);
        let orange_pantone = rgb8(251, 86, 7);
        let winter_sky = rgb8(255, 0, 110);
        let blue_violet = rgb8(131, 56, 236);
        let azure = rgb8(58, 134, 255);

        Palette {
            mango,
            orange_pantone,
            winter_sky,
            blue_violet,
            azure
        }
    }
}