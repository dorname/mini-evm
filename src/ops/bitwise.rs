use crate::log_utils::*;
use crate::ops::traits::*;
use crate::utils::*;
use crate::{evm::Evm, stack::StackData};
use num_bigint::BigUint;
use num_traits::{zero, One, ToPrimitive};

impl Bitwise for Evm {
    /// 与运算
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x08, 0x60, 0x04, 0x16];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn and(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_a = self.stack.pop();
        let unit_b = self.stack.pop();
        let mut logger = LogTemplate::new_two_cal(
            "AND".to_owned(),
            "&".to_owned(),
            unit_a.clone(),
            unit_b.clone(),
        );
        logger.log_two_cal();
        let a = unit_a.0;
        let b = unit_b.0;
        let result = a & b;
        logger.set_result(result.clone());
        logger.set_is_negative(0u8);
        logger.log_store_val();
        logger.log_real_val();
        self.stack.push(StackData::new(result.to_bytes_be(), 0u8));
    }
    /// 或运算
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x08, 0x60, 0x04, 0x17];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn or(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_a = self.stack.pop();
        let unit_b = self.stack.pop();
        let mut logger = LogTemplate::new_two_cal(
            "OR".to_owned(),
            "|".to_owned(),
            unit_a.clone(),
            unit_b.clone(),
        );
        logger.log_two_cal();
        let a = unit_a.0;
        let b = unit_b.0;
        let result = a | b;
        logger.set_result(result.clone());
        logger.set_is_negative(0u8);
        logger.log_store_val();
        logger.log_real_val();
        self.stack.push(StackData::new(result.to_bytes_be(), 0u8));
    }

    /// 异或运算
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x08, 0x60, 0x04, 0x18];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn xor(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_a = self.stack.pop();
        let unit_b = self.stack.pop();
        let mut logger = LogTemplate::new_two_cal(
            "XOR".to_owned(),
            "^".to_owned(),
            unit_a.clone(),
            unit_b.clone(),
        );
        logger.log_two_cal();
        let a = unit_a.0;
        let b = unit_b.0;
        let result = a ^ b;
        logger.set_result(result.clone());
        logger.set_is_negative(0u8);
        logger.log_store_val();
        logger.log_real_val();
        self.stack.push(StackData::new(result.to_bytes_be(), 0u8));
    }

    /// 非运算
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x08, 0x19];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn not(&mut self) {
        if self.stack.len() < 1 {
            panic!("Stack underflow");
        }
        let unit_a = self.stack.pop();
        let mut logger = LogTemplate::new_two_cal(
            "NOT".to_owned(),
            "!=".to_owned(),
            unit_a.clone(),
            unit_a.clone(),
        );
        logger.log_two_cal();
        let a = unit_a.0;
        let sign_a = if unit_a.1 == 1u8 { 0u8 } else { 1u8 };
        let result: BigUint = (BigUint::one() << 256) - a;
        logger.set_result(result.clone());
        logger.set_is_negative(sign_a);
        logger.log_store_val();
        logger.log_real_val();
        self.stack.push(StackData::new(result.to_bytes_be(), 0u8));
    }
    /// 字节运算
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x08, 0x60, 0x04, 0x1a];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn byte(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_position = self.stack.pop();
        let unit_b = self.stack.pop();
        let mut logger = LogTemplate::new_two_cal(
            "BYTE".to_owned(),
            " byte ".to_owned(),
            unit_b.clone(),
            unit_position.clone(),
        );
        logger.log_two_cal();
        let position = get_uint256(unit_position);
        if position >= BigUint::from(32u8) {
            logger.set_result(zero());
            logger.set_is_negative(0u8);
            logger.log_store_val();
            logger.log_real_val();
            self.stack
                .push(StackData::new(0u8.to_be_bytes().to_vec(), 0u8));
        } else {
            let b = unit_b.0;
            let result: BigUint = (b >> (8 * position.to_usize().unwrap())) & BigUint::from(0xffu8);
            logger.set_result(result.clone());
            logger.set_is_negative(0u8);
            logger.log_store_val();
            logger.log_real_val();
            self.stack.push(StackData::new(result.to_bytes_be(), 0u8));
        }
    }

    /// 左移位运算
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x08, 0x60, 0x04, 0x1b];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn shl(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_r = self.stack.pop();
        let unit_l = self.stack.pop();
        let mut logger = LogTemplate::new_two_cal(
            "SHL".to_owned(),
            "<<".to_owned(),
            unit_l.clone(),
            unit_r.clone(),
        );
        logger.log_two_cal();
        let left = get_uint256(unit_l);
        let right = get_uint256(unit_r);
        let result: BigUint = if right >= (BigUint::from(1u8) << 256) {
            zero()
        } else {
            let mask = (BigUint::from(1u8) << 256) - BigUint::from(1u8);
            (left << (right.to_usize().unwrap())) & mask
        };
        logger.set_result(result.clone());
        logger.set_is_negative(0u8);
        logger.log_store_val();
        logger.log_real_val();
        self.stack.push(StackData::new(result.to_bytes_be(), 0u8));
    }
    /// 右移位运算
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x08, 0x60, 0x04, 0x1c];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn shr(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_r = self.stack.pop();
        let unit_l = self.stack.pop();
        let mut logger = LogTemplate::new_two_cal(
            "SHR".to_owned(),
            ">>".to_owned(),
            unit_l.clone(),
            unit_r.clone(),
        );
        logger.log_two_cal();
        let left = get_uint256(unit_l);
        let right = get_uint256(unit_r);
        let result: BigUint = if right >= (BigUint::from(1u8) << 256) {
            zero()
        } else {
            left >> right.to_usize().unwrap()
        };
        logger.set_result(result.clone());
        logger.set_is_negative(0u8);
        logger.log_store_val();
        logger.log_real_val();
        self.stack.push(StackData::new(result.to_bytes_be(), 0u8));
    }

    ///符号右移位运算
    /// ```
    /// let excute_codes = "60ff60ee0360011d";
    /// let bytes = hex::decode(excute_codes).unwrap();
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn sar(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_r = self.stack.pop();
        let unit_l = self.stack.pop();
        let sign_l = unit_l.1;
        let mut logger = LogTemplate::new_two_cal(
            "SAR".to_owned(),
            ">>".to_owned(),
            unit_l.clone(),
            unit_r.clone(),
        );
        logger.log_two_cal();
        let left = get_uint256(unit_l);
        let right = get_uint256(unit_r);
        let result: BigUint = if right >= (BigUint::from(1u8) << 256) {
            zero()
        } else {
            let mask = (BigUint::from(1u8) << 256) - BigUint::from(1u8);
            if sign_l == 1u8 {
                (left >> right.to_usize().unwrap()) ^ mask
            } else {
                left >> right.to_usize().unwrap()
            }
        };
        logger.set_result(result.clone());
        logger.set_is_negative(sign_l);
        logger.log_store_val();
        logger.log_real_val();
        self.stack
            .push(StackData::new(result.to_bytes_be(), sign_l));
    }
}

