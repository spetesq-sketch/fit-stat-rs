#![no_std]
#![warn(clippy::unwrap_used)]

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use libm::powf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    Female,
    Male,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DateTime {
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
}

impl DateTime {
    pub fn new(year: i32, month: u8, day: u8, hour: u8, minute: u8) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WeightEntry {
    pub time: Option<DateTime>,
    pub weight: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserData {
    pub gender: Gender,
    pub age: u8,
    pub height: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub data: UserData,
    weight: Vec<WeightEntry>,
}
impl User {
    pub fn new(
        gender: Gender,
        age: u8,
        height: f32,
        weight: f32,
        time_point: Option<DateTime>,
    ) -> Self {
        let data = UserData {
            gender,
            age,
            height,
        };
        let weight_entry = WeightEntry {
            time: time_point,
            weight,
        };
        Self {
            data,
            weight: vec![weight_entry],
        }
    }
    pub fn change_user_data(&mut self, data: UserData) {
        self.data = data;
    }
    pub fn get_last_weight(&self) -> Option<f32> {
        self.weight.last().map(|weight_entry| weight_entry.weight)
    }
    pub fn add_weight(&mut self, weight: f32, time_point: Option<DateTime>) {
        let weight_entry = WeightEntry {
            time: time_point,
            weight,
        };
        self.weight.push(weight_entry);
    }

    pub fn get_health_report(&self) -> Result<HealthReport, HealthError> {
        let weight = self.get_last_weight().ok_or(HealthError::NoWeight)?;
        let height = self.data.height;

        if height <= 0.0 {
            return Err(HealthError::InvalidHeight);
        }

        let actual_bmi = calculate_bmi(weight, height).ok_or(HealthError::NoWeight)?;
        let age = self.data.age;

        let age_offset = match age {
            0..=24 => 19.0,
            25..=34 => 20.0,
            35..=44 => 21.0,
            45..=54 => 22.0,
            55..=64 => 23.0,
            _ => 24.0,
        };

        let target_weight = match self.data.gender {
            Gender::Male => (height - 100.0) - ((height - 150.0) / 4.0),
            Gender::Female => (height - 100.0) - ((height - 150.0) / 2.5),
            Gender::None => {
                let male_t = (height - 100.0) - ((height - 150.0) / 4.0);
                let female_t = (height - 100.0) - ((height - 150.0) / 2.5);
                (male_t + female_t) / 2.0
            }
        };

        let base_norm = 19.0;
        let bmi = actual_bmi - (age_offset - base_norm);
        let extra_weight = weight - target_weight;

        let min_normal = age_offset;
        let max_normal = min_normal + 6.0;

        let status = if actual_bmi < min_normal - 3.0 {
            Status::SevereWeightDeficit
        } else if actual_bmi < min_normal {
            Status::Underweight
        } else if actual_bmi <= max_normal {
            Status::NormalWeight
        } else if actual_bmi <= max_normal + 5.0 {
            Status::Overweight
        } else {
            Status::Obesity
        };

        Ok(HealthReport {
            bmi,
            target_weight,
            age_offset,
            actual_bmi,
            extra_weight,
            status,
        })
    }
}

pub fn calculate_bmi(weight: f32, height: f32) -> Option<f32> {
    if weight <= 0.0 || height <= 0.0 {
        return None;
    }
    let bmi = (1.3 * weight / powf(height, 2.5)) * 1000.0;
    Some(bmi)
}

#[derive(Debug)]
pub struct HealthReport {
    pub bmi: f32,
    pub target_weight: f32,
    pub age_offset: f32,
    pub actual_bmi: f32,
    pub extra_weight: f32,
    pub status: Status,
}

#[derive(Debug)]
pub enum Status {
    SevereWeightDeficit,
    Underweight,
    NormalWeight,
    Overweight,
    Obesity,
}
pub enum HealthError {
    NoWeight,
    InvalidHeight,
}
pub enum UserError {
    WrongWeight,
    WrongHeight,
}
