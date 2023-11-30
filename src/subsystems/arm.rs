use vex_rt::prelude::Error;

use crate::devices::motor_group::MotorGroup;

pub struct Arm {
  pub arm_motors: MotorGroup
}

impl Arm {
  pub fn new(arm_motors: MotorGroup) -> Arm {
    Arm {
      arm_motors: arm_motors
    }
  }

  pub fn tare(&mut self) -> Result<(), Error> {
    self.arm_motors.tare_position();
    Ok(())
  }

  pub fn move_absolute(&mut self, position: f64) -> Result<(), Error> {
    self.arm_motors.move_absolute(position, 100);
    Ok(())
  }
}
