use vex_rt::prelude::Error;

use crate::devices::pneumatic::{self, Pneumatic};

pub struct Flap {
    pub pneumatic: Pneumatic,
    prev_state: bool,
    current_state: bool,
}

const OPEN: bool = true;
const CLOSED: bool = false;

impl Flap {
    pub fn new(pneumatic: Pneumatic) -> Flap {
        Flap {
            pneumatic,
            prev_state: CLOSED,
            current_state: CLOSED,
        }
    }

    pub fn manual_control(&mut self, open: bool) -> Result<(), Error> {
        if open && self.prev_state == CLOSED {
            self.current_state = !self.current_state;
            self.pneumatic.connect()?.write(self.current_state)?;
        }

        self.prev_state = self.current_state;
        Ok(())
    }
}
