# Soft representation of Semtech SX1255/SX1257 registers.
Rather than use the data formatting of the IC, this soft representation
of the registers uses
units familiar to engineers, like dB, femtofarad, ohm, etc. This keeps
chip-specific constants from propagating throughout the software, and
is easier to program.

There are three steps between this structure and the IC: the
[Control::write](fn@crate::control::Control::write)
method translates this object to a
[HardRegisters](struct@crate::hard_registers::HardRegisters)
object, which makes use of IC-specific constants rather than the familiar
engineering units of
[Control](struct@crate::control::Control), and is closer to the data
format of the actual IC hardware registers. However,
[HardRegisters](struct@crate::hard_registers::HardRegisters)
still
doesn't quite match the IC, because the IC uses bit fields and data sizes
that don't directly match Rust data types. Unlike C, Rust does not directly
support bit-fields, instead using serialization[^bit_twiddling].
Thus, a 24-bit frequency on
the IC is represented by a [u32](std::primitive::u32) in a Rust program, which must be serialized to
24 bits.

So, the [HardRegisters](struct@crate::hard_registers::HardRegisters)
object is serialized to an array of 20 u8 data, in the format
actually used by the device. Finally, the data is written to the device
over an SPI bus.
[^bit_twiddling]: C bit-fields were often not useful for direct manipulation
of devices, since they inherently must use read-modify-write to change
bits in a machine word, and the device often would not respond to a bus
read as expected. C progammers generally learned to manipulate hardware
registers using constants and bitwise operators. In place of that,
this software tries to encapsulate some hardware register particulars
away from the programmer, using enough cycles to horrify a 1970's C
programmer, but still too few to be relevant in a modern implementation.
