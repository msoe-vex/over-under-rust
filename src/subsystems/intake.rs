use vex_rt::prelude::Error;

use crate::devices::motor_group::MotorGroup;

pub struct Intake {
    pub intake: MotorGroup,
    intake_direction: IntakeDirection,
    intake_state: IntakeState,
    prev_toggle_button: bool,
    prev_direction_button: bool
}

enum IntakeDirection {
    FORWARD,
    REVERSE
}

enum IntakeState {
    ON,
    OFF
}

impl Intake {
    pub fn new(motor_group: MotorGroup) -> Intake {
        Intake { 
            intake: motor_group,
            intake_direction: IntakeDirection::FORWARD,
            intake_state: IntakeState::OFF,
            prev_toggle_button: false,
            prev_direction_button: false
        }
    }

    pub fn manual_control(&mut self, toggle: bool, switch_direction: bool) -> Result<(), Error> {
        // Define state transitions
        if toggle && self.prev_toggle_button == false {
            if matches!(self.intake_state, IntakeState::OFF) {
                self.intake_state = IntakeState::ON;
            } else {
                self.intake_state = IntakeState::OFF
            }
        }
        self.prev_toggle_button = toggle;

        if switch_direction && self.prev_direction_button == false {
            if matches!(self.intake_direction, IntakeDirection::FORWARD) {
                self.intake_direction = IntakeDirection::REVERSE;
            } else {
                self.intake_direction = IntakeDirection::FORWARD;
            }
        }
        self.prev_direction_button = switch_direction;

        // Define state actions
        if matches!(self.intake_state, IntakeState::ON) {
            if matches!(self.intake_direction, IntakeDirection::FORWARD) {
                self.intake.move_i8(127)?;
            } else {
                self.intake.move_i8(-127)?;
            }
        } else {
            self.intake.move_i8(0)?;
        }

        Ok(())
    }
}
