use core::panic;

use crate::{evm::Evm, stack::StackData};
use crate::log_utils::*;
use crate::ops::traits::*;
use crate::utils::*;
use log::info;
use num_bigint::BigUint;
use num_traits::{zero, ToPrimitive};
impl ControlFlow for Evm {
    /// 跳转指令
    /// ```
    /// let excute_codes = "60040058";
    /// let bytes = hex::decode(excute_codes).unwrap();
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// println!("{:?}", evm_test.stack);
    /// ```
    fn jump(&mut self) {
        if self.stack.len() < 1 {
            panic!("stack underflow");
        }
        let destination = get_uint256(self.stack.pop());
        if self
            .valid_jumpdest
            .contains_key(&destination.to_usize().unwrap())
        {
            info!("jump to {:?}", &destination.to_usize().unwrap());
            self.pc = destination.to_usize().unwrap();
        } else {
            panic!("jump destination not exist")
        }
    }
    /// 条件跳转
    /// ```
    /// let excute_codes = "6001600657005B";
    /// let bytes = hex::decode(excute_codes).unwrap();
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// println!("{:?}", evm_test.stack);
    /// ```
    fn jumpi(&mut self) {
        if self.stack.len() < 2 {
            panic!("stack underflow");
        }
        let destination = get_uint256(self.stack.pop());
        let condition = get_uint256(self.stack.pop());
        if condition == BigUint::from(1u8) {
            if self
                .valid_jumpdest
                .contains_key(&destination.to_usize().unwrap())
            {
                info!("jump to {:?}", &destination.to_usize().unwrap());
                self.pc = destination.to_usize().unwrap();
            } else {
                panic!("jump destination not exist")
            }
        }
    }
    fn jumpdest(&mut self) {
        // self.valid_jumpdest.insert(self.pc, true);
        info!("jumpdest{:?}", self.valid_jumpdest);
    }
    /// 程序计算器指令
    /// ```
    /// let excute_codes = "6001600657005B58";
    /// let bytes = hex::decode(excute_codes).unwrap();
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// println!("{:?}", evm_test.stack);
    /// ```
    fn pc(&mut self) {
        self.stack.push(StackData::new(BigUint::from(self.pc - 1).to_bytes_be(), 0u8));
    }
}

#[test]
fn stop_test() {
    let excute_codes = "60040058";
    let bytes = hex::decode(excute_codes).unwrap();
    let mut evm_test = Evm::new(bytes);
    evm_test.run();
    println!("{:?}", evm_test.stack);
}

#[test]
fn jump_test() {
    let excute_codes = "600456005B";
    let bytes = hex::decode(excute_codes).unwrap();
    let mut evm_test = Evm::new(bytes);
    evm_test.run();
    println!("{:?}", evm_test.stack);
}

#[test]
fn jumpi_test() {
    let excute_codes = "6001600657005B";
    let bytes = hex::decode(excute_codes).unwrap();
    let mut evm_test = Evm::new(bytes);
    evm_test.run();
    println!("{:?}", evm_test.stack);
}

#[test]
fn pc_test() {
    let excute_codes = "6001600657005B58";
    let bytes = hex::decode(excute_codes).unwrap();
    let mut evm_test = Evm::new(bytes);
    evm_test.run();
    println!("{:?}", evm_test.stack);
}
