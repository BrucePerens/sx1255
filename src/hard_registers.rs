#[allow(unused_imports)]
use binary_serde::*;
#[allow(unused_imports)]
use serde_repr::{Serialize_repr, Deserialize_repr};

/// Hardware registers for the SX1255. These are not directly used by the
/// programmer, instead module soft_registers and struct SoftRegisters define
///  a more convenient format with input in the engineer's accustomed units: dB,
/// femtoFarad, ohm, etc. rather than a binary encoding. SoftRegisters.write
/// checks a copy of itself that represents the last written value, and updates
/// the hardware registers as required.
///
/// At this writing the SX1255 data sheet was at
/// https://semtech.my.salesforce.com/sfc/p/#E0000000JelG/a/44000000MDmE/Qs9oRoa8Sbb6mkImE9mtMh47H5LFx6KMbGcpb8L28SE
/// All the structures and fields correspond to the data sheet names,
/// in a few places I've attempted to make the names more readable, but they
/// should still be recognizable. 


#[derive(Debug, Default, PartialEq, Eq)]
#[binary_serde_bitfield(order = BitfieldBitOrder::MsbFirst)]
/// SX1255 hardware mode register.
pub(crate) struct Mode {
    #[bits(4)]
    pub _unused: (),
    #[bits(1)]
    pub driver_enable: bool,/// Power amplifier enable.
    #[bits(1)]
    pub tx_enable: bool,	/// Transmit enable _except_ power amplifier.
    #[bits(1)]
    pub rx_enable: bool,	/// Receiver enable.
    #[bits(1)]
    pub ref_enable: bool,	// Enable power supplies and oscillator.
}

#[derive(Debug, Default, PartialEq, Eq)]
#[binary_serde_bitfield(order = BitfieldBitOrder::MsbFirst)]
/// Integer frequency that serializes to 24 bits, as represented on SX1255.
pub(crate) struct Frequency {
  #[bits(24)]
  frequency: u32,
}

#[derive(Debug, Default, PartialEq, Eq)]
#[binary_serde_bitfield(order = BitfieldBitOrder::MsbFirst)]
/// SX1255 hardware transmit front-end control register.
pub(crate) struct TxFrontend {
    #[bits(1)]
    pub _unused1: (),
    #[bits(3)]
    pub dac_gain: u8,
    #[bits(4)]
    pub mixer_gain: u8,
    #[bits(2)]
    pub _unused2: (),
    #[bits(3)]
    pub mixer_tank_cap: u8,
    #[bits(3)]
    pub mixer_tank_res: u8,
    #[bits(1)]
    pub _unused3: (),
    #[bits(2)]
    pub pll_bw: u8,
    #[bits(5)]
    pub filter_bw: u8,
    #[bits(5)]
    pub _unused4: (),
    #[bits(3)]
    pub dac_bw: u8
}

#[derive(Debug, Default, PartialEq, Eq)]
#[binary_serde_bitfield(order = BitfieldBitOrder::MsbFirst)]
/// SX1255 hardware receive front-end control register.
pub(crate) struct RxFrontend {
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
    pub _unused: (),
    #[bits(2)]
    pub rx_pll_bw: u8,
    #[bits(1)]
    pub rx_adc_temp: bool,
}

#[derive(BinarySerde, Debug, Default, Eq, PartialEq)]
#[repr(u8)]
pub(crate) enum IOMap0 {
  #[default]
  PLLLockRx = 0,
  PLLLockRx1 = 1,
  PLLLockRx2 = 2,
  Eol = 3
}

#[derive(BinarySerde, Debug, Default, Eq, PartialEq)]
#[repr(u8)]
pub(crate) enum IOMap1 {
  #[default]
  PLLLockTx = 0,
}

#[derive(BinarySerde, Debug, Default, Eq, PartialEq)]
#[repr(u8)]
pub(crate) enum IOMap2 {
  #[default]
  XOscReady = 0,
}

#[derive(BinarySerde, Debug, Default, Eq, PartialEq)]
#[repr(u8)]
pub(crate) enum IOMap3 {
  #[default]
  PLLLockRxTx = 0,
}

#[derive(Debug, Default, Eq, PartialEq)]
#[binary_serde_bitfield(order = BitfieldBitOrder::MsbFirst)]
/// SX1255 hardware mapping of the 4 DIO pins.
pub(crate) struct IOMap {
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
pub(crate) struct ClockSelect {
    #[bits(4)]
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
pub(crate) struct Status {
    #[bits(4)]
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
pub(crate) enum IISMMode {
  #[default]
  A = 0,
  B1 = 1,
  B2 = 2,
}

#[derive(Debug, Default, Eq, PartialEq)]
#[binary_serde_bitfield(order = BitfieldBitOrder::MsbFirst)]
/// SX1255 hardware mapping of status bits.
pub(crate) struct IISM {
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
pub(crate) struct DigBridge {
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
  _unused: (),
}

/// This holds all of the hardware versions of the SX1255 registers. It's
/// only used by soft_registers::SoftRegisters.write(), which serializes a
/// soft representation of the registers in the engineer's accustomed units
/// (dB, etc.)into a bit vector representing the hardware encoding, and
/// suitable for writing to the SX1255.
#[derive(Debug, Default, Eq, PartialEq)]
pub(crate) struct HardRegisters {
    pub mode: Mode,
    pub rx: Frequency,
    pub tx: Frequency,
    pub version: u8,
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
