use vex_rt::prelude::Error;

use crate::devices::motor_group::MotorGroup;

pub struct Intake {
    pub intake: MotorGroup,
}

impl Intake {
    pub fn manual_control(&mut self, is_on: bool) -> Result<(), Error> {
        if (is_on) {
            self.intake.move_i8(127)?;
        }

        Ok(())
    }
}
