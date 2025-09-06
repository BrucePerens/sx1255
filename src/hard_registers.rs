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

#[derive(Debug, Default, PartialEq, Eq)]
#[binary_serde_bitfield(order = BitfieldBitOrder::MsbFirst)]
/// SX1255 hardware transmit front-end control register.
pub struct TxFrontend {
    #[bits(1)]
    #[doc(hidden)]
    pub _unused1: (),
    #[bits(3)]
    pub dac_gain: u8,
    #[bits(4)]
    pub mixer_gain: u8,
    #[bits(2)]
    #[doc(hidden)]
    pub _unused2: (),
    #[bits(3)]
    pub mixer_tank_cap: u8,
    #[bits(3)]
    pub mixer_tank_res: u8,
    #[bits(1)]
    #[doc(hidden)]
    pub _unused3: (),
    #[bits(2)]
    pub pll_bw: u8,
    #[bits(5)]
    pub filter_bw: u8,
    #[bits(5)]
    #[doc(hidden)]
    pub _unused4: (),
    #[bits(3)]
    pub dac_bw: u8
}

#[derive(Debug, Default, PartialEq, Eq)]
#[binary_serde_bitfield(order = BitfieldBitOrder::MsbFirst)]
/// SX1255 hardware receive front-end control register.
pub struct RxFrontend {
    #[bits(3)]
    pub lna_gain: u8,
    #[bits(4)]
    pub rx_pga_gain: u8,
    #[bits(1)]
    pub rx_zin: u8,
    #[bits(3)]
    pub rx_adc_bw: u8,
    #[bits(3)]
    pub rx_adc_trim: u8,
    #[bits(2)]
    pub rx_pga_bw: u8,
    #[bits(5)]
    #[doc(hidden)]
    pub _unused: (),
    #[bits(2)]
    pub rx_pll_bw: u8,
    #[bits(1)]
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

#[derive(Debug, Default, Eq, PartialEq)]
#[binary_serde_bitfield(order = BitfieldBitOrder::MsbFirst)]
/// SX1255 hardware mapping of clock select and loop-back modes.
pub struct ClockSelect {
    #[bits(4)]
    #[doc(hidden)]
    _unused: (),
    #[bits(1)]
    pub dig_loopback_enable: bool,
    #[bits(1)]
    pub rf_loopback_enable: bool,
    #[bits(1)]
    pub clock_output_enable: bool,
    #[bits(1)]
    pub clock_select_tx_dac: bool,
}

#[derive(Debug, Default, Eq, PartialEq)]
#[binary_serde_bitfield(order = BitfieldBitOrder::MsbFirst)]
/// SX1255 hardware mapping of status bits.
pub struct Status {
    #[bits(4)]
    #[doc(hidden)]
    _unused: (),
    #[bits(1)]
    eol: bool,
    #[bits(1)]
    xosc_ready: bool,
    #[bits(1)]
    pll_lock_rx: bool,
    #[bits(1)]
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

#[derive(Debug, Default, Eq, PartialEq)]
#[binary_serde_bitfield(order = BitfieldBitOrder::MsbFirst)]
/// SX1255 hardware mapping of status bits.
pub struct IISM {
  #[bits(1)]
  rx_during_tx_disable: bool,
  #[bits(1)]
  tx_during_rx_disable: bool,
  #[bits(2)]
  mode: IISMMode,
  #[bits(4)]
  clock_div: u8
}

#[derive(Debug, Default, Eq, PartialEq)]
#[binary_serde_bitfield(order = BitfieldBitOrder::MsbFirst)]
/// SX1255 hardware mapping of status bits.
pub struct DigBridge {
    #[bits(1)]
    int_dec_mantissa: u8,
    #[bits(1)]
    int_dec_m_parameter: u8,
    #[bits(3)]
    int_dec_n_parameter: u8,
    #[bits(1)]
    iism_truncation: bool,
    #[bits(1)]
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

// Retain dead code checks for the rest of this library module and mark
// everything that is exported as used.
#[allow(dead_code)]
fn _stub() {
    let reg: HardRegisters = HardRegisters::default();
    let mut data: [u8; 20] = [0; 20];
    reg.serialize(&mut data);
}
