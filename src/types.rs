//! Types for the SCD4x sensor.

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SensorData {
    pub co2: u16,
    pub temperature: f32,
    pub humidity: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RawSensorData {
    pub co2: u16,
    pub temperature: u16,
    pub humidity: u16,
}

/// The SCD4x sensor variant, as reported by the `get_sensor_variant` command
/// (datasheet section 3.10.6).
///
/// The variant is determined from the top 4 bits (bits 15..12) of the response,
/// as the lower 12 bits are not significant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SensorVariant {
    /// SCD40 sensor variant (response bits 15..12 = `0000`).
    SCD40,
    /// SCD41 sensor variant (response bits 15..12 = `0001`).
    SCD41,
    /// SCD42 sensor variant (response bits 15..12 = `0010`).
    SCD42,
    /// SCD43 sensor variant (response bits 15..12 = `0101`).
    SCD43,
    /// An unrecognized sensor variant.
    Unknown,
}
