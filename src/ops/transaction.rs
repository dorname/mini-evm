use crate::evm::Evm;
use crate::ops::traits::*;
use crate::stack::StackData;
use crate::utils::*;
use num_bigint::BigUint;
use num_traits::ToPrimitive;
/// TODO 交易指令后期也需要根据实际区块链实现调整
impl TransactionTraits for Evm {
    fn address(&mut self) {
        self.stack.push(StackData::new(
            hex::decode(&self.txn.get_this_addr()[2..]).unwrap(),
            0u8,
        ));
    }
    fn calldataload(&mut self) {
        if self.stack.len() < 1 {
            panic!("Stack underflow");
        }
        let offset = get_uint256(self.stack.pop());
        let data = hex::decode(&self.txn.get_data()[2..]).unwrap();
        let mut result_data = data[offset.to_usize().unwrap()..].to_vec();
        if result_data.len() < 32 {
            result_data.resize(32, 0);
        }
        self.stack.push(StackData::new(result_data, 0u8));
    }

    fn calldatasize(&mut self) {
        let size = (self.txn.get_data().len() - 2) / 2;
        self.stack.push(StackData::new(size.to_be_bytes().to_vec(), 0u8));
    }

    fn caller(&mut self) {
        self.stack.push(StackData::new(
            hex::decode(&self.txn.get_caller()[2..]).unwrap(),
            0u8,
        ));
    }
    fn callvalue(&mut self) {
        self.stack.push(StackData::new(self.txn.get_value().to_bytes_be(), 0u8));
    }
    fn codecopy(&mut self) {
        if self.stack.len() < 3 {
            panic!("Stack underflow");
        }
        let mem_offset = get_uint256(self.stack.pop());
        let code_offset = get_uint256(self.stack.pop());
        let length = get_uint256(self.stack.pop());

        if self.memory.len() < (mem_offset.clone() + length.clone()).to_usize().unwrap() {
            self.memory.resize(
                (mem_offset.clone() + length.clone()).to_usize().unwrap() - self.memory.len(),
                0,
            );
        }

        let addr = self.txn.get_this_addr();
        let codedata = get_account_db().get_account(addr).code.clone();
        for i in 0..length.to_usize().unwrap() {
            if code_offset.to_usize().unwrap() + i < codedata.len() {
                self.memory[(code_offset.clone() + BigUint::from(i)).to_usize().unwrap()] =
                    codedata[code_offset.to_usize().unwrap() + i];
            }
        }
    }
    fn codesize(&mut self) {
        let addr = self.txn.get_this_addr();
        let result = get_account_db().get_account(addr).code.clone();
        self.stack.push(StackData::new(result.len().to_be_bytes().to_vec(), 0u8));
    }
    fn gasprice(&mut self) {
        self.stack.push(StackData::new(self.txn.get_gas_price().to_bytes_be(), 0u8));
    }
    fn origin(&mut self) {
        self.stack.push(StackData::new(
            hex::decode(&self.txn.get_origin()[2..]).unwrap(),
            0u8,
        ));
    }
    fn calldatacopy(&mut self) {
        if self.stack.len() < 3 {
            panic!("Stack underflow");
        }
        let mem_offset = get_uint256(self.stack.pop());
        let calldata_offset = get_uint256(self.stack.pop());
        let length = get_uint256(self.stack.pop());

        if self.memory.len() < (mem_offset.clone() + length.clone()).to_usize().unwrap() {
            self.memory.resize(
                (mem_offset.clone() + length.clone()).to_usize().unwrap() - self.memory.len(),
                0,
            );
        }

        let calldata = hex::decode(&self.txn.get_data()[2..]).unwrap();
        for i in 0..length.to_usize().unwrap() {
            if calldata_offset.to_usize().unwrap() + i < calldata.len() {
                self.memory[(calldata_offset.clone() + BigUint::from(i))
                    .to_usize()
                    .unwrap()] = calldata[calldata_offset.to_usize().unwrap() + i];
            }
        }
    }
}
