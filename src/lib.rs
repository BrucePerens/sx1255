//! Rust software for Semtech SX1255

#[doc = include_str!("../markdown/hard_registers.md")]
pub mod hard_registers;

#[doc = include_str!("../markdown/registers.md")]
pub mod registers;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
