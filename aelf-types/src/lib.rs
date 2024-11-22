mod pb;
mod extensions;

pub use pb::aelf::Address;
pub use pb::aelf::Hash;


#[macro_export]
macro_rules! address {
    ($string:expr) => {{
        Address::from_b58($string).ok()
    }};
}

#[macro_export]
macro_rules! hash {
    ($string:expr) => {{
        Hash::from_hex($string).ok()
    }};
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address() {
        let address = address!("2DZER7qHVwv3PUMFsHuQaQbE4wDFsCRzJsxLwYEk8rgM3HVn1S");
        let expected = Some(Address {
            value: vec![0xa0, 0x34, 0x93, 0x82, 0xb8, 0x9d, 0x5f, 0x20, 0x72, 0x76, 0xf0, 0x98, 0x3a, 0x4, 0x1e, 0x5a, 0x11, 0x31, 0x34, 0xb8, 0x97, 0x88, 0x8e, 0x39, 0x99, 0x1c, 0x5b, 0x59, 0xde, 0x23, 0xd, 0x16]
        });

        assert_eq!(address, expected);
    }

    #[test]
    fn test_hash() {
        let hash = hash!("a0349382b89d5f207276f0983a041e5a113134b897888e39991c5b59de230d16");
        let expected = Some(Hash {
            value: vec![0xa0, 0x34, 0x93, 0x82, 0xb8, 0x9d, 0x5f, 0x20, 0x72, 0x76, 0xf0, 0x98, 0x3a, 0x4, 0x1e, 0x5a, 0x11, 0x31, 0x34, 0xb8, 0x97, 0x88, 0x8e, 0x39, 0x99, 0x1c, 0x5b, 0x59, 0xde, 0x23, 0xd, 0x16]
        });
        assert_eq!(hash, expected);
    }
}
