use std::fmt;

#[derive(Debug)]
pub struct Address {
    address_type: u8,
    address: u64,
}

impl Address {
    pub fn new(address_type: u8, address: u64) -> Self {
        Address {
            address_type: address_type,
            address: address,
        }
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = self.address.to_be_bytes();

        write!(
            f,
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X} ({:02X})",
            bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7], self.address_type,
        )
    }
}
