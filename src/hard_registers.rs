use binary_serde::*;

#[derive(Debug, Default, PartialEq, Eq)]
#[binary_serde_bitfield(order = BitfieldBitOrder::MsbFirst)]
/// # SX1255 hardware mode register.
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
    /// Enable power supplies and oscillator.
    pub ref_enable: bool,
}

#[derive(Debug, Default, PartialEq, Eq)]
#[binary_serde_bitfield(order = BitfieldBitOrder::MsbFirst)]
/// Integer frequency value.
/// The actual frequency will be
/// (oscillator_frequency * frequency_value) / 2^20 .
///
/// 0xC0E38E is the default value of the hardware register, and should 
/// result in 434 MHz with a 36 MHz crystal.
/// The resolution will be 34.3323 Hz if the oscillator is 36 MHz.
/// This value is read only when it is written and the IC then leaves SLEEP
/// mode by a translation of
/// [Mode::ref_enable](self::Mode::ref_enable)
/// from 0 to 1.

pub struct Frequency {
  #[bits(24)]
  frequency: u32,
}

#[derive(Debug, Default, PartialEq, Eq)]
#[binary_serde_bitfield(order = BitfieldBitOrder::MsbFirst)]
/// SX1255 hardware transmit front-end control register.
pub struct Version {
    #[bits(4)]
    fill_revision_number: u8,
    #[bits(4)]
    metal_mask_revision_number: u8,
}

#[repr(u8)]
#[derive(Debug, BinarySerde, Default, PartialEq, Eq)]
/// Settings for [TxFrontend.tx_mixer_tank_res]
enum TxMixerTankResistance {
    #[default]
    R950Ohm = 0,
    R1110Ohm = 1,
    R1320Ohm = 2,
    R1650Ohm = 3,
    R2180Ohm = 4,
    R3240Ohm = 5,
    R6000Ohm = 6,
    R64000Ohm = 7
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
    pub mixer_tank_res: TxMixerTankResistance,

    #[bits(1)]
    #[doc(hidden)]
    pub _unused3: (),

    #[bits(2)]
	/// Transmit PLL bandwidth, (value + 1) * 75 KHz.
    pub pll_bw: u8,

    #[bits(5)]
    /// Transmit analog filter bandwidth (DSB),
    /// 3 db bandwidth in MHz = 17.15 * (41 - value).
    pub filter_bw: u8,

    #[bits(5)]
    #[doc(hidden)]
    pub _unused4: (),

    #[bits(3)]
    /// Number of taps of FIR-DAC.
    /// number of taps = 24 + (8 * value), maximum is 64.
    pub dac_bw: u8
}

#[repr(u8)]
#[derive(Debug, BinarySerde, Default, PartialEq, Eq)]
/// Settings for [RxFrontend.rx_zin]
enum RxZIn {
    #[default]
    I50Ohm = 0,
    I200Ohm = 1,
}

#[repr(u8)]
#[derive(Debug, BinarySerde, Default, PartialEq, Eq)]
/// Settings for [RxFrontend.rx_adc_bw]
/// The data sheet has a cryptic comment: "use 0x01 instead".
enum RxADCBw {
    #[default]
    BWOver400KHz = 7,
    BW200To400KHz = 5,
    BW100To400KHz = 2,
}

#[repr(u8)]
#[derive(Debug, BinarySerde, Default, PartialEq, Eq)]
/// Settings for [RxFrontend.rx_pga_bw]
enum RxPGABw {
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
    /// Receive PLL bandwidth. bandwidth = (value + 1) * 75 KHz.
    pub rx_pll_bw: u8,

    #[bits(1)]
    /// Puts the receive ADC into temperature-measurement mode.
    pub rx_adc_temp: bool,
}

#[derive(BinarySerde, Debug, Default, Eq, PartialEq)]
#[repr(u8)]
pub enum IOMap0 {
  #[default]
  PLLLockRx = 0,
  PLLLockRx1 = 1,
  PLLLockRx2 = 2,
  Eol = 3
}

#[derive(BinarySerde, Debug, Default, Eq, PartialEq)]
#[repr(u8)]
pub enum IOMap1 {
  #[default]
  PLLLockTx = 0,
}

#[derive(BinarySerde, Debug, Default, Eq, PartialEq)]
#[repr(u8)]
pub enum IOMap2 {
  #[default]
  XOscReady = 0,
}

#[derive(BinarySerde, Debug, Default, Eq, PartialEq)]
#[repr(u8)]
pub enum IOMap3 {
  #[default]
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

#[derive(BinarySerde, Debug, Default, Eq, PartialEq)]
#[repr(u8)]
/// Values for [ClockSelect.clock_select_tx_dac].
pub enum ClockSelectTxDAC {
  #[default]
  Internal = 0,
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
    eol: bool,

    #[bits(1)]
    /// Set when the oscillator is stable.
    xosc_ready: bool,

    #[bits(1)]
    /// Set when the receive PLL is locked.
    pll_lock_rx: bool,

    #[bits(1)]
    /// Set when the transmit PLL is locked.
    pll_lock_tx: bool,
}

#[derive(BinarySerde, Debug, Default, Eq, PartialEq)]
#[repr(u8)]
pub enum IISMMode {
  #[default]
  A = 0,
  B1 = 1,
  B2 = 2,
}

#[derive(BinarySerde, Debug, Default, Eq, PartialEq)]
#[repr(u8)]
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
  rx_during_tx_disable: bool,

  #[bits(1)]
  tx_during_rx_disable: bool,

  #[bits(2)]
  mode: IISMMode,

  #[bits(4)]
  clock_div: IISMClockDiv
}

#[derive(BinarySerde, Debug, Default, Eq, PartialEq)]
#[repr(u8)]
pub enum IntDecMantissa {
  #[default]
  M8 = 0,
  M9 = 1,
}

#[derive(BinarySerde, Debug, Default, Eq, PartialEq)]
#[repr(u8)]
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
    int_dec_mantissa: IntDecMantissa,

    #[bits(1)]
    /// Interpolation / Decimation factor = mantissa * 3^m * 2^n
    /// m value.
    int_dec_m_parameter: u8,

    #[bits(3)]
    /// Interpolation / Decimation factor = mantissa * 3^m * 2^n
    /// n value. Valid range is 0..=6.
    int_dec_n_parameter: u8,

    #[bits(1)]
    /// IISM truncation. 0 = truncate MSB, align on LSB.
    /// 1 = truncate LSB, align on MSB.
    iism_truncation: IISMTruncation,

    #[bits(1)]
    /// Set when the selected values are invalid and force the IISM off.
    iism_status: bool,

    #[bits(1)]
    #[doc(hidden)]
    _unused: (),
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
/// spread #[ignore(dead_code)] all over, and can benefit from dead-code
/// notification where something is *actually* dead.
#[allow(dead_code)]
fn _stub() {
    let reg: HardRegisters = HardRegisters::default();
    let mut data: [u8; 20] = [0; 20];
    reg.serialize(&mut data);
}
