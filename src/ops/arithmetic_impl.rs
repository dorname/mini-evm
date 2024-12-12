use crate::log_utils::*;
use crate::ops::traits::*;
use crate::utils::*;
use crate::{evm::Evm, stack::StackData};
use num_bigint::{BigInt, BigUint, Sign};
use num_integer::Integer;
use num_traits::{zero, One, ToPrimitive, Zero};
/// 算术指令集特征
impl Arithmetic for Evm {
    /// add
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x02, 0x60, 0x03,0x01];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn add(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_a = self.stack.pop();
        let unit_b = self.stack.pop();
        let mut logger = LogTemplate::new_two_cal(
            "ADD".to_owned(),
            "+".to_owned(),
            unit_a.clone(),
            unit_b.clone(),
        );
        logger.log_two_cal();
        let sign_a = unit_a.1;
        let sign_b = unit_b.1;
        let a = get_uint256(unit_a);
        let b = get_uint256(unit_b);
        // !(1 ^ 1)=1 !(0 ^ 0)=1 !(1 ^ 0)=0 !(0 ^ 1)=0
        // true 同号 为1 false 异号 为0
        let same_sign = !((sign_a ^ sign_b) != 0) as u8;
        let sign_a_b = if same_sign == 1u8 && sign_a == 1u8 {
            1u8
        } else {
            0u8
        };
        let result: BigUint = match same_sign {
            1u8 => {
                let mut add_result: BigUint =
                    (a.clone() + b.clone()) % (BigUint::from(1u32) << 256); //加法结果需要模2^256，防止溢出\
                if sign_a_b == 1 {
                    add_result = (BigUint::from(1u32) << 256) - add_result;
                }
                add_result
            }
            _ => {
                // 1 -2
                if a < b {
                    let add_result: BigUint = (BigUint::from(1u32) << 256)
                        - ((b.clone() - a.clone()) % (BigUint::from(1u32) << 256)); // 加法结果需要模2^256，防止溢出
                    add_result
                } else {
                    // 2 -1
                    let add_result: BigUint =
                        (a.clone() - b.clone()) % (BigUint::from(1u32) << 256); // 加法结果需要模2^256，防止溢出
                    add_result
                }
            }
        };
        logger.set_result(result.clone());
        logger.set_is_negative(sign_a_b.clone());
        logger.log_store_val();
        logger.log_real_val();
        self.stack
            .push(StackData::new(result.to_bytes_be(), sign_a_b));
    }

    /// 乘法指令
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x02, 0x60, 0x03,0x02];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn mul(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_a = self.stack.pop();
        let unit_b = self.stack.pop();
        let mut logger = LogTemplate::new_two_cal(
            "MUL".to_owned(),
            "*".to_owned(),
            unit_a.clone(),
            unit_b.clone(),
        );
        logger.log_two_cal();
        let sign_a = unit_a.1;
        let sign_b = unit_b.1;
        let a = get_uint256(unit_a);
        let b = get_uint256(unit_b);
        let mut result: BigUint = (a * b) % (BigUint::from(1u32) << 256);
        // !(1 ^ 1)=1 !(0 ^ 0)=1 !(1 ^ 0)=0 !(0 ^ 1)=0
        // true 同号 为1 false 异号 为0
        let same_sign = !((sign_a ^ sign_b) != 0) as u8;
        if same_sign == 0u8 {
            result = (BigUint::from(1u32) << 256) - result;
        }
        logger.set_result(result.clone());
        logger.set_is_negative(same_sign);
        logger.log_store_val();
        logger.log_real_val();
        self.stack
            .push(StackData::new(result.to_bytes_be(), same_sign));
    }

    /// 减法指令
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x04, 0x60, 0x03,0x03];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn sub(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_a = self.stack.pop();
        let unit_b = self.stack.pop();
        let sign_a = unit_a.1;
        let sign_b = unit_b.1;
        let mut logger = LogTemplate::new_two_cal(
            "SUB".to_owned(),
            "-".to_owned(),
            unit_a.clone(),
            unit_b.clone(),
        );
        logger.log_two_cal();
        // true 同号 为1 false 异号 为0
        let same_sign = !((sign_a ^ sign_b) != 0) as u8;
        let a = get_uint256(unit_a);
        let b = get_uint256(unit_b);
        let sign_a_b = if same_sign == 1 && a.clone() < b.clone() || same_sign == 0 && sign_a == 1 {
            1u8
        } else {
            0u8
        };
        let result: BigUint = match same_sign {
            1u8 => {
                // -1 - -2
                // 1 - 2
                if a < b {
                    (BigUint::from(1u32) << 256) - ((b - a) % (BigUint::from(1u32) << 256))
                } else {
                    (a - b) % (BigUint::from(1u32) << 256)
                }
            }
            _ => {
                //-1 - 2
                // 1 - - 2
                // 2 - -1
                (BigUint::from(1u32) << 256) - ((a + b) % (BigUint::from(1u32) << 256))
            }
        };
        logger.set_result(result.clone());
        logger.set_is_negative(sign_a_b);
        logger.log_store_val();
        logger.log_real_val();
        self.stack
            .push(StackData::new(result.to_bytes_be(), sign_a_b));
    }

    /// 除法指令
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x06, 0x60, 0x03,0x04];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn div(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_a = self.stack.pop();
        let unit_b = self.stack.pop();
        let mut logger = LogTemplate::new_two_cal(
            "DIV".to_owned(),
            "/".to_owned(),
            unit_a.clone(),
            unit_b.clone(),
        );
        logger.log_two_cal();
        let a = get_uint256(unit_a);
        let b = get_uint256(unit_b);
        if b == BigUint::from(0u32) {
            panic!("Division by zero");
        }
        let result: BigUint = (a / b) % (BigUint::from(1u32) << 256);
        logger.set_result(result.clone());
        logger.set_is_negative(0u8);
        logger.log_store_val();
        logger.log_real_val();
        self.stack.push(StackData::new(result.to_bytes_be(), 0u8));
    }

    ///带符号除法运算
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x02, 0x60, 0x03,0x05];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn sdiv(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_a = self.stack.pop();
        let unit_b = self.stack.pop();
        let mut logger = LogTemplate::new_two_cal(
            "SDIV".to_owned(),
            "/".to_owned(),
            unit_a.clone(),
            unit_b.clone(),
        );
        logger.log_two_cal();
        let sign_a = unit_a.1;
        let sign_b = unit_b.1;
        let a = get_uint256(unit_a);
        let b = get_uint256(unit_b);
        // true 同号 为1 false 异号 为0
        let same_sign = !((sign_a ^ sign_b) != 0) as u8;
        if b == BigUint::from(0u32) {
            panic!("Division by zero");
        }
        let mut result: BigUint = (a / b) % (BigUint::from(1u32) << 256);
        let sign_a_b = if same_sign == 1 {
            0u8
        } else {
            result = (BigUint::from(1u32) << 256) - result;
            1u8
        };
        logger.set_result(result.clone());
        logger.set_is_negative(sign_a_b);
        logger.log_store_val();
        logger.log_real_val();
        self.stack
            .push(StackData::new(result.to_bytes_be(), sign_a_b));
    }

    /// 取模指令
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x02, 0x60, 0x03,0x06];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn n_mod(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_a = self.stack.pop();
        let unit_b = self.stack.pop();
        let mut logger = LogTemplate::new_two_cal(
            "MOD".to_owned(),
            "%".to_owned(),
            unit_a.clone(),
            unit_b.clone(),
        );
        logger.log_two_cal();
        let a = get_uint256(unit_a);
        let b = get_uint256(unit_b);
        if b == zero() {
            panic!("Mod by zero");
        }
        let result = a % b;
        logger.set_result(result.clone());
        logger.set_is_negative(0u8);
        logger.log_store_val();
        logger.log_real_val();
        self.stack.push(StackData::new(result.to_bytes_be(), 0u8));
    }

    /// 带符号取模运算
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x02, 0x60, 0x03,0x07];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn smod(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_a = self.stack.pop();
        let unit_b = self.stack.pop();
        let mut logger = LogTemplate::new_two_cal(
            "SMOD".to_owned(),
            "%".to_owned(),
            unit_a.clone(),
            unit_b.clone(),
        );
        logger.log_two_cal();
        let sign_a = unit_a.1;
        let sign_b = unit_b.1;
        let a = get_uint256(unit_a);
        let b = get_uint256(unit_b);
        // true 同号 为1 false 异号 为0
        let same_sign = !((sign_a ^ sign_b) != 0) as u8;
        if b == zero() {
            panic!("Smod by zero");
        }
        let mut result = a % b;
        let mut is_negative = 0u8;
        if same_sign == 0u8 {
            result = (BigUint::from(1u32) << 256) - result;
            is_negative = 1u8;
        }
        logger.set_result(result.clone());
        logger.set_is_negative(is_negative);
        logger.log_store_val();
        logger.log_real_val();
        self.stack
            .push(StackData::new(result.to_bytes_be(), is_negative));
    }

    /// 加法取模运算
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x02, 0x60, 0x03,0x60,0x03,0x08];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn add_mod(&mut self) {
        if self.stack.len() < 3 {
            panic!("Stack underflow");
        }
        let unit_a: (BigUint, u8) = self.stack.pop();
        let unit_b = self.stack.pop();
        let unit_c = self.stack.pop();
        let mut logger = LogTemplate::new_three_cal(
            "ADDMOD".to_owned(),
            "+".to_owned(),
            unit_a.clone(),
            unit_b.clone(),
            unit_c.clone(),
        );
        logger.log_three_cal();
        let sign_c = unit_c.1;
        let a = get_uint256(unit_a);
        let b = get_uint256(unit_b);
        let c = unit_c.0;//get_uint256(unit_c);
        if c.is_zero() {
            panic!("Mod by Zero");
        }
        // if a+b > 0
        let sign_a_b = match a.clone() + b.clone() {
            v if v >= zero() => 0u8,
            _ => 1u8,
        };
        let mut result: BigUint = (a + b) % c;
        // true 同号 为1 false 异号 为0
        // let same_sign = !((sign_a_b ^ sign_c) != 0) as u8;
        let mut is_negative = 0u8;
        // if same_sign == 0u8 {
        //     is_negative = 1u8;
        //     result = (BigUint::from(1u32) << 256) - result;
        // };
        logger.set_result(result.clone());
        logger.set_is_negative(is_negative);
        logger.log_store_val();
        logger.log_real_val();
        self.stack
            .push(StackData::new(result.to_bytes_be(), is_negative));
    }

    /// 乘法取模指令
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x02, 0x60, 0x03,0x60,0x03,0x09];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn mul_mod(&mut self) {
        if self.stack.len() < 3 {
            panic!("Stack underflow");
        }
        let unit_a = self.stack.pop();
        let unit_b = self.stack.pop();
        let unit_c = self.stack.pop();
        let mut logger = LogTemplate::new_three_cal(
            "MULMOD".to_owned(),
            "*".to_owned(),
            unit_a.clone(),
            unit_b.clone(),
            unit_c.clone(),
        );
        logger.log_three_cal();
        let sign_a = unit_a.1;
        let sign_b = unit_b.1;
        let sign_c = unit_c.1;

        let a = get_uint256(unit_a);
        let b = get_uint256(unit_b);
        let c = get_uint256(unit_c);
        if c.is_zero() {
            panic!("Mod by Zero");
        }
        let sign_a_b_c = ((sign_a | sign_b | sign_c) != 0) as u8;
        let mut mul_mod_result = (a.clone() * b.clone()) % c.clone();
        if sign_a_b_c != 0 {
            mul_mod_result = (BigUint::from(1u32) << 256) - mul_mod_result;
        }
        logger.set_result(mul_mod_result.clone());
        logger.set_is_negative(sign_a_b_c);
        logger.log_store_val();
        logger.log_real_val();
        self.stack
            .push(StackData::new(mul_mod_result.to_bytes_be(), sign_a_b_c));
    }

    /// 指数运算指令
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x02, 0x60, 0x03,0x0a];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn exp(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_a = self.stack.pop();
        let unit_b = self.stack.pop();
        let mut logger = LogTemplate::new_two_cal(
            "EXP".to_owned(),
            "^".to_owned(),
            unit_a.clone(),
            unit_b.clone(),
        );
        logger.log_two_cal();
        let a = get_uint256(unit_a);
        let b = get_uint256(unit_b);
        let result = if b.is_zero() {
            BigUint::from(0u8)
        } else {
            a.modpow(&b, &(BigUint::from(1u32) << 256))
        };
        logger.set_result(result.clone());
        logger.set_is_negative(0u8);
        logger.log_store_val();
        logger.log_real_val();
        self.stack.push(StackData::new(result.to_bytes_be(), 0u8));
    }

    /// 符号位扩展指令
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x02, 0x60, 0x03,0x0b];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    fn sign_extend(&mut self) {
        if self.stack.len() < 2 {
            panic!("Stack underflow");
        }
        let unit_b = self.stack.pop();
        let unit_x = self.stack.pop();
        let mut logger = LogTemplate::new_two_cal(
            "SIGNEXTEND".to_owned(),
            "extend".to_owned(),
            unit_b.clone(),
            unit_x.clone(),
        );
        logger.log_two_cal();
        let b = get_uint256(unit_b);
        let x = get_uint256(unit_x);
        if b < BigUint::from(32u8) {
            // 将 b 转换为 u32
            let b_u32 = b.to_u32().unwrap();

            // 计算位索引：bit_index = 8 * b + 7
            let bit_index = 8u32 * b_u32 + 7;

            // 创建符号位掩码：sign_bit = 1 << bit_index
            let sign_bit = BigUint::one() << bit_index;

            // 创建值掩码：mask = (1 << (bit_index + 1)) - 1
            let mask = (BigUint::one() << (bit_index + 1)) - BigUint::one();

            // 创建全 1 掩码：full_mask = 2^256 - 1
            let full_mask = (BigUint::one() << 256) - BigUint::one();

            // 计算 not_mask = full_mask ^ mask
            let not_mask = &full_mask ^ &mask;

            // 提取有效位
            let value = &x & &mask;

            // 检查符号位是否为 1
            let is_negative = (&x & &sign_bit) != BigUint::zero();

            // 执行符号扩展
            let result: BigInt = if is_negative {
                // 负数，需要将高位全部置为 1
                let extended: BigInt = BigInt::from_biguint(Sign::Minus, not_mask | value);
                // 确保结果在 256 位范围内
                let modulus: BigInt = BigInt::from(1u8) << 256;

                (extended + &modulus).mod_floor(&modulus)
            } else {
                // 正数，保持不变
                BigInt::from_biguint(Sign::Plus, value)
            };

            // 将结果转换回 BigUint
            let result_uint = match result.to_biguint() {
                Some(res) => res,
                None => panic!("Failed to convert result to BigUint"),
            };

            logger.set_result(result_uint.clone());
            logger.set_is_negative(is_negative as u8);
            logger.log_store_val();
            logger.log_real_val();
            // 将结果压回栈中
            self.stack.push(StackData::new(
                result_uint.to_bytes_be(),
                if is_negative { 1u8 } else { 0u8 },
            ));
        } else {
            logger.set_result(x.clone());
            logger.set_is_negative(0u8);
            logger.log_store_val();
            logger.log_real_val();
            self.stack.push(StackData::new(x.to_bytes_be(), 0u8));
        }
    }
}

