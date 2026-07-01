#![no_std]
#![warn(clippy::unwrap_used)]

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use libm::powf;

#[cfg(feature = "bincode")]
use bincode_next::{Decode, Encode};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Gerder for user data
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
pub enum Gender {
    Female,
    Male,
    None,
}

///Custom struct for data ( i wanted to use chrono but no std doesnt support it)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
pub struct DateTime {
    pub year: i32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
pub enum Unit {
    #[default]
    Metric,
    Imperial,
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

/// WeightEntry consider weight and time for graphs (im not sure that DataTime is the best way)
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
pub struct WeightEntry {
    pub time: Option<DateTime>,
    pub weight: f32,
}

/// UserData for gender, age, height
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
pub struct UserData {
    pub gender: Gender,
    pub age: u8,
    pub height: f32,
    pub unit: Unit,
}

/// User consider UserData and Vec of WeightEntry
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
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
        unit: Option<Unit>,
        time_point: Option<DateTime>,
    ) -> Result<Self, UserError> {
        if weight < 10.0 {
            return Err(UserError::WrongWeight);
        } else if height < 30.0 {
            return Err(UserError::WrongHeight);
        }
        let data = UserData {
            gender,
            age,
            height,
            unit: unit.unwrap_or_default(),
        };
        let weight_entry = WeightEntry {
            time: time_point,
            weight,
        };
        Ok(Self {
            data,
            weight: vec![weight_entry],
        })
    }
    /// Fn to change user data like height, age etc
    pub fn change_user_data(&mut self, data: UserData) -> Result<(), UserError> {
        if data.height < 30.0 {
            return Err(UserError::WrongHeight);
        }
        self.data = data;
        Ok(())
    }
    /// Fn to get last height I advise you to use the function of this structure, because otherwise there may be an error
    pub fn get_last_weight(&self) -> f32 {
        unsafe { self.weight.get_unchecked(self.weight.len() - 1).weight }
    }
    pub fn add_weight(
        &mut self,
        weight: f32,
        time_point: Option<DateTime>,
    ) -> Result<(), UserError> {
        if weight < 10.0 {
            return Err(UserError::WrongWeight);
        }
        let weight_entry = WeightEntry {
            time: time_point,
            weight,
        };
        self.weight.push(weight_entry);
        Ok(())
    }
    pub fn change_weight(&mut self, id: usize, weight_entry: WeightEntry) -> Result<(), UserError> {
        if let Some(entry) = self.weight.get_mut(id) {
            if weight_entry.weight < 10.0 {
                return Err(UserError::WrongWeight);
            }
            *entry = weight_entry;
            return Ok(());
        }
        Err(UserError::NotFound)
    }
    /// Fn to get history of weight
    pub fn weight_history(&self, amount: Option<usize>) -> Vec<WeightEntry> {
        let am = amount.unwrap_or(self.weight.len());
        let start_index = self.weight.len().saturating_sub(am);
        self.weight[start_index..].to_vec()
    }

    pub fn get_health_report(&self) -> HealthReport {
        let (weight, height) = match self.data.unit {
            Unit::Metric => (self.get_last_weight(), self.data.height),
            Unit::Imperial => (self.get_last_weight() * 0.45359237, self.data.height * 2.54),
        };

        let bmi = calculate_bmi(weight, height);
        let age = self.data.age;

        let age_offset = match age {
            0..=24 => 19.0,
            25..=34 => 20.0,
            35..=44 => 21.0,
            45..=54 => 22.0,
            55..=64 => 23.0,
            _ => 24.0,
        };

        let target_weight_kg = match self.data.gender {
            Gender::Male => (height - 100.0) - ((height - 150.0) / 4.0),
            Gender::Female => (height - 100.0) - ((height - 150.0) / 2.5),
            Gender::None => {
                let male_t = (height - 100.0) - ((height - 150.0) / 4.0);
                let female_t = (height - 100.0) - ((height - 150.0) / 2.5);
                (male_t + female_t) / 2.0
            }
        };

        let target_weight = match self.data.unit {
            Unit::Metric => target_weight_kg,
            Unit::Imperial => target_weight_kg / 0.45359237,
        };

        let base_norm = 19.0;
        let actual_bmi = bmi - (age_offset - base_norm);
        let extra_weight = match self.data.unit {
            Unit::Metric => weight - target_weight_kg,
            Unit::Imperial => {
                let extra_kg = weight - target_weight_kg;
                extra_kg / 0.45359237
            }
        };

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

        HealthReport {
            bmi,
            target_weight,
            age_offset,
            actual_bmi,
            extra_weight,
            status,
        }
    }
}

pub fn calculate_bmi(weight: f32, height: f32) -> f32 {
    1.3 * weight / powf(height / 100.0, 2.5)
}

/// A small report for health
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
pub struct HealthReport {
    /// Just bmi according to the Oxford formula
    pub bmi: f32,
    /// The best height for user
    pub target_weight: f32,
    /// BMI varies slightly with age
    pub age_offset: f32,
    /// BMI with age offset
    pub actual_bmi: f32,
    /// Amount of excess weight (kg)
    pub extra_weight: f32,
    /// Weight status: underweight, normal weight, etc.
    pub status: Status,
}

/// Weight status: underweight, normal weight, etc.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
pub enum Status {
    SevereWeightDeficit,
    Underweight,
    NormalWeight,
    Overweight,
    Obesity,
}
#[derive(Debug)]
pub enum UserError {
    WrongWeight,
    WrongHeight,
    NotFound,
}
