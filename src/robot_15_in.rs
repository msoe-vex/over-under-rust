use core::time::Duration;

use alloc::vec;
use vex_rt::{
    prelude::*,
    robot::Robot,
    rtos::{Context, Mutex},
};

use crate::{
    devices::{motor_group::MotorGroup, pneumatic::Pneumatic, smart_motor::SmartMotor},
    subsystems::{flap::Flap, intake::Intake, tank_drive::TankDrive},
};

pub struct Robot15In {
    drive: Mutex<TankDrive>,
    right_flap: Mutex<Flap>,
    left_flap: Mutex<Flap>,
    intake: Mutex<Intake>,
    controller: Controller,
}

impl Robot for Robot15In {
    fn new(peripherals: Peripherals) -> Self {
        let left_motor1 = SmartMotor::new(
            peripherals.port14,
            Gearset::ThirtySixToOne,
            EncoderUnits::Degrees,
            true,
        );
        let left_motor2 = SmartMotor::new(
            peripherals.port11,
            Gearset::ThirtySixToOne,
            EncoderUnits::Degrees,
            false,
        );
        let left_motor3 = SmartMotor::new(
            peripherals.port03,
            Gearset::ThirtySixToOne,
            EncoderUnits::Degrees,
            false,
        );
        let left_motor4 = SmartMotor::new(
            peripherals.port04,
            Gearset::ThirtySixToOne,
            EncoderUnits::Degrees,
            true,
        );
        let right_motor1 = SmartMotor::new(
            peripherals.port05,
            Gearset::ThirtySixToOne,
            EncoderUnits::Degrees,
            false,
        );
        let right_motor2 = SmartMotor::new(
            peripherals.port15,
            Gearset::ThirtySixToOne,
            EncoderUnits::Degrees,
            true,
        );
        let right_motor3 = SmartMotor::new(
            peripherals.port08,
            Gearset::ThirtySixToOne,
            EncoderUnits::Degrees,
            true,
        );
        let right_motor4 = SmartMotor::new(
            peripherals.port09,
            Gearset::ThirtySixToOne,
            EncoderUnits::Degrees,
            false,
        );
        let intake_motor: SmartMotor = SmartMotor::new(
            peripherals.port13,
            Gearset::EighteenToOne,
            EncoderUnits::Degrees,
            false,
        );

        let left_flap: Flap = Flap::new(Pneumatic::new(peripherals.port_h));

        let right_flap: Flap = Flap::new(Pneumatic::new(peripherals.port_g));

        Self {
            drive: Mutex::new(TankDrive {
                left_side: MotorGroup {
                    motors: vec![left_motor1, left_motor2, left_motor3, left_motor4],
                },
                right_side: MotorGroup {
                    motors: vec![right_motor1, right_motor2, right_motor3, right_motor4],
                },
            }),
            left_flap: Mutex::new(left_flap),
            right_flap: Mutex::new(right_flap),
            intake: Mutex::new(Intake {
                intake: MotorGroup {
                    motors: vec![intake_motor],
                },
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
            self.drive
                .lock()
                .manual_control(
                    self.controller.left_stick.get_y().unwrap(),
                    self.controller.right_stick.get_y().unwrap(),
                )
                .unwrap_or_else(|err| {
                    println!("{err}");
                });

            let intake_state = self.controller.l2.is_pressed().unwrap_or(false);
            self.intake
                .lock()
                .manual_control(intake_state)
                .unwrap_or_else(|err| {
                    println!("{err}");
                });

            let left_flap_state = self.controller.l1.is_pressed().unwrap_or(false);
            self.left_flap
                .lock()
                .manual_control(left_flap_state)
                .unwrap_or_else(|err| {
                    println!("{err}");
                });

            let right_flap_state = self.controller.r1.is_pressed().unwrap_or(false);
            self.right_flap
                .lock()
                .manual_control(right_flap_state)
                .unwrap_or_else(|err| {
                    println!("{err}");
                });

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
