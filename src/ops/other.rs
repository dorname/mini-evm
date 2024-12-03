use std::f64::consts::E;

use crate::ops::traits::{CurrentBlockInfo, Other};
use crate::stack::StackData;
use crate::transaction::Transaction;
use crate::{evm, utils::*};
use crate::{evm::Evm, log_entry::LogEntry};
use log::{info, logger};
use num_bigint::BigUint;
use num_traits::{zero, ToPrimitive};
impl Other for Evm {
    /// sha3指令
    /// ```
    /// let excute_codes = "5F5F20";
    /// let bytes = hex::decode(excute_codes).unwrap();
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// println!("{:?}", evm_test.stack);
    /// ```
    fn sha3(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        fn get_data(memory: &Vec<u8>, offset: BigUint, size: BigUint) -> Vec<u8> {
            let start = offset.to_usize().unwrap();
            let end = (offset + size).to_usize().unwrap();
            memory[start..end].to_vec()
        }
        let offset = get_uint256(self.stack.pop());
        let size = get_uint256(self.stack.pop());
        let data = get_data(&self.memory, offset, size);
        let hash = keccak256(&data).to_vec();
        info!("sha3:{:?}", vec_to_hex_string(hash.clone()));
        self.stack.push(StackData::new(hash, 0u8));
    }
    /// log1-log4指令
    /// ```
    /// let excute_codes = "60aa6000526001601fa0";
    /// let bytes = hex::decode(excute_codes).unwrap();
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// println!("{:?}", evm_test.stack);
    /// ```
    fn log(&mut self, num_topics: usize) {
        if self.stack.len() < 2 + num_topics {
            panic!("Stack underflow");
        }
        let mem_offset = get_uint256(self.stack.pop());
        let length = get_uint256(self.stack.pop());
        let topics: Vec<BigUint> = (0..num_topics)
            .into_iter()
            .map(|_| get_uint256(self.stack.pop()))
            .collect();
        let data = self.memory
            [mem_offset.to_usize().unwrap()..(mem_offset + length).to_usize().unwrap()]
            .to_vec();
        let log_entry = LogEntry::init(self.txn.get_this_addr(), data, topics);
        self.logs.push(log_entry);
    }
    /// datacopy指令
    /// 将上一轮计算的结果，复制到内存上
    /// ```
    /// let excute_codes = "60a26000526001601ff3";
    /// let bytes = hex::decode(excute_codes).unwrap();
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// println!("{:?}", evm_test.return_data);
    /// println!("{:?}", evm_test.stack);
    /// println!("{:?}", vec_to_hex_string(evm_test.memory.clone()));
    /// let next_excute_codes = "6001600060003e";
    /// let next_bytes = hex::decode(next_excute_codes).unwrap();
    /// evm_test.next_codes(next_bytes);
    /// evm_test.run();
    /// println!("{:?}", evm_test.return_data);
    /// println!("{:?}", evm_test.stack);
    /// println!("{:?}", vec_to_hex_string(evm_test.memory.clone()));
    /// ```
    fn return_datacopy(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let mem_offset = get_uint256(self.stack.pop());
        let return_offset = get_uint256(self.stack.pop());
        let length = get_uint256(self.stack.pop());
        if (&return_offset + &length).to_usize().unwrap() > self.return_data.len() {
            panic!("Return data out of bounds");
        }
        if self.memory.len() < (&mem_offset + &length).to_usize().unwrap() {
            self.memory
                .resize((&mem_offset + &length).to_usize().unwrap(), 0u8);
        }
        self.memory
            [mem_offset.to_usize().unwrap()..(mem_offset + length.clone()).to_usize().unwrap()]
            .copy_from_slice(
                &self.return_data[return_offset.to_usize().unwrap()
                    ..(return_offset + length).to_usize().unwrap()],
            );
    }
    /// datasize指令
    /// 查看返回数据的长度
    /// ```
    /// let excute_codes = "61aaaa6000526002601ff33d";
    /// let bytes = hex::decode(excute_codes).unwrap();
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// println!("{:?}", evm_test.stack);
    /// ```
    fn return_datasize(&mut self) {
        self.stack
            .push(StackData::new(self.return_data.len().to_be_bytes().to_vec(), 0u8));
    }
    /// return指令
    /// 返回数据
    /// ```
    /// let excute_codes = "60a26000526001601ff3";
    /// let bytes = hex::decode(excute_codes).unwrap();
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// println!("{:?}", evm_test.stack);
    /// ```
    fn return_fn(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let mem_offset = get_uint256(self.stack.pop());
        let length = get_uint256(self.stack.pop());
        info!("mem_offset:{}", &mem_offset.to_usize().unwrap());
        info!("length:{}", &length.to_usize().unwrap());
        if self.memory.len() < (&mem_offset + &length).to_usize().unwrap() {
            self.memory
                .resize((&mem_offset + &length).to_usize().unwrap(), 0u8);
        }
        self.return_data = self.memory[mem_offset.clone().to_usize().unwrap()
            ..(mem_offset.clone() + length).to_usize().unwrap()]
            .to_vec();
        self.memory = self.memory[0..mem_offset.to_usize().unwrap()].to_vec();
    }
    /// revert指令
    /// 异常情况可以通过该指令将交易回滚
    /// ```
    /// let excute_codes = "60a26000526001601ff3";
    /// let bytes = hex::decode(excute_codes).unwrap();
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// println!("{:?}", evm_test.stack);
    /// ```
    fn revert(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let mem_offset = get_uint256(self.stack.pop());
        let length = get_uint256(self.stack.pop());

        let total_len = (&mem_offset + &length).to_usize().unwrap();
        //如果内存长度不足，拓展内存
        if self.memory.len() < total_len.clone() {
            self.memory.resize(total_len.clone(), 0u8);
        }

        self.return_data = self.memory[mem_offset.to_usize().unwrap()..total_len].to_vec();
        self.success = false;
    }
    fn invalid(&mut self) {
        self.success = false;
    }
    fn gas(&mut self) {
        self.stack
            .push(StackData::new((self.txn.get_gas_limit() - self.gas_used.clone()).to_bytes_be(), 0u8));
    }
}

