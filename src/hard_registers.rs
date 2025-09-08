// Copyright (C) 2025 Bruce Perens
// All Rights Reserved
// This software is not presently under an Open Source license, I'll consider
// what to do about that if someone pays me to do so, or when I'm done.

use binary_serde::*;

#[derive(Debug, Default, PartialEq, Eq)]
#[binary_serde_bitfield(order = BitfieldBitOrder::MsbFirst)]
/// # Operating modes of the IC.
///
/// When none of these enables are true, the IC is in SLEEP mode.
///
/// In sleep mode, the IC draws a maximum of 1µA and typically 0.2µA. So, no
/// power switch should be necessary for it.
///
/// In STANDBY mode, typically 1.15mA and maximum 1.5mA are drawn.
///
/// In RECEIVE mode, typically 18mA and maximum 25mA are drawn.
///
/// In TRANSMIT mode, typically 60mA and maximum 90mA are drawn.
///
/// If the transmit VCO is already stable, transmit wake-up should take
/// about 120µs
///
pub struct Mode {
    #[bits(4)]
    #[doc(hidden)]
    pub _unused: (),

    #[bits(1)]
    /// Power amplifier enable.
    pub driver_enable: bool,

    #[bits(1)]
    /// Transmit enable _except_ power amplifier.
    pub tx_enable: bool,

    #[bits(1)]
    /// Receiver enable.
    pub rx_enable: bool,

    #[bits(1)]
    /// Enable power supplies and oscillator in standby mode.
    ///
    pub standby_enable: bool,
}

#[derive(Debug, Default, PartialEq, Eq)]
#[binary_serde_bitfield(order = BitfieldBitOrder::MsbFirst)]
/// Integer frequency value.
/// To calculate the frequency, first find the step resolution.
/// for SX1255 step_frequency = oscillator_frequency / 2^20. 
/// for SX1257 step_frequency = the oscillator_frequency / 2^19. 
/// The frequency will be step_frequency * value.
/// The oscillator frequency may be 32 MHz to 36.864 MHz. It is useful,
/// for frequency accuracy, to measure and store the actual oscillator
/// frequency, rather than count on the oscillator to have the exact
/// frequency specified.
///
/// The oscillator cold-start time is 300µs, the frequency synthesizer
/// wake-up is 50-150µs, and hop time is 20µs for steps as large as 400 KHz,
/// 30µs for 1.2 MHz, 50µs for 25 MHz. The PLL ready indication can be mapped
/// to the digital-IO pins and CPU GPIO inputs can be set up generate an
// interrupt when the synthesizer is ready. VCOs operate at twice the RF
/// frequency for SX1257, and four times for SX1255, centered at 1.9 GHz.
///
/// 0xC0E38E is the default value of the hardware register, and should 
/// result in 434 MHz on SX1255 with a 36 MHz crystal, 868 on SX1257.
/// The step resolution will be 34.3323 Hz on SX1255 if the oscillator is 36 MHz,
/// 38.6646 on SX1257.
/// This value is read only when the least significant byte is written to
/// the IC, OR when the IC enters STANDBY mode from SLEEP mode by a
/// transition of
/// [Mode::ref_enable](self::Mode::standby_enable)
/// from 0 to 1.
/// Writing the IC hardware frequency value can be used for frequency hopping,
/// scanning, etc.

pub struct Frequency {
  #[bits(24)]
  frequency: u32,
}

#[derive(Debug, Default, PartialEq, Eq)]
#[binary_serde_bitfield(order = BitfieldBitOrder::MsbFirst)]
/// Version data.
pub struct Version {
    #[bits(4)]
    /// 1 for SX1255, this register is not documented for SX1257.
    fill_revision_number: u8,

    #[bits(4)]
    /// 0xA for SX1255, this register is not documented for SX1257.
    metal_mask_revision_number: u8,
}

#[repr(u8)]
#[derive(Debug, BinarySerde, Default, PartialEq, Eq)]
/// Settings for [TxFrontend::mixer_tank_resistance]
// Toto, I have a feeling we're not in ASCII any longer. 😉
// Rust is programmed in unicode, so we might as well use it.
// The version of tx_mixer_tank_resistance in
// [crate::registers::TxFrontend] will directly take
// integer resistance values and convert them to these values, nobody
// else will have to find Ω on the keyboard.
pub enum TxMixerTankResistance {
    #[default]
    Ω950 = 0,
    Ω1110 = 1,
    Ω1320 = 2,
    Ω1650 = 3,
    Ω2180 = 4,
    Ω3240 = 5,
    Ω6000 = 6,
    Ω64000 = 7 // Resistance "off", approximate value.
}

