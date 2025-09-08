# Rust software for Semtech SX1255/SX1257.

The Semtech SX1255 and SX1257 are SDR transceiver front-end integrated circuits,
SX1255 is specified for 400-512 MHz and can commonly work down to 350 MHz. 
SX1257 is specified to operate in ISM bands from 860 to 1000 MHz.
They include separate transmit and receive frequency synthesizers and go from
RF to bits, and back, for the transmit and receive path, with full-duplex
capability. Together with a system-on-a-chip CPU module and a minimum of
additional components, they are a complete SDR transceiver, potentially with
a GUI, local wireless networking, and other features.

If you aren't using the full-duplex capability (or possibly even if you are, by
multiplexing) it is also possible to use it to sample your power amplifier for
pre-distortion, since the system is capable of producing modulations that
require excellent linearity.

Rustdoc API Documentation is [here](BrucePerens.github.io/sx1255-doc).

This is a work-in-progress that I created for my own education, and is not
proposed for any project at this time, although it is obviously inspired
by LinHT. It is not under an Open Source license at the moment, I will deal
with that if someone pays me to do so, or when the project is done.
