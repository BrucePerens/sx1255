# Semtech SX1255 Hardware Registers
This module contains structures *similar* to the the hardware registers as
used on the SX1255, and software to serialize them to the format of the
actual hardware registers. The Rust data structures differ from the actual
hardware registers where the data sizes are not directly supported by Rust,
for example frequencies on the SX1255 are 24 bits, and the Rust type for
them is a [std::primitive::u32]. Nor does Rust natively support bit-fields.
The
[HardRegisters::serialize](fn@crate::hard_registers::HardRegisters::serialize)
function serializes the data to fit the actual
register formats. 

Some of these structures are not directly used by the
programmer. Instead, the module [registers](mod@crate::registers)
defines similar data structures using more convenient data formats in the
engineer's accustomed units: dB, femtoFarad, ohm, etc.; rather than a binary
encoding. [Registers::write](fn@crate::registers::Registers::write)
translates its data to the
[HardRegisters](struct@crate::hard_registers::HardRegisters)
data format, serializes that to the actual data format used by the IC hardware,
and uses the SPI bus to write the hardware registers.

At this writing the SX1255 data sheet was at
<https://semtech.my.salesforce.com/sfc/p/#E0000000JelG/a/44000000MDmE/Qs9oRoa8Sbb6mkImE9mtMh47H5LFx6KMbGcpb8L28SE>

All the structures and fields correspond to the data sheet names, except that
in a few places I've attempted to make the names more readable, but they
should still be recognizable. 