#[derive(Debug, Default, PartialEq, Eq)]
#[binary_serde_bitfield(order = BitfieldBitOrder::MsbFirst)]
/// SX1255 hardware transmit front-end control register.
pub struct TxFrontend {
    #[bits(1)]
    #[doc(hidden)]
    pub _unused1: (),

    #[bits(3)]
    /// Transmit DAC gain. 3 dB steps ranging from -9 dB for 0, to
    /// 0 dB for 3. Setting the high bit imposes a test Vref voltage (where?)
    pub dac_gain: u8,

    #[bits(4)]
    /// Transmit mixer gain. 37.5 + (2 * value) dB. 2 dB steps.
    pub mixer_gain: u8,

    #[bits(2)]
    #[doc(hidden)]
    pub _unused2: (),

    #[bits(3)]
    /// Transmit mixer tank capacitor. 128 * value femtofarads.
    pub mixer_tank_cap: u8,

    #[bits(3)]
    /// Resistance in paralle with the transmit mixer tank.
    pub mixer_tank_resistance: TxMixerTankResistance,

    #[bits(1)]
    #[doc(hidden)]
    pub _unused3: (),

    #[bits(2)]
	/// Transmit PLL loop filter bandwidth, (value + 1) * 75 KHz.
    pub pll_bw: u8,

    #[bits(5)]
    /// The transmit I/Q filters remove quantization noise created by the
    /// transmit I/Q FIR DACs.
    /// Transmit analog filter 3 db DSB bandwidth in MHz = 
    /// 17.15 * (41 - value). This value has 30% accuracy.
    /// The filter bandwidth should be set for wider than the transmit
    /// bandwidth to reduce group-delay issues.
    pub filter_bw: u8,

    #[bits(5)]
    #[doc(hidden)]
    pub _unused4: (),

    #[bits(3)]
    /// Number of taps of the Transmit I/Q filters.
    /// number of taps = 24 + (8 * value), maximum is 64.
    /// 1 would be an SSB filter 3 dB bandwidth of 450 KHz,
    /// 5 would be an SSB filter 3 dB bandwidth of 290 KHz.
    /// Reducing bandwidth is useful for reducing quantization noise.
    /// The filter bandwidth should be set for wider than the transmit
    /// bandwidth to reduce group-delay issues.
    pub dac_bw: u8
}

#[repr(u8)]
#[derive(Debug, BinarySerde, Default, PartialEq, Eq)]
/// Settings for [RxFrontend::rx_zin]
pub enum RxZIn {
    #[default]
    I50Ω = 0,
    I200Ω = 1,
}

#[repr(u8)]
#[derive(Debug, BinarySerde, Default, PartialEq, Eq)]
/// Settings for [RxFrontend::rx_adc_bw]
/// The data sheet has a cryptic comment: "use 0x01 instead".
pub enum RxADCBw {
    #[default]
    BWOver400KHz = 7,
    BW200To400KHz = 5,
    BW100To400KHz = 2,
}

#[repr(u8)]
#[derive(Debug, BinarySerde, Default, PartialEq, Eq)]
/// Settings for [RxFrontend::rx_pga_bw]
pub enum RxPGABw {
    #[default]
    BW1500KHz = 0,
    BW1000KHz = 1,
    BW750KHz = 2,
    BW500KHz = 3,
}

#[derive(Debug, Default, PartialEq, Eq)]
#[binary_serde_bitfield(order = BitfieldBitOrder::MsbFirst)]
/// SX1255 hardware receive front-end control register.
pub struct RxFrontend {
    #[bits(3)]
    /// Receive LNA gain. Values 0 and 7 are not used. Value 1 is 0 dB, and
    /// gain descends in -6 dB steps until value 6 is -48 dB
    /// This effects both the receiver noise figure and IP3, receiver performance
    /// will be best with this value at minimum, see the RX Front-End
    /// specification in the data sheet.
    pub lna_gain: u8,

    #[bits(4)]
    /// Receive programmable gain amplifier gain.
    /// gain = lowest gain + (2 dB * value)
    pub rx_pga_gain: u8,

    #[bits(1)]
    /// Receiver input impedance. 0 is 50 ohm, 1 is 200 ohm.
    pub rx_zin: RxZIn,

