use crate::log_utils::*;
use crate::ops::traits::*;
use crate::utils::*;
use crate::{evm::Evm, stack::StackData};
use num_bigint::BigUint;
use num_traits::ToPrimitive;

impl Memory for Evm {
    /// 内存读指令
    /// ```
    /// use evm_lab::evm::Evm;
    /// use crate::evm::Evm;
    /// use crate::log_utils::*;
    /// use crate::ops::traits::*;
    /// use num_bigint::BigUint;
    /// use num_traits::{ToPrimitive};
    /// let excute_codes = "61ff02600152600151";
    /// let bytes = hex::decode(excute_codes).unwrap();
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn mload(&mut self) {
        if self.stack.len() < 1 {
            panic!("Stack underflow");
        }
        let offset = self.stack.pop().0;
        let info_err = format!("读取偏移位置为{:?}的内存", offset);
        let mut logger = LogTemplate::new_cal("MLOAD".to_owned(), info_err.to_owned());
        logger.log_cal();
       
        // 如果内存长度不够，自动扩展
        if self.memory.len() < (offset.clone() + BigUint::from(32u8)).to_usize().unwrap() {
            let resize = match (offset.clone() + BigUint::from(32u8)).to_usize().unwrap() - self.memory.len(){
                x if x > 0 => {
                    (x-1)/32 + 1
                }
                _ => 0,
            };
            self.memory.resize(
                resize*32+ self.memory.len(),
                0,
            ); // 将不足的部分填充为
        }
        let value = self.memory[offset.to_usize().unwrap()..(offset.clone() + BigUint::from(32u8)).to_usize().unwrap()].to_vec();
        logger.set_result(BigUint::from_bytes_be(&value));
        logger.log_store_val();
        logger.log_real_val();
        self.stack.push(StackData::new(value, 0u8));
    }
    /// 内存大小读指令
    /// ```
    /// use evm_lab::evm::Evm;
    /// use crate::evm::Evm;
    /// use crate::log_utils::*;
    /// use crate::ops::traits::*;
    /// use num_bigint::BigUint;
    /// use num_traits::{ToPrimitive};
    /// let excute_codes = "61ff0260015359";
    /// let bytes = hex::decode(excute_codes).unwrap();
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn msize(&mut self) {
        let mut logger = LogTemplate::new_cal("MSIZE".to_owned(), "获取当前内存大小".to_owned());
        logger.log_cal();
        logger.set_result(BigUint::from(self.memory.len()));
        self.stack
            .push(StackData::new(self.memory.len().to_be_bytes().to_vec(), 0));
        logger.log_store_val();
        logger.log_real_val();
    }

    /// 内存写指令
    /// 一个十六进制数代表4位
    /// ```
    /// use crate::evm::Evm;
    /// let excute_codes = "61ff02600152";
    /// let bytes = hex::decode(excute_codes).unwrap();
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn mstore(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_offset = self.stack.pop();
        let unit_value = self.stack.pop();
        let mut logger = LogTemplate::new_two_cal(
            "MSTORE".to_owned(),
            "mstore".to_owned(),
            unit_value.clone(),
            unit_offset.clone(),
        );
        logger.log_two_cal();
        let offset = unit_offset.0;
        println!(
            "{},{}",
            offset.clone(),
            BigUint::from(64u8).to_usize().unwrap()
        );
        let value = unit_value.0;
        // 如果内存长度不够，自动扩展
        if self.memory.len() < (offset.clone() + BigUint::from(32u8)).to_usize().unwrap() {
            self.memory.resize(
                (offset.clone() + BigUint::from(32u8)).to_usize().unwrap(),
                0,
            ); // 将不足的部分填充为
        }
        let mut val_bytes = value.to_bytes_be();
        if val_bytes.len() < 32 {
            // 如果字节长度不足 32 字节，前面填充 0
            let padding = vec![0u8; 32 - val_bytes.len()];
            val_bytes = [padding, val_bytes].concat();
        }
        // 将 32 字节数据写入内存中的偏移位置
        for (i, val_byte) in val_bytes.iter().enumerate() {
            self.memory[(offset.clone() + BigUint::from(i)).to_usize().unwrap()] = *val_byte;
        }
        self.fill_memory();
        //因为一个十六进制数代表4位所以打印的时候把长度设置成64位长度
        logger.log_memory_store_val(self.memory.clone());
    }

