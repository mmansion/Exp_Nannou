use nannou::prelude::*;
use nannou_osc as osc;

//--------------------------------------------------------
pub struct TouchOscFader {
    pub osc_address : String,
    pub osc_value   : f32
}

impl TouchOscFader {
    pub fn new(addr:String, val:Option<f32>) -> Self {
        let osc_address = addr;
        
        let osc_value = match val {
            Some(number) => number,
            None => 0.0
        };

        TouchOscFader {
            osc_address,
            osc_value
        }
    }

    pub fn set_value(&mut self, value:f32) {
        self.osc_value = value;
    }
}