    #[bits(3)]
    /// Receive delta-sigma SSB bandwidth.
    /// The data sheet has a cryptic comment on one line: "use 0x01 instead".
    pub rx_adc_bw: RxADCBw,

    #[bits(3)]
    /// Receive ADC trim for 36 MHz crystal. Defaults to 5.
    pub rx_adc_trim: u8,

    #[bits(2)]
    /// Receive programmable gain amplifier bandwidth.
    pub rx_pga_bw: RxPGABw,

    #[bits(5)]
    #[doc(hidden)]
    pub _unused: (),

    #[bits(2)]
    /// Receive PLL loop filter bandwidth. bandwidth = (value + 1) * 75 KHz.
    /// Wider bandwidth reduces lock time while increasing spurs and noise.
    pub rx_pll_bw: u8,

    #[bits(1)]
    /// Puts the receive ADC into temperature-measurement mode.
    /// The response of the sensor is -1C/Lsb. Measurement happens
    /// in less than 100μs. CMOS temperature
    /// measurement is inherently inaccurate and this should be
    /// calibrated against an external temperature measurement of
    /// the IC.
    pub rx_adc_temp: bool,
}

#[repr(u8)]
#[derive(BinarySerde, Debug, Default, Eq, PartialEq)]
pub enum IOMap0 {
  #[default]
  PLLLockRx = 0,
  PLLLockRx1 = 1,
  PLLLockRx2 = 2,
  Eol = 3
}

#[repr(u8)]
#[derive(BinarySerde, Debug, Default, Eq, PartialEq)]
pub enum IOMap1 {
  #[default]
  PLLLockTx = 0,
}

#[repr(u8)]
#[derive(BinarySerde, Debug, Default, Eq, PartialEq)]
pub enum IOMap2 {
  #[default]
  XOscReady = 0,
}

#[repr(u8)]
#[derive(BinarySerde, Debug, Default, Eq, PartialEq)]
pub enum IOMap3 {
  #[default]
  /// DIO3 carries PLL lock Tx in transmit mode, PLL lock Rx in receive
  /// mode. This is confusing since the IC has duplex mode. It might be
  /// best to map PLL lock Rx to DIO0 and Pll lock Tx to DIO1, at the
  /// expense of losing the low-battery indication, which can be polled
  /// from the status register.
  PLLLockRxTx = 0,
}

#[derive(Debug, Default, Eq, PartialEq)]
#[binary_serde_bitfield(order = BitfieldBitOrder::MsbFirst)]
/// SX1255 hardware mapping of the 4 DIO pins.
pub struct IOMap {
    #[bits(2)]
    pub iomap0: IOMap0,
    #[bits(2)]
    pub iomap1: IOMap1,
    #[bits(2)]
    pub iomap2: IOMap2,
    #[bits(2)]
    pub iomap3: IOMap3,
}

#[repr(u8)]
#[derive(BinarySerde, Debug, Default, Eq, PartialEq)]
/// This selects the clock for the transmit DAC only. For synchronization,
/// it's recommended to use the internal clock, so that the transmit DAC
/// and the I²S interface will be synchronized.
/// [ClockSelect::clock_select_tx_dac]
pub enum ClockSelectTxDAC {
  #[default]
  Internal = 0, // Recommended.
  External = 1,
}

#[derive(Debug, Default, Eq, PartialEq)]
#[binary_serde_bitfield(order = BitfieldBitOrder::MsbFirst)]
/// SX1255 clock select register.
pub struct ClockSelect {
    #[bits(4)]
    #[doc(hidden)]
    _unused: (),
    #[bits(1)]
    /// Enables the digital loop-back mode of the front-end.
    pub dig_loopback_enable: bool,

    #[bits(1)]
    /// Enables the RF loop-back mode of the front-end.
    /// This can be used by software to calibrate receiver and transmitter
    /// I/Q gain mismatch and phase imbalance, and transmitter
    /// DC offset.
    /// 
    pub rf_loopback_enable: bool,

    #[bits(1)]
    /// Enables clock output on the CLK_OUT pin.
    pub clock_output_enable: bool,

    #[bits(1)]
    /// 
    pub clock_select_tx_dac: ClockSelectTxDAC,
}

#[derive(Debug, Default, Eq, PartialEq)]
#[binary_serde_bitfield(order = BitfieldBitOrder::MsbFirst)]
/// SX1255 hardware mapping of status bits.
pub struct Status {
    #[bits(4)]
    #[doc(hidden)]
    _unused: (),
    #[bits(1)]