    /// 内存单字节写指令
    /// ```
    /// use crate::evm::Evm;
    /// let excute_codes = "61ff02600153";
    /// let bytes = hex::decode(excute_codes).unwrap();
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn mstore8(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_offset = self.stack.pop();
        let unit_value = self.stack.pop();
        let mut logger = LogTemplate::new_two_cal(
            "MSTORE8".to_owned(),
            "mstore8".to_owned(),
            unit_value.clone(),
            unit_offset.clone(),
        );
        logger.log_two_cal();
        let offset = unit_offset.0;
        println!(
            "{},{}",
            offset.clone(),
            BigUint::from(64u8).to_usize().unwrap()
        );
        let value = unit_value.0;
        // 如果内存长度不够，自动扩展
        if self.memory.len() < offset.clone().to_usize().unwrap() {
            self.memory
                .resize(offset.clone().to_usize().unwrap() + 1, 0); // 将不足的部分填充为
        }
        let mask = (BigUint::from(1u8) << 3) - BigUint::from(1u8);
        let low_val: BigUint = value & mask;
        self.memory[offset.clone().to_usize().unwrap()] = low_val.to_u8().unwrap();
        self.fill_memory();
        //因为一个十六进制数代表4位所以打印的时候把长度设置成64位长度
        logger.log_memory_store_val(self.memory.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evm::*;
    use once_cell::sync::Lazy;
    #[test]
    fn mstore_test() {
        Lazy::force(&INIT_LOG);
        // let excute_codes = "60ff600152";
        let excute_codes = "61ff02600152";
        // let excute_codes = "61ff02601452";
        let bytes = hex::decode(excute_codes).unwrap();
        let mut evm_test = Evm::new(bytes);
        evm_test.run();
        assert_eq!(
            "00000000000000000000000000000000000000000000000000000000000000ff0200000000000000000000000000000000000000000000000000000000000000".to_owned(),
            vec_to_hex_string(evm_test.memory));
    }

    #[test]
    fn mstore8_test() {
        Lazy::force(&INIT_LOG);
        let excute_codes = "61ff02600153";
        // let excute_codes = "6002602053";
        let bytes = hex::decode(excute_codes).unwrap();
        let mut evm_test = Evm::new(bytes);
        evm_test.run();
        assert_eq!(
            "0002000000000000000000000000000000000000000000000000000000000000",
            vec_to_hex_string(evm_test.memory)
        );
    }

    #[test]
    fn msize_test() {
        Lazy::force(&INIT_LOG);
        let excute_codes = "61ff0260015359";
        // let excute_codes = "61ff0260015259";
        let bytes = hex::decode(excute_codes).unwrap();
        let mut evm_test = Evm::new(bytes);
        evm_test.run();
        // 测试内存
        assert_eq!(
            "0002000000000000000000000000000000000000000000000000000000000000",
            vec_to_hex_string(evm_test.memory)
        );
        // 测试stack存储,evm.codes的官网会把前面多余的0去掉不展示，这里展示完整的stack内容，因为补0是EVM真实的操作
        assert_eq!(
            "0000000000000000000000000000000000000000000000000000000000000020",
            hex::encode(evm_test.stack.get(evm_test.stack.len()).data)
        );
    }

    #[test]
    fn mload_test() {
        Lazy::force(&INIT_LOG);
        let excute_codes = "61ff02600153600151";
        let bytes = hex::decode(excute_codes).unwrap();
        let mut evm_test = Evm::new(bytes);
        evm_test.run();
        // println!("{:?}", evm_test.stack.0.get(0).unwrap().data.to_vec());
        // 测试内存 这里evm.codes的官网会多展示64位而且都是0，暂时不知道为什么
        assert_eq!(
            "00020000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
            vec_to_hex_string(evm_test.memory)
        );
        // 测试stack存储,evm.codes的官网会把前面多余的0去掉不展示，这里展示完整的stack内容，因为补0是EVM真实的操作
        assert_eq!(
            "0200000000000000000000000000000000000000000000000000000000000000",
            vec_to_hex_string(evm_test.stack.0.get(0).unwrap().data.to_vec())
        );
    }
}
