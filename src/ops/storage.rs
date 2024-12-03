use crate::{evm::Evm, stack::StackData};
use crate::log_utils::*;
use crate::ops::traits::*;
use crate::utils::*;
use num_bigint::BigUint;
use num_traits::{zero, ToPrimitive};

impl Storage for Evm {
    /// 存储读指令
    /// ```
    /// use crate::evm::Evm;
    /// let excute_codes = "60f1600255600254";
    /// let bytes = hex::decode(excute_codes).unwrap();
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn sload(&mut self) {
        if self.stack.len() < 1 {
            panic!("stack underflow");
        }
        let key = get_uint256(self.stack.pop());
        let info_err = format!("读取键值为{:?}的存储值", key);
        let mut logger = LogTemplate::new_cal("SLOAD".to_owned(), info_err.to_owned());
        logger.log_cal();
        if self.storage.contains_key(&key) {
            let value = self.storage.get(&key).unwrap();
            logger.set_result(get_uint256(value.clone()));
            self.stack.push(StackData::new(value.0.to_bytes_be(),value.1));
        } else {
            logger.set_result(zero());
            self.stack.push(StackData::new(BigUint::from(0u8).to_bytes_be(), 0));
        }
        logger.log_store_val();
        logger.log_real_val();
    }

    /// 存储读指令
    /// ```
    /// use crate::evm::Evm;
    /// let excute_codes = "60f1600255";
    /// let bytes = hex::decode(excute_codes).unwrap();
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn sstore(&mut self) {
        if  self.stack.len() < 2 {
            panic!("stack underflow");
        }
        let key = self.stack.pop();
        let value = self.stack.pop();
        let mut logger = LogTemplate::new_two_cal(
            "SSTORE".to_owned(),
            "sstore".to_owned(),
            key.clone(),
            value.clone(),
        );
        self.storage.insert(get_uint256(key),value);
        logger.log_storage_cal();
        logger.log_storage_store_val(self.storage.clone());
    }
}

#[test]
fn sstore_test() {
    let excute_codes = "60f1600255";
    let bytes = hex::decode(excute_codes).unwrap();
    let mut evm_test = Evm::new(bytes);
    evm_test.run();
}

#[test]
fn sload_test() {
    let excute_codes = "60f1600255600254";
    let bytes = hex::decode(excute_codes).unwrap();
    let mut evm_test = Evm::new(bytes);
    evm_test.run();
}