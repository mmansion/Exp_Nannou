use nannou::{prelude::*, lyon::geom::arrayvec::Array};
use nannou_osc as osc;
use derivative::Derivative;

use std::collections::HashMap;

pub struct TouchOscClient {
    osc_base_addr: String,
    osc_port: u16,
    osc_receiver: osc::Receiver,
    pub touchosc_faders: Vec<TouchOscFader>,
    pub hashmap_faders:HashMap<String, TouchOscFader>
    // received_packets: Vec<(std::net::SocketAddr, osc::Packet)>
}

impl TouchOscClient {

    pub fn new(addr:String, port:u16) -> Self {
        let osc_base_addr = addr;
        let osc_port = port;
        let touchosc_faders = Vec::new();

        // Bind an `osc::Receiver` to a port.
        let osc_receiver = osc::receiver(osc_port).unwrap();

        let hashmap_faders = HashMap::new();

         // A vec for collecting packets and their source address.
        // let received_packets = vec![];

        TouchOscClient {
            osc_base_addr,
            osc_port,
            osc_receiver,
            touchosc_faders,
            hashmap_faders
        }
    }

    pub fn update(&mut self) {

        for (packet, addr) in self.osc_receiver.try_iter() {
            for msg in packet.into_msgs() {
                let args = msg.args.unwrap();
                
                match self.hashmap_faders.get_mut(&msg.addr) {
                    Some(fader) => {
                        fader.set_arg (
                            match &args[..] {
                                [osc::Type::Float(x)] => *x,
                                _etc => *fader.arg()
                            }
                        );
                    },
                    None => ( /*do nothing*/)
                }
                

                // for storing faders in vec<>
                // for fader in self.touchosc_faders.iter_mut() {
                //     let _addr = &fader.path;

                //     if msg.addr == fader.path {
                //         fader.set_arg (
                //             match &args[..] {
                //                 [osc::Type::Float(x)] => *x,
                //                 _etc => *fader.arg()
                //             }
                //         );
                //     }
                // }
            }

            for fader in self.touchosc_faders.iter() {
                println!("{} {}", fader.path, fader.arg);
            }
        }
    }

    // pub fn fader(&self, index:usize) -> &TouchOscFader {//borrowed ref
    //     return &self.touchosc_faders[index];
    // }

    pub fn fader(&self, path:&'static str) -> &TouchOscFader {//&'static str

        //book_reviews["Pride and Prejudice"]
        return &self.hashmap_faders[path];

        // Look up the values associated with some keys.
        // let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
        // for &book in &to_find {
        //     match book_reviews.get(book) {
        //         Some(review) => println!("{book}: {review}"),
        //         None => println!("{book} is unreviewed.")
        //     }
        // }
    }

    pub fn add_fader(&mut self, path:&'static str) {
        let name_path = path.to_string();
        self.hashmap_faders.insert(name_path.to_string(), TouchOscFader::new(name_path));

        for (key, value) in &self.hashmap_faders {
            println!("{},{}", key, value.path);
        }
        // self.touchosc_faders.push(TouchOscFader::new(path));
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
    pub path : String, //appened to base osc addr
    arg  : f32,
    // args : Array<f32;0.0>,
    pub value: f32, //always results in first argument
}

impl TouchOscFader {
    
    pub fn new(path:String) -> Self {
        let path = path;
        let arg = 0.0;
        let value = 0.0;
    
        TouchOscFader {
            path,
            arg,
            value
        }
    }

    pub fn set_arg(&mut self, v:f32) {
        self.value = v;
        self.arg = v;
    }

    //not get_arg
    pub fn arg(&self) -> &f32 { //get_ prefix is not used for getters in Rust code.
        return &self.arg;
    }
}