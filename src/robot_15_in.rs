use core::time::Duration;

use vex_rt::{
    prelude::*,
    robot::Robot,
    rtos::{Context, Mutex},
};

use crate::{
    devices::smart_motor::SmartMotor,
    drive::{self, Drive},
};

pub struct Robot15In {
    drive: Mutex<Drive>,
    controller: Controller,
}

impl Robot for Robot15In {
    fn new(peripherals: Peripherals) -> Self {
        Self {
            drive: Mutex::new(drive::Drive {
                left_drive: SmartMotor::new(
                    peripherals.port12,
                    Gearset::EighteenToOne,
                    EncoderUnits::Degrees,
                    false,
                ),
                right_drive: SmartMotor::new(
                    peripherals.port13,
                    Gearset::EighteenToOne,
                    EncoderUnits::Degrees,
                    true,
                ),
            }),
            controller: peripherals.master_controller,
        }
    }

    fn initialize(self: &mut Robot15In, _ctx: Context) {
        // Do any extra initialization here.
    }

    fn autonomous(self: &mut Robot15In, _ctx: Context) {
        println!("autonomous");
        // Write your autonomous routine here.
    }

    fn opcontrol(self: &mut Robot15In, ctx: Context) {
        println!("opcontrol");

        // This loop construct makes sure the drive is updated every 10
        // milliseconds.
        let mut l = Loop::new(Duration::from_millis(10));
        loop {
            // Update the motors.
            self.drive.lock().run(
                self.controller.left_stick.get_y().unwrap(),
                self.controller.right_stick.get_y().unwrap(),
            );

            select! {
                // If the driver control period is done, break out of the loop.
                _ = ctx.done() => break,

                // Otherwise, when it's time for the next loop cycle, continue.
                _ = l.select() => continue,
            }
        }
    }

    fn disabled(self: &mut Robot15In, _ctx: Context) {
        println!("disabled");
        // This runs when the robot is in disabled mode.
    }
}
