use crate::hard_registers::*;
use std::*;

#[allow(dead_code)]
pub struct Receive {
    frequency: f64,
}

#[allow(dead_code)]
pub struct Transmit {
    /// Frequency in Hz.
    frequency: f64,

    /// DAC gain in dB.
    dac_gain: f32,

    /// Mixer gain in dB.
    mixer_gain: f32,

    /// Tank capacitor in femtofarads. Only documented for SX1255.
    mixer_tank_cap: u16,

    /// Tank parallel resistor in KΩ. Only documented for SX1255.
    mixer_tank_res: f32,

    /// Transmit PLL loop filter bandwidth in KHz.
    pll_bandwidth: f32,

    /// Transmit analog filter DSB bandwidth in MHz.
    filter_bandwidth: f32,

    /// Number of taps of the transmit FIR-DAC.
    dac_bandwidth: u8,
}

#[doc = include_str!("../markdown/registers.md")]
#[allow(dead_code)]
pub struct Registers {
    /// The version of the IC.
    pub ic_version: ICVersion,

    /// The frequency of the clock crystal. This should be between 32 and 36.864
    /// MHz. For frequency accuracy, this should be measured per device, rather
    /// than simply taken from a specification.
    pub crystal_frequency: f64,
    pub mode: Mode,
    pub transmit: Transmit,
    pub receive: Receive,
}

#[allow(dead_code)]
impl Registers {
  /// Calculate the offset from baseband, in Hz, necessary in the SDR software
  /// to reach that exact frequency. The translation to
  /// [hard_registers::Frequency](crate::hard_registers::Frequency)
  /// will always work so that the IC is set to lower-than or equal-to the
  /// requested frequency, and thus this value will be a positive value less
  /// than the resolution of the IC, or zero. The resolution of the IC will be
  /// around 34 Hz, depending on the oscillator crystal.
  pub const fn offset(_frequency: f64) -> f64 {
    0.0
  }

  pub fn write() { }
}
