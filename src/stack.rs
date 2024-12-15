use core::fmt;
use std::str;

use num_bigint::BigUint;

// 实现一个存储结构清晰的evm栈结构，存储时尽量不使用第三方库的类型比如BigUint进行存储
// 但计算时可以借用BigUint进行计算，避免复杂的位运算
#[derive(Debug,Clone,PartialEq)]
pub struct Stack(pub Vec<StackData>);
impl Stack {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn push(&mut self, data: StackData) {
        if self.len() >= 1024 {
            panic!("stack overflow");
        }
        self.0.push(data);
    }
    pub fn pop(&mut self) -> (BigUint, u8) {
        let result = self.0.pop().unwrap();
        (BigUint::from_bytes_be(&result.data),result.sign)
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn get(&self, index: usize) -> StackData {
        let len = self.0.len();
        self.0[len-index].clone()
    }

    pub fn swap(&mut self,top:usize,index:usize){
        self.0.swap(top-1,top-index-1);
    }
}
#[derive(Clone,PartialEq)]
pub struct StackData {
    // 栈宽存储的位数256位，最大深度1024
    // 16*16 = 256 使用u16，数组长度为16。
    // 8 *32 = 256 使用u8，数组长度为8，
    // 0xff 一个十六进制数f,只有4位。 ff 1111 1111 是8位
    pub data: Vec<u8>,
    // 符号位，0表示正数，1表示负数
    pub sign: u8,
}
impl StackData {
    pub fn new(bytes: Vec<u8>,sign:u8) -> Self {
        if bytes.len() > 32 {
            panic!("stack overflow");
        }
        let mut data = [0u8; 32].to_vec();
        // bytes 长度小于32时，则前补0填充
        // 补0策略1：data低字节不断插入bytes的元素，截取长度32-len..32
        if bytes.len() <= 32 {
            data = [[0u8; 32].to_vec(), bytes.clone()].concat();
        }
        println!("data: {:?}", data[bytes.len()..].to_vec());
        Self {
            data: data[bytes.len()..].to_vec(),
            sign: sign,
        }
    }
}
impl fmt::Debug for StackData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StackArch")
        .field("data", &hex::encode(&self.data))
        .field("sign", &self.sign)
        .finish()
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::BigUint;
    use super::*;
    #[test]
    fn test_stack_data() {
        //0xff 1111 1111 255
        let stack_data: StackData = StackData::new(
            ((BigUint::from(1u8) << 256usize) - BigUint::from(1u8))
                .to_bytes_be()
                .to_vec(),
                0u8
        );
        println!("{:?}", stack_data);
        println!("data len : {:?}", stack_data.data.len());
        println!("data: {:?}", hex::encode(stack_data.data));
    }
}
