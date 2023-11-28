use vex_rt::prelude::Error;

use crate::devices::motor_group::MotorGroup;

pub struct TankDrive {
    pub left_side: MotorGroup,
    pub right_side: MotorGroup,
}

impl TankDrive {
    pub fn manual_control(&mut self, left: i8, right: i8) -> Result<(), Error> {
        self.left_side.move_i8(left)?;
        self.right_side.move_i8(right)?;
        Ok(())
    }
}
