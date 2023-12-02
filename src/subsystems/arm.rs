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
    self.arm_motors.move_absolute(position, None);
    Ok(())
  }

  pub fn two_pos_two_button(&mut self, pos1: f64, pos2: f64, but1: bool, but2: bool) {
    if but1 {
      self.move_absolute(pos1);
    } else if but2 {
      self.move_absolute(pos2);
    }
  }
}
