use std::collections::HashMap;

use crate::fake_db::{Account, AccountDb};
use crate::ops::traits::*;
use crate::stack::StackData;
use crate::{evm, utils::*};
use crate::{evm::Evm, transaction::Transaction};
use log::*;
use num_bigint::{BigUint, ToBigUint};
use num_traits::{zero, ToPrimitive, Zero};

use super::account;
impl Contract for Evm {
    fn create(&mut self) {
        if self.stack.len() < 3 {
            panic!("Stack underflow");
        }
        //获取堆栈数据
        let value = get_uint256(self.stack.pop());
        let mem_offset = get_uint256(self.stack.pop());
        let lenght = get_uint256(self.stack.pop());

        //拓展内存
        // 获取内存需要的长度
        let len = (&mem_offset + &lenght).to_usize().unwrap();
        if self.memory.len() < len {
            self.memory.resize(len, 0u8);
        }

        //获取初始代码
        let init_code = self.memory[mem_offset.to_usize().unwrap()..len].to_vec();

        // 检查创建者的余额是否足够
        let mut binding = get_account_db();
        info!("创建者地址{}", self.txn.get_this_addr());
        let creator_account = binding.get_account_mut(self.txn.get_this_addr());
        if creator_account.balance < value {
            panic!("Insufficient balance to create contract!");
        }

        // 扣除创建者指定的金额
        creator_account.balance -= value.clone();

        // 生成新的合约地址
        let mut creator_nonce = creator_account.nonce.to_bytes_be();
        let mut this_address = hex::decode(self.txn.get_this_addr().split_off(2)).unwrap();
        this_address.append(&mut creator_nonce);
        let new_contract_address_bytes = keccak256(&this_address);
        let new_contract_address = format!(
            "0x{}",
            hex::encode(&new_contract_address_bytes[(new_contract_address_bytes.len() - 20)..])
        );
        info!("新合约地址{}", new_contract_address);
        // 构建上下文
        let txn = Transaction::init(
            zero(),
            self.txn.get_gas_limit().clone(),
            self.txn.get_gas_limit().clone(),
            new_contract_address.clone(),
            value.clone(),
            hex::encode(init_code.clone()),
            self.txn.get_this_addr(),
            self.txn.get_origin(),
            new_contract_address.clone(),
            zero(),
            zero(),
            zero(),
        );

        // 创建并运行新的EVM实例
        let mut evm_sub = Evm::init_evm(init_code, txn);
        evm_sub.run();

        // 如果evm_sub实例返回错误，栈返回0,表示合约创建失败
        if !evm_sub.success {
            self.stack
                .push(StackData::new(0u8.to_be_bytes().to_vec(), 0u8));
        }

        //更新创建者的nouce
        creator_account.nonce += BigUint::from(1u8);

        // 存储合约的状态
        binding.insert(
            new_contract_address,
            Account::new(value, zero(), evm_sub.storage, evm_sub.return_data),
        );

        // 新创建合约的地址入栈
        self.stack.push(StackData::new(this_address, 0u8));
    }
    fn create2(&mut self) {
        if self.stack.len() < 4 {
            panic!("Stack underflow");
        }
        let value = get_uint256(self.stack.pop());
        let mem_offset = get_uint256(self.stack.pop());
        let lenght = get_uint256(self.stack.pop());
        let salt = get_uint256(self.stack.pop());

        //拓展内存
        // 获取内存需要的长度
        let len = (&mem_offset + &lenght).to_usize().unwrap();
        if self.memory.len() < len {
            self.memory.resize(len, 0u8);
        }

        //获取初始代码
        let init_code = self.memory[mem_offset.to_usize().unwrap()..len].to_vec();

        // 检查创建者的余额是否足够
        let mut binding = get_account_db();
        info!("创建者地址{}", self.txn.get_this_addr());
        let creator_account = binding.get_account_mut(self.txn.get_this_addr());
        if creator_account.balance < value {
            panic!("Insufficient balance to create contract!");
        }

        // 扣除创建者指定的金额
        creator_account.balance -= value.clone();

        // 生成新的合约地址
        let init_code_hash = hex::encode(keccak256(&init_code));
        let data_to_hash = "ff".to_string()
            + &self.txn.get_this_addr().split_off(2)
            + &hex::encode(salt.to_bytes_be())
            + &init_code_hash;
        let new_contract_address_bytes = keccak256(&hex::decode(data_to_hash).unwrap());
        let new_contract_address = format!(
            "0x{}",
            hex::encode(&new_contract_address_bytes[(new_contract_address_bytes.len() - 20)..])
        );
        info!("新合约地址{}", new_contract_address);

        // 构建上下文
        let txn = Transaction::init(
            zero(),
            self.txn.get_gas_limit().clone(),
            self.txn.get_gas_limit().clone(),
            new_contract_address.clone(),
            value.clone(),
            hex::encode(init_code.clone()),
            self.txn.get_this_addr(),
            self.txn.get_origin(),
            new_contract_address.clone(),
            zero(),
            zero(),
            zero(),
        );

        let mut evm_create2 = Evm::init_evm(init_code, txn);
        evm_create2.run();

        // 如果evm_sub实例返回错误，栈返回0,表示合约创建失败
        if !evm_create2.success {
            self.stack
                .push(StackData::new(0u8.to_be_bytes().to_vec(), 0u8));
        }

        //更新创建者的nouce
        creator_account.nonce += BigUint::from(1u8);

        // 存储合约的状态
        binding.insert(
            new_contract_address,
            Account::new(value, zero(), evm_create2.storage, evm_create2.return_data),
        );

        // 新创建合约的地址入栈
        self.stack
            .push(StackData::new(new_contract_address_bytes.to_vec(), 0u8));
    }
    fn selfdestruct(&mut self) {
        if self.stack.len() < 1 {
            panic!("Stack underflow");
        }

        // 弹出接收ETH的指定地址
        let raw_recipient = get_uint256(self.stack.pop());
        let recipient = "0x".to_string() + &hex::encode(raw_recipient.to_bytes_be().to_vec());

        let mut db = get_account_db_2();
        if !db.contains(recipient.clone()) {
            db.insert(
                recipient.clone(),
                Account::new(zero(), zero(), HashMap::new(), vec![]),
            );
        }
        let balance = db.get_account(self.txn.get_this_addr()).balance.clone();
        let account = db.get_account_mut(recipient);
        account.balance += balance;

        db.remove(self.txn.get_this_addr());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evm::*;
    use once_cell::sync::Lazy;
    #[test]
    fn test_create() {
        Lazy::force(&INIT_LOG);
        // let excute_codes = "5f5f6009f0";
        let excute_codes = "6c63ffffffff6000526004601cf3600052600d60136000f0";
        let bytes = hex::decode(excute_codes).unwrap();
        let txn = Transaction::init(
            zero(),
            zero(),
            BigUint::from(1000000u64),
            "0x9bbfed6889322e016e0a02ee459d306fc19545d8".to_string(),
            BigUint::from(10u8),
            "".to_string(),
            "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045".to_string(),
            "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045".to_string(),
            "0x9bbfed6889322e016e0a02ee459d306fc19545d8".to_string(),
            zero(),
            zero(),
            zero(),
        );
        // evm::init_log();
        let mut evm_test = Evm::init_evm(bytes, txn);
        evm_test.run();
        println!("{:?}", get_account_db());
    }

    #[test]
    fn test_create2() {
        Lazy::force(&INIT_LOG);
        // let excute_codes = "5f5f5f6009f5";
        let excute_codes = "6c63ffffffff6000526004601cf36000526004600d60136000f5";
        let bytes = hex::decode(excute_codes).unwrap();
        let txn = Transaction::init(
            zero(),
            zero(),
            zero(),
            "0x9bbfed6889322e016e0a02ee459d306fc19545d8".to_string(),
            BigUint::from(100000u64),
            "".to_string(),
            "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045".to_string(),
            "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045".to_string(),
            "0x9bbfed6889322e016e0a02ee459d306fc19545d8".to_string(),
            zero(),
            zero(),
            zero(),
        );
        // evm::init_log();
        let mut evm_test = Evm::init_evm(bytes, txn);
        evm_test.run();
        println!("{:?}", get_account_db());
    }

    #[test]
    fn test_selfdestruct() {
        Lazy::force(&INIT_LOG);
        let excute_codes = "6020ff";
        let bytes = hex::decode(excute_codes).unwrap();
        let txn = Transaction::init(
            zero(),
            zero(),
            zero(),
            "".to_string(),
            BigUint::from(100000u64),
            "".to_string(),
            "0x1000000000000000000000000000000000000c42".to_string(),
            "0x1000000000000000000000000000000000000c42".to_string(),
            "0x1000000000000000000000000000000000000c42".to_string(),
            zero(),
            zero(),
            zero(),
        );
        // evm::init_log();
        println!("销毁前{:?}", get_account_db_2());
        let mut evm_test = Evm::init_evm(bytes, txn);
        evm_test.run();
        println!("销毁后{:?}", get_account_db_2());
    }
}
