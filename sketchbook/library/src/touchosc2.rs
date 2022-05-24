use nannou::prelude::*;
use nannou_osc as osc;

//--------------------------------------------------------
pub struct TouchOscFader {
    pub osc_address : String,
    pub osc_value   : f32
}

impl TouchOscFader {
    pub fn new(addr:String, val:f32) -> Self {
        let osc_address = addr;
        let osc_value = val;

        TouchOscFader {
            osc_address,
            osc_value
        }
    }

    pub fn set_value(&mut self, value:f32) {
        self.osc_value = value;
    }
}