#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Flags(pub(crate) u8);
impl Flags {
    pub const IPV4: u8 = (1 << 0);
    pub const IPV6: u8 = (1 << 1);
    pub const EXPENSIVE: u8 = (1 << 2);
    pub const CONSTRAINED: u8 = (1 << 3);
    pub const DNS: u8 = (1 << 4);
    pub fn get_flag<const F: u8>(&self) -> bool {
        (self.0 & F) != 0
    }
}
