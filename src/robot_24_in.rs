use core::time::Duration;

use alloc::vec;
use vex_rt::{
    prelude::*,
    robot::Robot,
    rtos::{Context, Mutex},
};

use crate::{
    devices::{motor_group::MotorGroup, smart_motor::SmartMotor},
    subsystems::intake::Intake,
    subsystems::tank_drive::TankDrive,
};

pub struct Robot24In {
    drive: Mutex<TankDrive>,
    intake: Mutex<Intake>,
    controller: Controller,
}

impl Robot for Robot24In {
    fn new(peripherals: Peripherals) -> Self {
        let left_motor1 = SmartMotor::new(
            peripherals.port01,
            Gearset::EighteenToOne,
            EncoderUnits::Degrees,
            false,
        );
        let left_motor2 = SmartMotor::new(
            peripherals.port02,
            Gearset::EighteenToOne,
            EncoderUnits::Degrees,
            false,
        );
        let left_motor3 = SmartMotor::new(
            peripherals.port03,
            Gearset::EighteenToOne,
            EncoderUnits::Degrees,
            false,
        );
        let left_motor4 = SmartMotor::new(
            peripherals.port04,
            Gearset::EighteenToOne,
            EncoderUnits::Degrees,
            false,
        );
        let right_motor1 = SmartMotor::new(
            peripherals.port05,
            Gearset::EighteenToOne,
            EncoderUnits::Degrees,
            true,
        );
        let right_motor2 = SmartMotor::new(
            peripherals.port06,
            Gearset::EighteenToOne,
            EncoderUnits::Degrees,
            true,
        );
        let right_motor3 = SmartMotor::new(
            peripherals.port07,
            Gearset::EighteenToOne,
            EncoderUnits::Degrees,
            true,
        );
        let right_motor4 = SmartMotor::new(
            peripherals.port08,
            Gearset::EighteenToOne,
            EncoderUnits::Degrees,
            true,
        );
        let intake_motor = SmartMotor::new(
            peripherals.port09,
            Gearset::EighteenToOne,
            EncoderUnits::Degrees,
            true,
        );

        Self {
            drive: Mutex::new(TankDrive {
                left_side: MotorGroup {
                    motors: vec![left_motor1, left_motor2, left_motor3, left_motor4],
                },
                right_side: MotorGroup {
                    motors: vec![right_motor1, right_motor2, right_motor3, right_motor4],
                },
            }),
            intake: Mutex::new(Intake {
                intake: MotorGroup {
                    motors: vec![intake_motor],
                },
            }),

            controller: peripherals.master_controller,
        }
    }

    fn initialize(self: &mut Robot24In, _ctx: Context) {
        // Do any extra initialization here.
    }

    fn autonomous(self: &mut Robot24In, _ctx: Context) {
        println!("autonomous");
        // Write your autonomous routine here.
    }

    fn opcontrol(self: &mut Robot24In, ctx: Context) {
        println!("opcontrol");

        // This loop construct makes sure the drive is updated every 10
        // milliseconds.
        let mut l = Loop::new(Duration::from_millis(10));
        loop {
            // Update the motors.
            self.drive.lock().manual_control(
                self.controller.left_stick.get_y().unwrap(),
                self.controller.right_stick.get_y().unwrap(),
            );

            // intake control
            let is_on = self.controller.l2.is_pressed().unwrap_or(false);
            self.intake.lock().manual_control(is_on);

            select! {
                // If the driver control period is done, break out of the loop.
                _ = ctx.done() => break,

                // Otherwise, when it's time for the next loop cycle, continue.
                _ = l.select() => continue,
            }
        }
    }

    fn disabled(self: &mut Robot24In, _ctx: Context) {
        println!("disabled");
        // This runs when the robot is in disabled mode.
    }
}
