//! # Sample Conversion
//!
//! Module containing various conversions between standard audio integer types
//! and sample values.
//!
//! Conversions of values to/from `u8` follow the definition that the value 128
//! is the 0-point. Essentially, samples values on the range [-1,1] are mapped
//! to a `u8` value in the range [0,255].

use super::*;

/// Converts a u8 8-bit sample to a `SampleT`.
pub fn sample_from_u8(v: u8) -> SampleT {
    (v as SampleT - 128.0) / 128.0
}
/// Converts a raw byte to a `SampleT`.
pub fn sample_from_u8_bytes(v: [u8; 1]) -> SampleT {
    (v[0] as SampleT - 128.0) / 128.0
}

/// Converts a `SampleT` to an `u8`.
pub fn sample_to_u8(s: SampleT) -> u8 {
    (s * 128.0 + 128.0).round() as u8
}
/// Converts a `SampleT` to a raw little-endian byte.
pub fn sample_to_u8_bytes(s: SampleT) -> [u8; 1] {
    [sample_to_u8(s)]
}

/// Converts an i16 16-bit sample to a `SampleT`.
pub fn sample_from_i16(v: i16) -> SampleT {
    v as SampleT / ((1 << 15) as SampleT - 1.0)
}
/// Converts raw bytes to a `SampleT`.
pub fn sample_from_i16_bytes(v: [u8; 2]) -> SampleT {
    (i16::from_le_bytes(v) as SampleT) / ((1 << 15) as SampleT - 1.0)
}

/// Converts a `SampleT` to an `i16`.
pub fn sample_to_i16(s: SampleT) -> i16 {
    (s * ((1 << 15) as SampleT - 1.0)).round() as i16
}
/// Converts a `SampleT` to raw little-endian bytes.
pub fn sample_to_i16_bytes(s: SampleT) -> [u8; 2] {
    sample_to_i16(s).to_le_bytes()
}

/// Converts an i32 24-bit sample to a `SampleT`.
pub fn sample_from_i24(v: i32) -> SampleT {
    v as SampleT / ((1 << 23) as SampleT - 1.0)
}
/// Converts raw bytes to a `SampleT`.
pub fn sample_from_i24_bytes(v: [u8; 3]) -> SampleT {
    (i32::from_le_bytes([v[0], v[1], v[2], 0]) as SampleT) / ((1 << 23) as SampleT - 1.0)
}

/// Converts a `SampleT` to an `i24`.
pub fn sample_to_i24(s: SampleT) -> i32 {
    (s * ((1 << 23) as SampleT - 1.0)).round() as i32
}
/// Converts a `SampleT` to raw little-endian bytes.
pub fn sample_to_i24_bytes(s: SampleT) -> [u8; 3] {
    let i = sample_to_i24(s).to_le_bytes();

    [i[0], i[1], i[2]]
}