#[cfg(test)]
mod tests {
    use once_cell::sync::Lazy;
    use crate::evm::*;
    #[test]
    fn add_test() {
        Lazy::force(&INIT_LOG);
        let excute_codes = "6002600301";
        let bytes = hex::decode(excute_codes).unwrap();
        let mut evm_test = Evm::new(bytes);
        evm_test.run();
        assert_eq!("0000000000000000000000000000000000000000000000000000000000000005",hex::encode(evm_test.stack.get(1).data));
    }

    #[test]
    fn mul_test() {
        Lazy::force(&INIT_LOG);
        let excute_codes = "6002600302";
        let bytes = hex::decode(excute_codes).unwrap();
        let mut evm_test = Evm::new(bytes);
        evm_test.run();
        assert_eq!("0000000000000000000000000000000000000000000000000000000000000006",hex::encode(evm_test.stack.get(1).data));
    }

    #[test]
    fn sub_test() {
        Lazy::force(&INIT_LOG);
        let excute_codes = "6004600303600501";
        let bytes = hex::decode(excute_codes).unwrap();
        let mut evm_test = Evm::new(bytes);
        evm_test.run();
        assert_eq!("0000000000000000000000000000000000000000000000000000000000000004",hex::encode(evm_test.stack.get(1).data));
    }

