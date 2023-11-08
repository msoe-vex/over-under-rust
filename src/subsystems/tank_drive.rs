use crate::devices::motor_group::MotorGroup;

pub struct TankDrive {
    left_side: MotorGroup,
    right_side: MotorGroup,
}

impl TankDrive {
    pub fn manual_control(mut self, left: i8, right: i8) {
        self.left_side.move_i8(left);
        self.right_side.move_i8(right);
    }
}
