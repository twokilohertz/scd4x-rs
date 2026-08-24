//! Types for the SCD4x sensor.

#[cfg(feature = "fixed")]
use fixed::types::{I16F16, U16F16};

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SensorData {
    pub co2: u16,
    pub temperature: f32,
    pub humidity: f32,
}

/// Fixed-point variant of [`SensorData`], available with the `fixed` feature.
#[cfg(feature = "fixed")]
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SensorDataFixed {
    pub co2: u16,
    pub temperature: I16F16,
    pub humidity: U16F16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RawSensorData {
    pub co2: u16,
    pub temperature: u16,
    pub humidity: u16,
}

#[cfg(feature = "fixed")]
impl From<RawSensorData> for SensorDataFixed {
    fn from(raw: RawSensorData) -> Self {
        let RawSensorData {
            co2,
            temperature,
            humidity,
        } = raw;
        SensorDataFixed {
            co2,
            temperature: I16F16::from_bits(temperature as i32 * 175) - I16F16::from_num(45),
            humidity: U16F16::from_bits(humidity as u32 * 100),
        }
    }
}
