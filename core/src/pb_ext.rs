use std::iter::{Chain, Filter};
use std::path::Iter;
use crate::pb::aelf::{Address, ExecutionStatus, Hash, TransactionExecutingStateSet, TransactionTrace};
use bs58;
use sha2::{Sha256, Digest};
use crate::pb::aelf::ExecutionStatus::Executed;

impl Address {
    pub fn from_b58(b58: &str) -> Result<Self, String> {
        // Decode the base58 string to bytes
        let bytes = bs58::decode(b58).into_vec().map_err(|e| format!("Base58 decoding error: {:?}", e))?;

        // A valid base58check string should be at least 4 bytes for the checksum
        if bytes.len() < 4 {
            return Err("Input too short to contain checksum".to_string());
        }

        // Split the bytes into the data and the checksum
        let (data, checksum) = bytes.split_at(bytes.len() - 4);

        // Calculate the checksum of the data
        let computed_checksum = &Sha256::digest(Sha256::digest(data))[..4];

        // Verify the checksum
        if checksum != computed_checksum {
            return Err("Checksum does not match".to_string());
        }

        if data.len() != 32 {
            return Err("Invalid address".to_string());
        }

        let address = Address {
            value: data.to_vec()
        };

        Ok(address)
    }
    fn to_b58(&self) -> String {
        // Calculate the checksum
        let hash = calc_sha256(self.value.as_slice());
        let hash = calc_sha256(hash.as_slice());
        let checksum = &hash[..4]; // Use the first 4 bytes as the checksum

        // Append the checksum to the value
        let mut data_with_checksum = self.value.clone();
        data_with_checksum.extend_from_slice(checksum);

        // Encode the data with checksum
        bs58::encode(data_with_checksum).into_string()
    }
}

fn calc_sha256(bytes: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hasher.finalize().to_vec()
}


impl Hash {
    pub fn from_hex(hex_string: &str) -> Result<Self, String> {
        let bytes = hex::decode(hex_string).map_err(|e| format!("Hex decoding error: {:?}", e))?;
        if bytes.len() != 32 {
            return Err("Invalid hash".to_string());
        }
        Ok(Hash { value: bytes })
    }
    pub fn to_hex(&self) -> String {
        hex::encode(self.value.as_slice())
    }
}

impl TransactionTrace {
    pub fn is_successful(&self) -> bool {
        if self.execution_status != ExecutionStatus::Executed.into() { return false; }
        if self.pre_traces.iter().any(|trace| !trace.is_successful()) { return false; }
        if self.inline_traces.iter().any(|trace| !trace.is_successful()) { return false; }
        if self.post_traces.iter().any(|trace| !trace.is_successful()) { return false; }
        return true;
    }
    pub fn iter_valid_state_changes(&self) -> Box<dyn Iterator<Item=TransactionExecutingStateSet>> {
        if self.is_successful() {
            return self.iter_state_changes();
        }
        Box::new(self.pre_traces.iter()
            .chain(self.post_traces.iter())
            .filter(|trace| trace.is_successful())
            .flat_map(|trace| trace.iter_state_changes()))
    }
    pub fn iter_state_changes(&self) -> Box<dyn Iterator<Item=TransactionExecutingStateSet>> {
        Box::new(self.pre_traces.iter().flat_map(|trace| trace.iter_state_changes())
            .chain(self.state_set.iter())
            .chain(self.inline_traces.iter().flat_map(|trace| trace.iter_state_changes()))
            .chain(self.post_traces.iter().flat_map(|trace| trace.iter_state_changes()))
            .iter())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address() {
        let address = Address::from_b58("2DZER7qHVwv3PUMFsHuQaQbE4wDFsCRzJsxLwYEk8rgM3HVn1S").ok();
        let expected = Some(Address {
            value: vec![0xa0, 0x34, 0x93, 0x82, 0xb8, 0x9d, 0x5f, 0x20, 0x72, 0x76, 0xf0, 0x98, 0x3a, 0x4, 0x1e, 0x5a, 0x11, 0x31, 0x34, 0xb8, 0x97, 0x88, 0x8e, 0x39, 0x99, 0x1c, 0x5b, 0x59, 0xde, 0x23, 0xd, 0x16]
        });
        assert_eq!(address, expected);
        assert_eq!(address.expect("ok").to_b58(), "2DZER7qHVwv3PUMFsHuQaQbE4wDFsCRzJsxLwYEk8rgM3HVn1S");
    }

    #[test]
    fn test_hash() {
        let hash = Hash::from_hex("a0349382b89d5f207276f0983a041e5a113134b897888e39991c5b59de230d16").ok();
        let expected = Some(Hash {
            value: vec![0xa0, 0x34, 0x93, 0x82, 0xb8, 0x9d, 0x5f, 0x20, 0x72, 0x76, 0xf0, 0x98, 0x3a, 0x4, 0x1e, 0x5a, 0x11, 0x31, 0x34, 0xb8, 0x97, 0x88, 0x8e, 0x39, 0x99, 0x1c, 0x5b, 0x59, 0xde, 0x23, 0xd, 0x16]
        });
        assert_eq!(hash, expected);
        assert_eq!(hash.expect("ok").to_hex(), "a0349382b89d5f207276f0983a041e5a113134b897888e39991c5b59de230d16");
    }
}
