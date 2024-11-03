/// TODO
/// 实现区块链，修改成从链上读取
use num_bigint::BigUint;
#[derive(Debug, Clone)]
pub struct CurrentBlock {
    blockhash: BigUint,
    coinbase: BigUint,
    timestamp: BigUint,
    number: BigUint,
    prevrandao: BigUint,
    gaslimit: BigUint,
    chainid: BigUint,
    selfbalance: BigUint,
    basefee: BigUint,
}
impl CurrentBlock {
    pub fn init() -> Self {
        Self {
            blockhash: BigUint::from(0x7527123fc877fe753b3122dc592671bu128),
            coinbase: BigUint::from(0x388C818CA8B9251b393131C08a736A67u128),
            timestamp: BigUint::from(1625900000u128),
            number: BigUint::from(17871709u128),
            prevrandao: BigUint::from(0xce124dee50136f3f93f19667fb4u128),
            gaslimit: BigUint::from(30_u8),
            chainid: BigUint::from(1u8),
            selfbalance: BigUint::from(100u8),
            basefee: BigUint::from(30_u8),
        }
    }
    pub fn get_block_hash(&self) -> &BigUint {
        &self.blockhash
    }
    pub fn get_coinbase(&self) -> &BigUint {
        &self.coinbase
    }
    pub fn get_timestamp(&self) -> &BigUint {
        &self.timestamp
    }
    pub fn get_number(&self) -> &BigUint {
        &self.number
    }
    pub fn get_prevrandao(&self) -> &BigUint {
        &self.prevrandao
    }
    pub fn get_gaslimit(&self) -> &BigUint {
        &self.gaslimit
    }
    pub fn get_chainid(&self) -> &BigUint {
        &self.chainid
    }
    pub fn get_selfbalance(&self) -> &BigUint {
        &self.selfbalance
    }
    pub fn get_basefee(&self) -> &BigUint {
        &self.basefee
    }
}