    /// Set if the supply voltage gets too low.
    pub eol: bool,

    #[bits(1)]
    /// Set when the oscillator is stable.
    pub xosc_ready: bool,

    #[bits(1)]
    /// Set when the receive PLL is locked.
    pub pll_lock_rx: bool,

    #[bits(1)]
    /// Set when the transmit PLL is locked.
    pub pll_lock_tx: bool,
}

#[repr(u8)]
#[derive(BinarySerde, Debug, Default, Eq, PartialEq)]
pub enum IISMMode {
  #[default]
  /// In mode A, the IQ signals are directly from the sigma-delta modulator
  /// in receive, and to the FIR-DAC in transmit.
  A = 0,

  /// In mode B1, the IQ signals are pre and post-processed by the
  /// internal digital bridge, decimated on receive and interpolated
  /// upon transmit. Data I/O is to the I_IN, Q_IN, I_OUT, and Q_OUT
  /// pins.
  /// Full duplex is possible, with input and output running simultaneously,
  /// probably requiring two CPU I²S interfaces.
  /// DIO2 carries the WS pin, WS is one CLOCK_OUT period ahead of time.
  ///
  /// See the datasheet section on TX Noise Shaper to understand the
  /// preprocessing requirements for transmit data.
  B1 = 1,
 
  /// In mode B2, the IQ signals are pre and post-processed by the
  /// internal digital bridge, decimated on receive and interpolated
  /// upon transmit. Data I/O is to the I_IN, and I_OUT pins, with I
  /// and Q data interleaved.
  /// Full duplex is possible, with input and output running simultaneously,
  /// probably requiring two CPU I²S interfaces.
  /// DIO2 carries the WS pin, which is asserted
  /// once per IQ pair, WS=0 corresponds to I, WS=1 to Q.
  /// WS is one CLOCK_OUT period ahead of time. This is most compatible with
  /// I²S implementations of CPUs.
  ///
  /// See the datasheet section on TX Noise Shaper to understand the
  /// preprocessing requirements for transmit data.
  B2 = 2,
}

#[repr(u8)]
#[derive(BinarySerde, Debug, Default, Eq, PartialEq)]
pub enum IISMClockDiv {
  #[default]
  D0 = 0,
  D2 = 1,
  D4 = 2,
  D8 = 3,
  D12 = 4,
  D16 = 5,
  D24 = 6,
  D32 = 7,
  D64 = 8,
}

#[derive(Debug, Default, Eq, PartialEq)]
#[binary_serde_bitfield(order = BitfieldBitOrder::MsbFirst)]
/// SX1255 hardware mapping of IO control.
pub struct IISM {
  #[bits(1)]
  pub rx_during_tx_disable: bool,

  #[bits(1)]
  pub tx_during_rx_disable: bool,

  #[bits(2)]
  pub mode: IISMMode,

  #[bits(4)]
  pub clock_div: IISMClockDiv
}

#[repr(u8)]
#[derive(BinarySerde, Debug, Default, Eq, PartialEq)]
/// Values for [DigBridge::int_dec_mantissa]
pub enum IntDecMantissa {
  #[default]
  /// Mantissa is 8. Interpolation increases the effective number of data
  /// bits, the effective bits per interpolation/decimation are from
  /// the first set of tables in the Mode B section of the data sheet.
  M8 = 0,

  /// Mantissa is 9. Interpolation increases the effective number of data
  /// bits, the effective bits per interpolation/decimation are from
  /// the second set of tables in Mode B section of the data sheet.
  M9 = 1,
}

#[repr(u8)]
#[derive(BinarySerde, Debug, Default, Eq, PartialEq)]
/// Values for [DigBridge::iism_truncation]
pub enum IISMTruncation {
  #[default]
  /// Truncate MSB, align upon LSB.
  MSB = 0,

  /// Truncate LSB, align upon MSB.
  LSB = 1,
}

#[derive(Debug, Default, Eq, PartialEq)]
#[binary_serde_bitfield(order = BitfieldBitOrder::MsbFirst)]
/// SX1255 hardware mapping of status bits.
pub struct DigBridge {
    #[bits(1)]
    /// Interpolation / Decimation factor = mantissa * 3^m * 2^n
    /// mantissa: 0 = 8, 1 = 9
    ///
    /// See the data sheet on Mode B for the number of effective DAC
    /// bits per sample, for each interpolation/decimation.
    ///
    /// In duplex mode, receive and transmit interpolation must be
    /// identical to keep the input and output I²S in sync. So, the
    /// interpolation and decimation parameters are not set independently
    /// for transmit and receive.
    pub int_dec_mantissa: IntDecMantissa,

