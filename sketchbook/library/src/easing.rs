// EASING FUNCTIONS

use nannou::prelude::*;
//https://easings.net/

//https://easings.net/#easeInSine
pub fn ease_in_sine(x: f32) -> f32  {
    return 1.0 - (x * PI ).cos();
}