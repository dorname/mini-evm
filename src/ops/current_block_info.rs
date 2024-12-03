use crate::evm::Evm;
use crate::log_utils::*;
use crate::ops::traits::*;
use crate::stack::StackData;
use crate::utils::*;
use num_bigint::BigUint;
use num_traits::zero;
impl CurrentBlockInfo for Evm {
    fn basefee(&mut self) {
        self.stack
            .push(StackData::new(self.current_block.get_basefee().to_bytes_be(), 0u8));
    }
    fn blockhash(&mut self) {
        if self.stack.len() < 1 {
            panic!("stack underflow!");
        }
        let block_num = get_uint256(self.stack.pop());
        if block_num == *self.current_block.get_number() {
            self.stack.push(StackData::new(block_num.to_bytes_be(), 0u8));
        } else {
            self.stack.push(StackData::new(0u8.to_be_bytes().to_vec(), 0u8));
        }
    }
    fn chainid(&mut self) {
        self.stack
            .push(StackData::new(self.current_block.get_chainid().to_bytes_be(), 0u8));
    }
    fn coinbase(&mut self) {
        self.stack
            .push(StackData::new(self.current_block.get_coinbase().to_bytes_be(), 0u8));
    }
    fn gaslimit(&mut self) {
        self.stack
            .push(StackData::new(self.current_block.get_gaslimit().to_bytes_be(), 0u8));
    }
    fn number(&mut self) {
        self.stack
            .push(StackData::new(self.current_block.get_number().to_bytes_be(), 0u8));
    }
    fn prevrandao(&mut self) {
        self.stack
            .push(StackData::new(self.current_block.get_prevrandao().to_bytes_be(), 0u8));
    }
    fn selfbalance(&mut self) {
        self.stack
            .push(StackData::new(self.current_block.get_selfbalance().to_bytes_be(), 0u8));
    }
    fn timestamp(&mut self) {
        self.stack
            .push(StackData::new(self.current_block.get_timestamp().to_bytes_be(), 0u8));
    }
}