    #[bits(1)]
    /// Interpolation / Decimation factor = mantissa * 3^m * 2^n
    /// m value.
    pub int_dec_m_parameter: u8,

    #[bits(3)]
    /// Interpolation / Decimation factor = mantissa * 3^m * 2^n
    /// n value. Valid range is 0..=6.
    pub int_dec_n_parameter: u8,

    #[bits(1)]
    /// IISM truncation. The parallel data bus is expected to be 32 bits,
    /// but the effective number of data bits from interpolation/decimation
    /// is different. Thus, there are two truncation modes here.
    /// 0 = truncate MSB, align on LSB.
    /// 1 = truncate LSB, align on MSB.
    pub iism_truncation: IISMTruncation,

    #[bits(1)]
    /// Set when the selected values are invalid and force the IISM off.
    pub iism_status: bool,

    #[bits(1)]
    #[doc(hidden)]
    _unused: (),
}

#[repr(u8)]
#[derive(BinarySerde, Debug, Default, Eq, PartialEq)]
/// Values for [LowBatteryThreshold::threshold]
pub enum ThresholdValue {
  #[default]
  V2_516 = 0,
  V2_619 = 1,
  V2_724 = 2,
  V2_829 = 3,
  V2_935 = 4,
  V3_037 = 5,
  V3_143 = 6,
  V3_245 = 7,
}

#[derive(Debug, Default, Eq, PartialEq)]
#[binary_serde_bitfield(order = BitfieldBitOrder::MsbFirst)]
/// SX1257 hardware mapping of low battery threshold register.
/// This is not documented for SX1255.
pub struct LowBatteryThreshold {
    #[bits(5)]
    _unused: (),

    #[bits(3)]
    pub threshold: ThresholdValue,
}

#[doc = include_str!("../markdown/hard_registers.md")]
#[derive(Debug, Default, Eq, PartialEq)]
pub struct HardRegisters {
    pub mode: Mode,
    pub rx: Frequency,
    pub tx: Frequency,
    pub version: Version,
    pub tx_frontend: TxFrontend,
    pub rx_frontend: RxFrontend,
    pub io_map: IOMap,
    pub clock_select: ClockSelect,
    pub status: Status,
    pub iism: IISM,
    pub dig_bridge: DigBridge,
}

impl HardRegisters {
    pub fn serialize(&self, bytes: &mut [u8; 20]) {
        // There is probably a better way to do this with BinarySerdeBufSafe
        // and traits, but I haven't out how to do that yet. In the meantime,
        // this should run at least as quickly and well, but the code makes use
        // of ranges derived from the packed size of the individual structs that
        // we could avoid.
        const E : binary_serde::Endianness = Endianness::Big;
        self.mode.binary_serialize(&mut bytes[0..=0], E);
        self.rx.binary_serialize(&mut bytes[1..=3], E);
        self.tx.binary_serialize(&mut bytes[4..=6], E);
        self.version.binary_serialize(&mut bytes[7..=7], E);
		self.tx_frontend.binary_serialize(&mut bytes[8..=0xB], E);
        self.rx_frontend.binary_serialize(&mut bytes[0xC..=0xE], E);
        self.io_map.binary_serialize(&mut bytes[0xF..=0xF], E);
        self.clock_select.binary_serialize(&mut bytes[0x10..=0x10], E);
        self.status.binary_serialize(&mut bytes[0x11..=0x11], E);
        self.iism.binary_serialize(&mut bytes[0x12..=0x12], E);
        self.dig_bridge.binary_serialize(&mut bytes[0x13..=0x13], E);
    }
}


#[doc(hidden)]
/// This function isn't meant to be used. It doesn't do anything but provide
/// references for the exported code in this module, so that I don't have to
/// spread #[allow(dead_code)] all over, just in one place here, and can benefit
/// from dead-code notification where something is *actually* dead.
#[allow(dead_code)]
fn _stub() {
    let reg: HardRegisters = HardRegisters::default();
    let mut data: [u8; 20] = [0; 20];
    reg.serialize(&mut data);
}
