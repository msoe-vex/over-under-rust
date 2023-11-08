use crate::devices::motor_group::MotorGroup;

pub struct Intake {
    intake: MotorGroup
}

impl Intake {
    pub fn manual_control(mut self, isOn:bool) -> Result<(), Error> {
        if (isOn) {
            self.intake.connect()?.move_i8(127);
        }
    }
}