    #[test]
    fn div_test() {
        Lazy::force(&INIT_LOG);
        let excute_codes = "6006601204";
        let bytes = hex::decode(excute_codes).unwrap();
        let mut evm_test = Evm::new(bytes);
        evm_test.run();
        assert_eq!("0000000000000000000000000000000000000000000000000000000000000003",hex::encode(evm_test.stack.get(1).data));
    }
    /// 算数指令

    #[test]
    fn sdiv_test() {
        Lazy::force(&INIT_LOG);
        let excute_codes = "6004600203600b05";
        let bytes = hex::decode(excute_codes).unwrap();
        let mut evm_test = Evm::new(bytes);
        evm_test.run();
        assert_eq!("fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffb",hex::encode(evm_test.stack.get(1).data));
    }

    #[test]
    fn mod_test() {
        Lazy::force(&INIT_LOG);
        let excute_codes = "6004600806";
        let bytes = hex::decode(excute_codes).unwrap();
        let mut evm_test = Evm::new(bytes);
        evm_test.run();
        assert_eq!("0000000000000000000000000000000000000000000000000000000000000000",hex::encode(evm_test.stack.get(1).data));
    }

    #[test]
    fn smod_test() {
        Lazy::force(&INIT_LOG);
        let excute_codes = "6008600403600907";
        let bytes = hex::decode(excute_codes).unwrap();
        let mut evm_test = Evm::new(bytes);
        evm_test.run();
        println!("{:?}", evm_test.stack);
        assert_eq!("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff",hex::encode(evm_test.stack.get(1).data));

    }

