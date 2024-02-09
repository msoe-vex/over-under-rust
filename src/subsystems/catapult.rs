use vex_rt::prelude::{Error, AdiAnalog};

// use crate::num_traits::float::FloatCore;
use crate::devices::motor_group::MotorGroup;

pub struct Catapult {
    pub cata_motors: MotorGroup,
    pub cata_pot: AdiAnalog,
    cata_state: CataState,
    prev_launch_button: bool,
}

enum CataState {
    HOLDING,
    LAUNCHING,
    RETRACTING,
}

impl Catapult {
    pub fn new(motors: MotorGroup, pot: AdiAnalog) -> Catapult {
        Catapult {
            cata_motors: motors,
            cata_pot: pot,
            cata_state: CataState::RETRACTING,
            prev_launch_button: false,
        }
    }

    pub fn launch(&mut self, launch_button: bool) -> Result<(), Error> {
        // Define State Transitions
        if matches!(self.cata_state, CataState::HOLDING) {
            if launch_button && self.prev_launch_button == false {
                self.cata_state = CataState::LAUNCHING;
            }
        } else if matches!(self.cata_state, CataState::LAUNCHING) {
            self.cata_state = CataState::RETRACTING
        } else {
            // RETRACTING
            let target_pos = self.cata_motors.get_target_position();
            let current_pos = self.cata_motors.get_position();
            let error = target_pos? - current_pos?;

            if -10.0 <= error && error <= 10.0 {
                self.cata_state = CataState::HOLDING;
            }
        }

        // Define State Actions
        if matches!(self.cata_state, CataState::HOLDING) {
            if self.cata_pot.read()? <= 180 {
                self.cata_motors.move_voltage(12000);
            } else {
                self.cata_motors.move_voltage(0);
            }
        } else if matches!(self.cata_state, CataState::LAUNCHING) {
            self.cata_motors.move_relative(90.0, None);
        } else {
            // RETRACTING
            () // Do nothing as the motor should already be attempting to move
               // the the target position
        }

        Ok(())
    }
}
