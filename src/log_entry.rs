use num_bigint::BigUint;
#[derive(Debug, Clone)]
pub struct LogEntry {
    address: String,
    data: Vec<u8>,
    topics: Vec<BigUint>,
}

impl LogEntry {
    pub fn init(address: String, data: Vec<u8>, topics: Vec<BigUint>) -> Self {
        Self {
            address: address,
            data: data,
            topics: topics,
        }
    }
}
