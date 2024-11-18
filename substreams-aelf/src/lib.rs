use bs58;
use sha2::{Sha256, Digest};
pub use substreams_aelf_core::pb;
use substreams_aelf_core::pb::aelf::Address;
use prost::Message;


/// Decodes a base58 string with a checksum. Returns the decoded data without the checksum.
fn b58_to_address(b58_string: &str) -> Result<Address, String> {
    // Decode the base58 string to bytes
    let bytes = bs58::decode(b58_string).into_vec().map_err(|e| format!("Base58 decoding error: {:?}", e))?;

    // A valid base58check string should be at least 4 bytes for the checksum
    if bytes.len() < 4 {
        return Err("Input too short to contain checksum".to_string());
    }

    // Split the bytes into the data and the checksum
    let (data, checksum) = bytes.split_at(bytes.len() - 4);

    // Calculate the checksum of the data
    let computed_checksum = &Sha256::digest(&Sha256::digest(data))[..4];

    // Verify the checksum
    if checksum != computed_checksum {
        return Err("Checksum does not match".to_string());
    }

    if data.len() != 32 {
        return        Err("Invalid address".to_string());
    }

    let address = Address {
        value: data.to_vec()
    };

    Ok(address)
}

#[macro_export]
macro_rules! address {
    ($string:expr) => {{
        b58_to_address($string).unwrap()
    }};
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address() {
        let address = address!("2DZER7qHVwv3PUMFsHuQaQbE4wDFsCRzJsxLwYEk8rgM3HVn1S");
        let expected = Address{
            value: vec![0xa0, 0x34, 0x93, 0x82, 0xb8, 0x9d, 0x5f, 0x20, 0x72, 0x76, 0xf0, 0x98, 0x3a, 0x4, 0x1e, 0x5a, 0x11, 0x31, 0x34, 0xb8, 0x97, 0x88, 0x8e, 0x39, 0x99, 0x1c, 0x5b, 0x59, 0xde, 0x23, 0xd, 0x16]
        };

        assert_eq!(address, expected);
    }
}
