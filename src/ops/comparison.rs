use crate::log_utils::*;
use crate::ops::traits::*;
use crate::utils::*;
use crate::{evm::Evm, stack::StackData};
use num_bigint::BigUint;
use num_traits::{one, zero};
impl Comparison for Evm {
    /// 小于
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x08, 0x60, 0x04, 0x10];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn lt(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_a = self.stack.pop();
        let unit_b = self.stack.pop();
        let mut logger = LogTemplate::new_two_cal(
            "LT".to_string(),
            "<".to_string(),
            unit_a.clone(),
            unit_b.clone(),
        );
        let a = get_uint256(unit_a);
        let b = get_uint256(unit_b);
        logger.log_two_cal();
        let res = BigUint::from((a < b) as u8);
        logger.set_result(res.clone());
        logger.set_is_negative(0u8);
        logger.log_store_val();
        logger.log_real_val();
        self.stack.push(StackData::new(res.to_bytes_be(), 0u8));
    }

    /// 大于
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x08, 0x60, 0x04, 0x11];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn gt(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_a = self.stack.pop();
        let unit_b = self.stack.pop();
        let mut logger = LogTemplate::new_two_cal(
            "GT".to_string(),
            ">".to_string(),
            unit_a.clone(),
            unit_b.clone(),
        );
        let a = get_uint256(unit_a);
        let b = get_uint256(unit_b);
        logger.log_two_cal();
        let res = BigUint::from((a > b) as u8);
        logger.set_result(res.clone());
        logger.set_is_negative(0u8);
        logger.log_store_val();
        logger.log_real_val();
        self.stack.push(StackData::new(res.to_bytes_be(), 0u8));
    }

    /// 等于
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x08, 0x60, 0x04, 0x12];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn eq(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_a = self.stack.pop();
        let unit_b = self.stack.pop();
        let mut logger = LogTemplate::new_two_cal(
            "EQ".to_string(),
            "=".to_string(),
            unit_a.clone(),
            unit_b.clone(),
        );
        let a = get_uint256(unit_a);
        let b = get_uint256(unit_b);
        logger.log_two_cal();
        let res = BigUint::from((a == b) as u8);
        logger.set_result(res.clone());
        logger.set_is_negative(0u8);
        logger.log_store_val();
        logger.log_real_val();
        self.stack.push(StackData::new(res.to_bytes_be(), 0u8));
    }

    /// 零值判断
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x08, 0x60, 0x04, 0x13];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn is_zero(&mut self) {
        if self.stack.len() < 1 {
            panic!("Stack underflow");
        }
        let unit_a = self.stack.pop();
        let mut logger = LogTemplate::new_two_cal(
            "ISZERO".to_string(),
            "is_zero".to_string(),
            unit_a.clone(),
            (zero(), 0u8),
        );
        let a = get_uint256(unit_a);
        logger.log_two_cal();
        logger.set_is_negative(0u8);
        if a == zero() {
            logger.set_result(one());
            self.stack
                .push(StackData::new(1u8.to_be_bytes().to_vec(), 0u8));
        } else {
            logger.set_result(zero());
            self.stack
                .push(StackData::new(0u8.to_be_bytes().to_vec(), 0u8));
        }
        logger.log_store_val();
        logger.log_real_val();
    }

    /// 带符号的大于比较
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x08, 0x60, 0x04, 0x14];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn sgt(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_a = self.stack.pop();
        let unit_b = self.stack.pop();
        let sign_a = unit_a.1;
        let sign_b = unit_b.1;
        let mut logger = LogTemplate::new_two_cal(
            "SGT".to_string(),
            ">".to_string(),
            unit_a.clone(),
            unit_b.clone(),
        );
        let a = get_uint256(unit_a);
        let b = get_uint256(unit_b);
        logger.log_two_cal();
        // true 同号 为1 false 异号 为0
        let same_sign = !((sign_a ^ sign_b) != 0) as u8;
        let res = if same_sign == 1u8 {
            if sign_a == 1u8 {
                BigUint::from((a < b) as u8)
            } else {
                BigUint::from((a > b) as u8)
            }
        } else {
            if sign_b == 1u8 {
                BigUint::from(1u8)
            } else {
                BigUint::from(0u8)
            }
        };
        logger.set_result(res.clone());
        logger.set_is_negative(0u8);
        logger.log_store_val();
        logger.log_real_val();
        self.stack.push(StackData::new(res.to_bytes_be(), 0u8));
    }

    /// 带符号的小于比较
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x08, 0x60, 0x04, 0x15];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn slt(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_a = self.stack.pop();
        let unit_b = self.stack.pop();
        let sign_a = unit_a.1;
        let sign_b = unit_b.1;
        let mut logger = LogTemplate::new_two_cal(
            "SLT".to_string(),
            "<".to_string(),
            unit_a.clone(),
            unit_b.clone(),
        );
        let a = get_uint256(unit_a);
        let b = get_uint256(unit_b);
        logger.log_two_cal();
        // true 同号 为1 false 异号 为0
        let same_sign = !((sign_a ^ sign_b) != 0) as u8;
        let res = if same_sign == 1u8 {
            if sign_a == 1u8 {
                BigUint::from((a > b) as u8)
            } else {
                BigUint::from((a < b) as u8)
            }
        } else {
            if sign_b == 1u8 {
                BigUint::from(0u8)
            } else {
                BigUint::from(1u8)
            }
        };
        logger.set_result(res.clone());
        logger.set_is_negative(0u8);
        logger.log_store_val();
        logger.log_real_val();
        self.stack.push(StackData::new(res.to_bytes_be(), 0u8));
    }
}

#[cfg(test)]
mod tests {
    use crate::evm::*;
    use once_cell::sync::Lazy;
    #[test]
    fn lt_test() {
        Lazy::force(&INIT_LOG);
        let bytes = vec![0x60, 0x08, 0x60, 0x04, 0x10];
        let mut evm_test = Evm::new(bytes);
        evm_test.run();
        println!("{:?}", evm_test.stack);
    }

    #[test]
    fn gt_test() {
        Lazy::force(&INIT_LOG);
        let bytes = vec![0x60, 0x08, 0x60, 0x04, 0x11];
        let mut evm_test = Evm::new(bytes);
        evm_test.run();
        println!("{:?}", evm_test.stack);
    }

    #[test]
    fn slt_test() {
        Lazy::force(&INIT_LOG);
        let bytes = vec![
            0x60, 0x08, 0x60, 0x04, 0x03, 0x60, 0x06, 0x60, 0x01, 0x03, 0x12,
        ];
        let mut evm_test = Evm::new(bytes);
        evm_test.run();
        println!("{:?}", evm_test.stack);
    }

    #[test]
    fn sgt_test() {
        Lazy::force(&INIT_LOG);
        let bytes = vec![
            0x60, 0x08, 0x60, 0x04, 0x03, 0x60, 0x06, 0x60, 0x03, 0x03, 0x13,
        ];
        let mut evm_test = Evm::new(bytes);
        evm_test.run();
        println!("{:?}", evm_test.stack);
    }

    #[test]
    fn eq_test() {
        Lazy::force(&INIT_LOG);
        let bytes = vec![0x60, 0x08, 0x60, 0x08, 0x14];
        let mut evm_test = Evm::new(bytes);
        evm_test.run();
        println!("{:?}", evm_test.stack);
    }

    #[test]
    fn is_zero_test() {
        Lazy::force(&INIT_LOG);
        let bytes = vec![0x60, 0x00, 0x15];
        let mut evm_test = Evm::new(bytes);
        evm_test.run();
        println!("{:?}", evm_test.stack);
    }
}