#[cfg(test)]
mod tests {
    use crate::evm::*;
    use once_cell::sync::Lazy;
    #[test]
    fn test_and() {
        Lazy::force(&INIT_LOG);
        let bytes = vec![0x60, 0x08, 0x60, 0x06, 0x03, 0x60, 0x02, 0x16];
        let mut evm_test = Evm::new(bytes);
        evm_test.run();
        println!("{:?}", evm_test.stack);
    }

    #[test]
    fn test_byte() {
        Lazy::force(&INIT_LOG);
        let bytes = vec![0x61, 0xff, 0x00, 0x60, 30, 0x1a];
        let mut evm_test = Evm::new(bytes);
        evm_test.run();
        println!("{:?}", evm_test.stack);
    }

    #[test]
    fn test_shl() {
        Lazy::force(&INIT_LOG);
        let excute_codes =
            "7fff0000000000000000000000000000000000000000000000000000000000000060041b";
        let bytes = hex::decode(excute_codes).unwrap();
        let mut evm_test = Evm::new(bytes);
        evm_test.run();
        println!("{:?}", evm_test.stack);
    }

    #[test]
    fn test_shr() {
        Lazy::force(&INIT_LOG);
        let excute_codes = "60ff60041c";
        let bytes = hex::decode(excute_codes).unwrap();
        let mut evm_test = Evm::new(bytes);
        evm_test.run();
        println!("{:?}", evm_test.stack);
    }

    #[test]
    fn test_sar() {
        Lazy::force(&INIT_LOG);
        let excute_codes = "60ff60ee0360011d";
        let bytes = hex::decode(excute_codes).unwrap();
        let mut evm_test = Evm::new(bytes);
        evm_test.run();
        println!("{:?}", evm_test.stack);
    }
}
