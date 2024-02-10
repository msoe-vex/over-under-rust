use alloc::vec::Vec;
use vex_rt::{prelude::{Error, Motor}, motor::MotorError};

use super::smart_motor::SmartMotor;

pub struct MotorGroup {
    pub motors: Vec<SmartMotor>,
}

impl MotorGroup {
    pub fn new(motors: Vec<SmartMotor>) -> MotorGroup {
        MotorGroup { motors }
    }

    // Transforms a the local vector of SmartMotors into a vector of Motors so we can use the methods
    pub fn connect(&mut self) -> Result<Vec<&mut Motor>, Error> {
        self.motors
            .iter_mut()
            .map(|motor| motor.connect())
            .collect::<Result<Vec<_>, _>>()
    }

    pub fn move_i8(&mut self, voltage: i8) -> Result<(), Error> {
        for motor in self.connect()? {
            motor.move_i8(voltage)?;
        }
        Ok(())
    }
    pub fn move_velocity(&mut self, velocity: i32) -> Result<(), Error> {
        for motor in self.connect()? {
            motor.move_velocity(velocity)?;
        }
        Ok(())
    }
    pub fn move_voltage(&mut self, voltage: i32) -> Result<(), Error> {
        for motor in self.connect()? {
            motor.move_voltage(voltage)?;
        }
        Ok(())
    }
    pub fn move_absolute(&mut self, position: f64, velocity: Option<i32>) -> Result<(), Error> {
        for motor in self.connect()? {
            motor.move_absolute(position, velocity.unwrap_or(100))?;
        }
        Ok(())
    }
    pub fn move_relative(&mut self, position: f64, velocity: Option<i32>) -> Result<(), Error> {   
        for motor in self.connect()? {
            motor.move_relative(position, velocity.unwrap_or(100))?;
        }
        Ok(())
    }
    pub fn set_zero_position(&mut self, position: f64) -> Result<(), Error> {
        for motor in self.connect()? {
            motor.set_zero_position(position)?;
        }
        Ok(())
    }
    pub fn tare_position(&mut self) -> Result<(), Error> {
        for motor in self.connect()? {
            motor.tare_position()?;
        }
        Ok(())
    }
    pub fn get_target_position(&mut self) -> Result<f64, MotorError> {   
        return self.connect().unwrap().get(0).unwrap().get_target_position();
    }
    pub fn get_position(&mut self) -> Result<f64, MotorError> {   
        return self.connect().unwrap().get(0).unwrap().get_position();
    }
}
