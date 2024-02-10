use core::mem;

use alloc::string::String;
use vex_rt::{
    adi::{AdiDigitalOutput, AdiPort},
    prelude::Error,
};

pub enum Pneumatic {
    Connected(AdiDigitalOutput),
    Disconnected(Option<AdiPort>),
}

impl Pneumatic {
    pub fn new(port: AdiPort) -> Pneumatic {
        return Pneumatic::Disconnected(Some(port));
    }

    pub fn connect(&mut self) -> Result<&mut AdiDigitalOutput, Error> {
        // match statements are just switch statements that MUST provide a case for all enum values
        match self { 
            Pneumatic::Connected(device) => Ok(device),
            Pneumatic::Disconnected(port) => {
                // Takes ownership of the AdiPort output
                let taken_port = mem::take(port); 
                // Creates a AdiDigitalOutput object from the port
                let device = taken_port
                    .unwrap()
                    .into_adi_digital_output()
                    // Swaps the AdiDigitalOutError object for a custom error string
                    .map_err(|_| Error::Custom(String::from("error")))?;

                mem::swap(self, &mut Pneumatic::Connected(device));

                match self {
                    Pneumatic::Connected(motor) => Ok(motor),
                    Pneumatic::Disconnected(..) => unreachable!(),
                }
            }
        }
    }
}
