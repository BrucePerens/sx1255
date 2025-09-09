use crate::hard_registers::ICVersion;
use std::*;

#[allow(dead_code)]
pub struct Receive {
    /// Frequency in MHz.
    pub frequency: f64,

    /// Indicates that the receiver PLL is ready after setting frequency.
    pub pll_locked: bool,

    /// Imput impedance in ohms.
    pub input_impedance: u8,

    /// Receive LNA gain in dB.
    /// This effects both the receiver noise figure and IP3, receiver performance
    /// will be best with this value at minimum, see the RX Front-End
    /// specification in the data sheet.
    pub lna_gain: f32,

    /// Receive baseband amplifier gain in dB.
    pub baseband_gain: f32,

    /// Receiver input impedance. 0 is 50 ohm, 1 is 200 ohm.
    pub zin: u8,

    /// Receive delta-sigma SSB bandwidth, minimum, in KHz.
    pub adc_bw: u16,

    /// Receive programmable gain amplifier bandwidth in dB.
    pub pga_bw: f32,

    /// Receive PLL loop filter bandwidth. bandwidth = (value + 1) * 75 KHz.
    /// Wider bandwidth reduces lock time while increasing spurs and noise.
    pub pll_bw: u16,

    /// Puts the receive ADC into temperature-measurement mode.
    /// The response of the sensor is -1C/Lsb. Measurement happens
    /// in less than 100μs. CMOS temperature
    /// measurement is inherently inaccurate and this should be
    /// calibrated against an external temperature measurement of
    /// the IC.
    pub adc_temp: bool,
}

#[allow(dead_code)]
pub struct Transmit {
    /// Frequency in MHz.
    pub frequency: f64,


    /// DAC gain in dB.
    pub dac_gain: f32,

    /// Mixer gain in dB.
    pub mixer_gain: f32,

    /// Tank capacitor in femtofarads. Only documented for SX1255.
    pub mixer_tank_cap: u16,

    /// Tank parallel resistor in KΩ. Only documented for SX1255.
    pub mixer_tank_res: f32,

    /// Transmit PLL loop filter bandwidth in KHz.
    pub pll_bandwidth: f32,

    /// Transmit analog filter DSB bandwidth in MHz.
    pub filter_bandwidth: f32,

    /// Number of taps of the transmit FIR-DAC.
    dac_bandwidth: u8,
}

#[derive(Default)]
pub enum LoopBack {
  #[default]
  Off,
  Digital,
  RF,
}

#[derive(Default)]
pub enum Mode {
    #[default]
    Sleep,
    Standby,
    Receive,
    Transmit,
    FullDuplex,
}

#[doc = include_str!("../markdown/control.md")]
#[allow(dead_code)]
pub struct Control {
    /// The frequency of the clock crystal. This should be between 32 and 36.864
    /// MHz. For frequency accuracy, this should be measured per device, rather
    /// than simply taken from a specification.
    pub crystal_frequency: f64,
    pub mode: Mode,
    pub loop_back: LoopBack,
    pub clock_output_enable: bool,
    pub battery_lower_limit: f32,
    pub transmit: Transmit,
    pub receive: Receive,
}

#[allow(dead_code)]
/// SX1255/SX1257 soft status information, decoded from
/// [hard_registers::Status](crate::hard_registers::Status) and/or digital I/O
/// lines.
pub struct Status {
    /// The version of the IC.
    pub ic_version: ICVersion,
    pub battery_low: bool,
    pub oscillator_stable: bool,
    /// Indicates that the receive PLL is ready after setting frequency.
    pub receive_pll_locked: bool,
    /// Indicates that the transmit PLL is ready after setting frequency.
    pub transmit_pll_locked: bool,
}

#[allow(dead_code)]
impl Control {
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
