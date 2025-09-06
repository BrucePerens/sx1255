pub struct Frequency {
  /// The frequency in MHz.
  pub frequency: f64,
}

impl Frequency {
  /// Calculate the offset from baseband, in Hz, necessary in the SDR software
  /// to reach that exact frequency. The translation to
  /// [hard_registers::Frequency](crate::hard_registers::Frequency)
  /// will always work so that the IC is set to lower-than or equal-to the
  /// requested frequency, and thus this value will be a positive value less
  /// than the resolution of the IC, or zero. The resolution of the IC will be
  /// around 34 Hz, depending on the oscillator crystal.
  pub const fn offset(&self) -> f64 {
    0.0
  }
}

#[doc = include_str!("../markdown/registers.md")]
pub struct Registers {
}

impl Registers {
  pub fn write() { }
}