    #[test]
    fn add_mod_test() {
        Lazy::force(&INIT_LOG);
        // let bytes = vec![0x60, 0x08, 0x60, 0x04, 0x03, 0x60, 0x01, 0x60, 0x09, 0x08];
        // let bytes = vec![0x60, 0x03, 0x60, 0x06, 0x03, 0x60, 0x01, 0x60, 0x09, 0x08];
        let excute_codes = "60086004036001600908";
        let bytes = hex::decode(excute_codes).unwrap();
        let mut evm_test = Evm::new(bytes);
        evm_test.run();
        // println!("{:?}", evm_test.stack);
        assert_eq!("000000000000000000000000000000000000000000000000000000000000000a",hex::encode(evm_test.stack.get(1).data));
    }

    #[test]
    fn mul_mod_test() {
        Lazy::force(&INIT_LOG);
        let bytes = vec![0x60, 0x08, 0x60, 0x04, 0x03, 0x60, 0x01, 0x60, 0x09, 0x09];
        // let bytes = vec![0x60, 0x03, 0x60, 0x06, 0x03, 0x60, 0x01, 0x60, 0x09, 0x09];

        let mut evm_test = Evm::new(bytes);
        evm_test.run();
        println!("{:?}", evm_test.stack);
    }

    #[test]
    fn exp_test() {
        Lazy::force(&INIT_LOG);
        let bytes = vec![0x60, 0x03, 0x60, 0x02, 0x0a];
        let mut evm_test = Evm::new(bytes);
        evm_test.run();
        println!("{:?}", evm_test.stack);
    }

    #[test]
    fn sign_extend_test() {
        Lazy::force(&INIT_LOG);
        let bytes = vec![0x60, 0x08, 0x60, 0x04, 0x0b];
        let mut evm_test = Evm::new(bytes);
        evm_test.run();
        println!("{:?}", evm_test.stack);
    }
}
