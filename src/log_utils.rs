use std::collections::{hash_map, HashMap};

use crate::{ops::memory, utils::*};
use log::*;
use num_bigint::BigUint;
pub struct LogTemplate {
    //操作符名称
    op_name: String,
    //获取运算符符号
    op: String,
    //数值a
    unit_a: (BigUint, u8),
    //数值b
    unit_b: (BigUint, u8),
    //数值c
    unit_c: (BigUint, u8),
    //计算结果
    result: BigUint,
    //结果是否为负数
    is_negative: u8,
}
impl LogTemplate {
    pub fn log_cal(&self) {
        let a = get_uint256(self.unit_a.clone());
        info!("{}计算过程:({})", self.op_name, self.op);
    }

    pub fn log_two_cal(&self) {
        let a = get_uint256(self.unit_a.clone());
        let b = get_uint256(self.unit_b.clone());
        let sign_a = if self.unit_a.1 != 0 { "-" } else { "" };
        let sign_b = if self.unit_b.1 != 0 { "-" } else { "" };
        info!(
            "{}计算过程:({}{:?}{}{}{:?})",
            self.op_name, sign_a, a, self.op, sign_b, b
        );
    }
    
    pub fn log_storage_cal(&self) {
        let a = get_uint256(self.unit_a.clone());
        let b = get_uint256(self.unit_b.clone());
        let sign_a = if self.unit_a.1 != 0 { "-" } else { "" };
        let sign_b = if self.unit_b.1 != 0 { "-" } else { "" };
        info!(
            "{}插入的键值:({}{:?},{}{:?})",
            self.op_name, sign_a, a, sign_b, b
        );
    }


    pub fn log_three_cal(&self) {
        let a = get_uint256(self.unit_a.clone());
        let b = get_uint256(self.unit_b.clone());
        let c = get_uint256(self.unit_c.clone());
        let sign_a = if self.unit_a.1 != 0 { "-" } else { "" };
        let sign_b = if self.unit_b.1 != 0 { "-" } else { "" };
        let sign_c = if self.unit_c.1 != 0 { "-" } else { "" };
        info!(
            "{}计算过程:({}{:?}{}{}{:?}%{}{})",
            self.op_name, sign_a, a, self.op, sign_b, b, sign_c, c
        );
    }

    pub fn log_store_val(&self) {
        info!(
            "{}的存储值:{:?}",
            self.op_name,
            vec_to_hex_string(self.result.to_bytes_be())
        );
    }
    
    pub fn log_storage_store_val(&self,hash_map:HashMap<BigUint, (BigUint, u8)>) {
        info!("{}的存储值:{:?}", self.op_name, hash_map);
    }
    
    pub fn log_memory_store_val(&self, memory: Vec<u8>) {
        info!("{}的存储值:{:?}", self.op_name, vec_to_hex_string(memory));
    }

    pub fn log_real_val(&self) {
        let sign_result = if self.is_negative == 1 { "-" } else { "" };
        info!(
            "{}的真实值:{}{:?}",
            self.op_name,
            sign_result,
            get_uint256((self.result.clone(), self.is_negative))
        );
    }

    pub fn set_result(&mut self, result: BigUint) {
        self.result = result;
    }

    pub fn set_is_negative(&mut self, is_negative: u8) {
        self.is_negative = is_negative;
    }

    pub fn new_cal(op_name: String, op: String) -> Self {
        Self {
            op_name,
            op,
            unit_a: (BigUint::from(0u8), 0),
            unit_b: (BigUint::from(0u8), 0),
            unit_c: (BigUint::from(0u8), 0),
            result: BigUint::from(0u8),
            is_negative: 0u8,
        }
    }

    pub fn new_two_cal(
        op_name: String,
        op: String,
        unit_a: (BigUint, u8),
        unit_b: (BigUint, u8),
    ) -> Self {
        Self {
            op_name,
            op,
            unit_a,
            unit_b,
            unit_c: (BigUint::from(0u8), 0),
            result: BigUint::from(0u8),
            is_negative: 0u8,
        }
    }

    pub fn new_three_cal(
        op_name: String,
        op: String,
        unit_a: (BigUint, u8),
        unit_b: (BigUint, u8),
        unit_c: (BigUint, u8),
    ) -> Self {
        Self {
            op_name,
            op,
            unit_a,
            unit_b,
            unit_c,
            result: BigUint::from(0u8),
            is_negative: 0u8,
        }
    }
}
