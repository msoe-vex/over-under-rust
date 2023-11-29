use vex_rt::prelude::*;

use crate::devices::pneumatic::Pneumatic;

pub struct Flap {
    pub pneumatic: Pneumatic,
    prev_button_state: bool,
    current_flap_state: bool,
}

const OPEN: bool = true;
const CLOSED: bool = false;

impl Flap {
    pub fn new(pneumatic: Pneumatic) -> Flap {
        Flap {
            pneumatic,
            prev_button_state: false,
            current_flap_state: CLOSED,
        }
    }

    pub fn manual_control(&mut self, button_pressed: bool) -> Result<(), Error> {
        if button_pressed && self.prev_button_state == false {
            self.current_flap_state = !self.current_flap_state;
            self.pneumatic.connect()?.write(self.current_flap_state)?;
        }

        self.prev_button_state = button_pressed;
        Ok(())
    }
}
