use crate::evm::Evm;
use crate::log_utils::*;
use crate::ops::traits::*;
use crate::utils::*;
use num_bigint::BigUint;
use num_traits::zero;
impl CurrentBlockInfo for Evm {
    fn basefee(&mut self) {
        self.stack
            .push((self.current_block.get_basefee().clone(), 0u8));
    }
    fn blockhash(&mut self) {
        if self.stack.len() < 1 {
            panic!("stack underflow!");
        }
        let block_num = get_uint256(self.stack.pop().unwrap());
        if block_num == *self.current_block.get_number() {
            self.stack.push((block_num, 0u8));
        } else {
            self.stack.push((zero(), 0u8));
        }
    }
    fn chainid(&mut self) {
        self.stack
            .push((self.current_block.get_chainid().clone(), 0u8));
    }
    fn coinbase(&mut self) {
        self.stack
            .push((self.current_block.get_coinbase().clone(), 0u8));
    }
    fn gaslimit(&mut self) {
        self.stack
            .push((self.current_block.get_gaslimit().clone(), 0u8));
    }
    fn number(&mut self) {
        self.stack
            .push((self.current_block.get_number().clone(), 0u8));
    }
    fn prevrandao(&mut self) {
        self.stack
            .push((self.current_block.get_prevrandao().clone(), 0u8));
    }
    fn selfbalance(&mut self) {
        self.stack
            .push((self.current_block.get_selfbalance().clone(), 0u8));
    }
    fn timestamp(&mut self) {
        self.stack
            .push((self.current_block.get_timestamp().clone(), 0u8));
    }
}
