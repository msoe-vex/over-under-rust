use vex_rt::prelude::Error;

use crate::devices::motor_group::MotorGroup;

pub struct TankDrive {
    pub left_side: MotorGroup,
    pub right_side: MotorGroup,
    direction: Direction,
    prev_dir_button: bool
}

enum Direction {
    FORWARD,
    REVERSE
}

impl TankDrive {
    pub fn new(left_side: MotorGroup, right_side: MotorGroup) -> TankDrive {
        TankDrive { 
            left_side: left_side, 
            right_side: right_side, 
            direction: Direction::FORWARD, 
            prev_dir_button: false 
        }
    }

    pub fn manual_control(&mut self, left: i8, right: i8, direction_toggle: bool) -> Result<(), Error> {
        // Define State Transitions
        if direction_toggle && self.prev_dir_button == false {
            if matches!(self.direction, Direction::FORWARD) {
                self.direction = Direction::REVERSE;
            } else {
                self.direction = Direction::FORWARD;
            }
        }
        self.prev_dir_button = direction_toggle;
        
        // Define State Actions
        if matches!(self.direction, Direction::FORWARD) {
            self.left_side.move_i8(left)?;
            self.right_side.move_i8(right)?;
        } else {
            self.left_side.move_i8(-right)?;
            self.right_side.move_i8(-left)?;
        }
        Ok(())
    }

    pub fn arcade_control(&mut self, power: i8, turn: i8, direction_toggle: bool) -> Result<(), Error> {
        // Define State Transitions
        if direction_toggle && self.prev_dir_button == false {
            if matches!(self.direction, Direction::FORWARD) {
                self.direction = Direction::REVERSE;
            } else {
                self.direction = Direction::FORWARD;
            }
        }
        self.prev_dir_button = direction_toggle;
        
        // Define State Actions
        if matches!(self.direction, Direction::FORWARD) {
            self.left_side.move_i8(power + turn)?;
            self.right_side.move_i8(power - turn)?;
        } else {
            self.left_side.move_i8(-(power - turn))?;
            self.right_side.move_i8(-(power + turn))?;
        }
        Ok(())
    }
}
