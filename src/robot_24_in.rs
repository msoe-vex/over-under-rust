use core::time::Duration;

use alloc::vec;
use vex_rt::{
    prelude::*,
    robot::Robot,
    rtos::{Context, Mutex},
};

use crate::{
    devices::{motor_group::MotorGroup, smart_motor::SmartMotor},
    subsystems::{intake::Intake, arm::Arm},
    subsystems::tank_drive::TankDrive,
};

pub struct Robot24In {
    drive: Mutex<TankDrive>,
    intake: Mutex<Intake>,
    intake_arm: Mutex<Arm>,
    controller: Controller,
}

impl Robot for Robot24In {
    fn new(peripherals: Peripherals) -> Self {
        let left_motor1 = SmartMotor::new(
            peripherals.port03,
            Gearset::EighteenToOne,
            EncoderUnits::Degrees,
            true,
        );
        let left_motor2 = SmartMotor::new(
            peripherals.port04,
            Gearset::EighteenToOne,
            EncoderUnits::Degrees,
            true,
        );
        let left_motor3 = SmartMotor::new(
            peripherals.port18,
            Gearset::EighteenToOne,
            EncoderUnits::Degrees,
            true,
        );
        let left_motor4 = SmartMotor::new(
            peripherals.port06,
            Gearset::EighteenToOne,
            EncoderUnits::Degrees,
            true,
        );
        let right_motor1 = SmartMotor::new(
            peripherals.port08,
            Gearset::EighteenToOne,
            EncoderUnits::Degrees,
            false,
        );
        let right_motor2 = SmartMotor::new(
            peripherals.port09,
            Gearset::EighteenToOne,
            EncoderUnits::Degrees,
            false,
        );
        let right_motor3 = SmartMotor::new(
            peripherals.port10,
            Gearset::EighteenToOne,
            EncoderUnits::Degrees,
            false,
        );
        let right_motor4 = SmartMotor::new(
            peripherals.port11,
            Gearset::EighteenToOne,
            EncoderUnits::Degrees,
            false,
        );
        let intake_motor = SmartMotor::new(
            peripherals.port14,
            Gearset::ThirtySixToOne,
            EncoderUnits::Degrees,
            true,
        );
        let left_intake_arm_motor = SmartMotor::new(
            peripherals.port12,
            Gearset::ThirtySixToOne,
            EncoderUnits::Degrees,
            true
        );
        let right_intake_arm_motor = SmartMotor::new(
            peripherals.port13,
            Gearset::ThirtySixToOne,
            EncoderUnits::Degrees,
            false
        );
        let bottom_cata_motor = SmartMotor::new(
            peripherals.port15,
            Gearset::ThirtySixToOne,
            EncoderUnits::Degrees,
            false
        );
        let top_cata_motor = SmartMotor::new(
            peripherals.port16,
            Gearset::ThirtySixToOne,
            EncoderUnits::Degrees,
            true
        );
        // let cata_pot = AdiAnalog::new(
            
        // )

        Self {
            drive: Mutex::new(TankDrive::new(
                MotorGroup {
                    motors: vec![left_motor1, left_motor2, left_motor3, left_motor4],
                },
                MotorGroup {
                    motors: vec![right_motor1, right_motor2, right_motor3, right_motor4],
                },
            )),
            intake: Mutex::new(Intake::new(
                MotorGroup {
                    motors: vec![intake_motor],
                },
            )),
            intake_arm: Mutex::new(Arm::new(
                MotorGroup{
                    motors: vec![left_intake_arm_motor, right_intake_arm_motor],
                }
            )),

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
                false
            );

            // intake control
            let intake_direction = self.controller.l2.is_pressed().unwrap_or(false);
            let intake_state = self.controller.r2.is_pressed().unwrap_or(false);
            self.intake.lock().manual_control(intake_state, intake_direction);

            self.intake_arm.lock().two_pos_two_button(
                0.0, 
                82.0, 
                self.controller.b.is_pressed().unwrap(), 
                self.controller.x.is_pressed().unwrap()
            );
        
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
