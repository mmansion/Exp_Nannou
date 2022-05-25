use nannou::prelude::*;
use nannou_osc as osc;
use derivative::Derivative;

pub struct TouchOscClient {
    osc_base_addr: String,
    osc_port: u16,
    osc_receiver: osc::Receiver,
    pub touchosc_faders: Vec<TouchOscFader>,
    // received_packets: Vec<(std::net::SocketAddr, osc::Packet)>
}

impl TouchOscClient {

    pub fn new(addr:String, port:u16) -> Self {
        let osc_base_addr = addr;
        let osc_port = port;
        let touchosc_faders = Vec::new();

        // Bind an `osc::Receiver` to a port.
        let osc_receiver = osc::receiver(osc_port).unwrap();

         // A vec for collecting packets and their source address.
        // let received_packets = vec![];

        TouchOscClient {
            osc_base_addr,
            osc_port,
            osc_receiver,
            touchosc_faders,
        }
    }

    pub fn update(&mut self) {
        
        for (packet, addr) in self.osc_receiver.try_iter() {

            for msg in packet.into_msgs() {

                let args = msg.args.unwrap();

                for fader in self.touchosc_faders.iter_mut() {
                    let _addr = &fader.path;

                    if msg.addr == fader.path {
                        fader.set_arg (
                            match &args[..] {
                                [osc::Type::Float(x)] => *x,
                                _etc => *fader.arg()
                            }
                        );
                    }
                }
            }

            for fader in self.touchosc_faders.iter() {
                println!("{} {}", fader.path, fader.arg);
            }
        }
    }

    pub fn add_fader(&mut self, path:String) {
        self.touchosc_faders.push(TouchOscFader::new(path));
    }

//     pub fn fader(&self, index:u8) -> &f32 {
//         // match str {
//         //     "foo" => foo(),
//         //     "bar" => bar(),
//         //     "baz" => baz(),
//         //     "barfl" => barfl(),
//         //     _ => {}
// }
//     }
}

//--------------------------------------------------------
pub struct TouchOscFader {
    path : String, //appened to base osc addr
    arg  : f32,
}

impl TouchOscFader {
    
    pub fn new(path:String) -> Self {
        let path = path;
        let arg = 0.0;
    
        TouchOscFader {
            path,
            arg
        }
    }

    pub fn set_arg(&mut self, v:f32) {
        self.arg = v;
    }

    //not get_arg
    pub fn arg(&self) -> &f32 { //get_ prefix is not used for getters in Rust code.
        return &self.arg;
    }
}