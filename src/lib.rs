#![no_std]
#![warn(clippy::unwrap_used)]
#![allow(non_snake_case)]

extern crate alloc;

use alloc::vec;
use alloc::vec::Vec;
use libm::powf;

#[cfg(feature = "bincode")]
use bincode_next::{Decode, Encode};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "chrono")]
use chrono::{DateTime, Utc};

#[cfg(feature = "chrono")]
/// Converts a `chrono::DateTime<Utc>` into a Unix timestamp in milliseconds.
/// This is useful for storing time in a `no_std` compatible format.
pub fn chrono_time(time: DateTime<Utc>) -> i64 {
    time.timestamp_millis()
}
#[cfg(feature = "chrono")]
/// Attempts to create a `chrono::DateTime<Utc>` from a Unix timestamp in milliseconds.
/// Returns `None` if the timestamp is invalid.
pub fn datetime_from_millis(millis: i64) -> Option<DateTime<Utc>> {
    DateTime::from_timestamp_millis(millis)
}

/// Represents the user's biological sex or gender.
/// This is used to adjust target weight calculations based on physiological differences.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
pub enum Gender {
    Female,
    Male,
    None,
}

//////Custom struct for data ( i wanted to use chrono but no std doesnt support it)
///#[derive(Debug, Clone, Copy, PartialEq, Eq)]
///#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
///#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
///pub struct DateTime {
///    pub year: i32,
///    pub month: u8,
///    pub day: u8,
///    pub hour: u8,
///    pub minute: u8,
///}

/// Measurement system preference for the user's weight and height.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
pub enum Unit {
    #[default]
    Metric,
    Imperial,
}

// impl DateTime {
//     pub fn new(year: i32, month: u8, day: u8, hour: u8, minute: u8) -> Self {
//         Self {
//             year,
//             month,
//             day,
//             hour,
//             minute,
//         }
//     }
// }

/// Represents a single weight measurement at a specific point in time.
/// The `time` is stored as an optional Unix timestamp in milliseconds.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
pub struct WeightEntry {
    pub time: Option<i64>,
    pub weight: f32,
}

/// Core physiological and preference data of the user.
/// This data is generally static but can be updated over time (e.g., age or height).
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
pub struct UserData {
    pub gender: Gender,
    pub age: u8,
    pub height: f32,
    pub unit: Unit,
}

/// The main entity representing a user.
/// It contains the user's static profile data and a chronological history of their weight entries.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
pub struct User {
    pub data: UserData,
    weight: Vec<WeightEntry>,
}
impl User {
    /// Creates a new `User` instance with their initial physiological data and first weight entry.
    ///
    /// # Errors
    /// Returns `UserError::WrongWeight` if the initial weight is less than 10.0.
    /// Returns `UserError::WrongHeight` if the height is less than 30.0.
    pub fn new(
        gender: Gender,
        age: u8,
        height: f32,
        weight: f32,
        unit: Option<Unit>,
        time_point: Option<i64>,
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
    /// Updates the user's core physiological data (e.g., if they grew taller or had a birthday).
    ///
    /// # Errors
    /// Returns `UserError::WrongHeight` if the new height is less than 30.0.
    pub fn change_user_data(&mut self, data: UserData) -> Result<(), UserError> {
        if data.height < 30.0 {
            return Err(UserError::WrongHeight);
        }
        self.data = data;
        Ok(())
    }

    /// Retrieves the user's most recently recorded weight.
    /// Note: This relies on the internal weight history never being empty.
    pub fn get_last_weight(&self) -> f32 {
        unsafe { self.weight.get_unchecked(self.weight.len() - 1).weight }
    }
    /// Adds a new weight measurement to the user's history.
    ///
    /// # Errors
    /// Returns `UserError::WrongWeight` if the provided weight is less than 10.0.
    pub fn add_weight(&mut self, weight: f32, time_point: Option<i64>) -> Result<(), UserError> {
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
    /// Modifies an existing weight entry at the specified index.
    ///
    /// # Errors
    /// Returns `UserError::NotFound` if the index is out of bounds.
    /// Returns `UserError::WrongWeight` if the new weight is less than 10.0.
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

    /// Retrieves a chronological list of recent weight entries.
    ///
    /// If `amount` is provided, it returns up to that many of the most recent entries.
    /// If `amount` is `None`, it returns the entire weight history.
    pub fn weight_history(&self, amount: Option<usize>) -> Vec<WeightEntry> {
        let am = amount.unwrap_or(self.weight.len());
        let start_index = self.weight.len().saturating_sub(am);
        self.weight[start_index..].to_vec()
    }

    /// Generates a comprehensive health report based on the user's current data and latest weight.
    /// The report includes BMI (calculated via the Oxford formula), target weight, and weight status.
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

/// Calculates the Body Mass Index (BMI) using the modern Oxford formula.
/// The Oxford formula (`1.3 * weight / height^2.5`) is considered more accurate than the traditional calculation
pub fn calculate_bmi(weight: f32, height: f32) -> f32 {
    1.3 * weight / powf(height / 100.0, 2.5)
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bincode", derive(Encode, Decode))]
/// A comprehensive summary of the user's health metrics based on their latest data.
pub struct HealthReport {
    /// The user's Body Mass Index calculated using the Oxford formula.
    pub bmi: f32,
    /// The estimated ideal weight for the user based on their height and gender.
    pub target_weight: f32,
    /// The baseline BMI considered "normal" for the user's specific age group.
    pub age_offset: f32,
    /// The user's BMI adjusted relative to their age group's baseline.
    pub actual_bmi: f32,
    /// The difference between the user's current weight and their target weight.
    pub extra_weight: f32,
    /// Categorization of the user's current weight (e.g., Normal, Overweight).
    pub status: Status,
}

/// Categorization of the user's weight status based on their adjusted BMI.
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
/// Possible errors that can occur during user data creation or manipulation.
#[derive(Debug)]
pub enum UserError {
    WrongWeight,
    WrongHeight,
    NotFound,
}