#[test]
fn test_sha3() {
    let excute_codes = "5F5F20";
    let bytes = hex::decode(excute_codes).unwrap();
    // let bytes = vec![0x61, 0xff,0x00];
    let mut evm_test = Evm::new(bytes);
    evm_test.run();
    println!("{:?}", evm_test.stack);
}

#[test]
fn test_log() {
    let excute_codes = "60aa6000526001601fa0";
    let bytes = hex::decode(excute_codes).unwrap();
    let mut evm_test = Evm::new(bytes);
    evm_test.run();
    println!("{:?}", evm_test.logs);
}

#[test]
fn test_return() {
    let excute_codes = "60a26000526001601ff3";
    let bytes = hex::decode(excute_codes).unwrap();
    let mut evm_test = Evm::new(bytes);
    evm_test.run();
    println!("{:?}", evm_test.return_data);
}

#[test]
fn test_returnsize() {
    let excute_codes = "61aaaa6000526002601ff33d";
    let bytes = hex::decode(excute_codes).unwrap();
    let mut evm_test = Evm::new(bytes);
    evm_test.run();
    println!("{:?}", evm_test.return_data);
    println!("{:?}", evm_test.stack);
}

#[test]
fn test_returncopy() {
    let excute_codes = "60a26000526001601ff3";
    let bytes = hex::decode(excute_codes).unwrap();
    let mut evm_test = Evm::new(bytes);
    evm_test.run();
    println!("{:?}", evm_test.return_data);
    println!("{:?}", evm_test.stack);
    println!("{:?}", vec_to_hex_string(evm_test.memory.clone()));
    let next_excute_codes = "6001600060003e";
    let next_bytes = hex::decode(next_excute_codes).unwrap();
    evm_test.next_codes(next_bytes);
    evm_test.run();
    println!("{:?}", evm_test.return_data);
    println!("{:?}", evm_test.stack);
    println!("{:?}", vec_to_hex_string(evm_test.memory.clone()));
}

#[test]
fn test_revert() {
    let excute_codes = "60aa6000526001601ffd";
    let bytes = hex::decode(excute_codes).unwrap();
    let mut evm_test = Evm::new(bytes);
    evm_test.run();
    println!("{:?}", vec_to_hex_string(evm_test.return_data));
}

#[test]
fn test_gas() {
    let excute_codes = "60205a";
    let bytes = hex::decode(excute_codes).unwrap();
    let txn = Transaction::init(
        zero(),
        BigUint::from(1u8),
        BigUint::from(100u8),
        "".to_string(),
        BigUint::from(10u8),
        "".to_string(),
        "0x1000000000000000000000000000000000000c42".to_string(),
        "0x1000000000000000000000000000000000000c42".to_string(),
        "0x1000000000000000000000000000000000000c42".to_string(),
        zero(),
        zero(),
        zero(),
    );
    evm::init_log();
    let mut evm_test = Evm::init_evm(bytes, txn);
    evm_test.run();
    println!("{:?}", evm_test.stack);
    println!(
        "gaslimit={:?},gasused={:?}",
        evm_test.txn.get_gas_limit(),
        evm_test.gas_used
    );
}
