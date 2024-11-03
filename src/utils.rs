use crate::{
    const_var::*,
    fake_db::{self, AccountDb},
};
use num_bigint::BigUint;
use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex};
use std::{fmt::format, sync::MutexGuard};
use tiny_keccak::{Hasher, Keccak};

/// 判断是否是有符号的数据
/// 如果是则返回其补码，否则返回本身
/// ```
/// use evm_lab::evm::Evm;
/// let bytes = vec![0x60, 0x01, 0x60, 0x01,0x50];
/// let mut evm_test = Evm::new(bytes);
/// let unit_x = (BigUint::from(1u32),1u8);
/// let result = evm_test.get_uint256(unit_x);
pub fn get_uint256(unit_x: (BigUint, u8)) -> BigUint {
    match unit_x.1 {
        1 => (BigUint::from(1u32) << 256) - unit_x.0,
        _ => unit_x.0,
    }
}

pub fn vec_to_hex_string(bytes: Vec<u8>) -> String {
    bytes
        .iter()
        .map(|byte: &u8| match format!("{:1x}", byte) {
            v if v.len() < 2 => {
                format!("0{}", v)
            }
            v if v.len() >= 2 => {
                // println!("{}", v);
                v
            }
            _ => "err".to_string(),
        })
        .collect::<String>()
}

pub fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32];
    hasher.update(data);
    hasher.finalize(&mut output);
    output
}

pub fn println_hex(bytes: Vec<u8>, size: usize) -> String {
    format!("{:0>size$}", vec_to_hex_string(bytes), size = size)
}

pub fn get_instruction_name(op: u8) -> String {
    match op {
        PUSH0 => "PUSH0".to_string(),
        operation if operation >= PUSH1 && operation <= PUSH32 => {
            let num = (operation - PUSH1 + 1) as usize;
            format!("PUSH{}", num)
        }
        PUSH32 => "PUSH32".to_string(),
        POP => "POP".to_string(),
        ADD => "ADD".to_string(),
        MUL => "MUL".to_string(),
        SUB => "SUB".to_string(),
        DIV => "DIV".to_string(),
        SDIV => "SDIV".to_string(),
        MOD => "MOD".to_string(),
        SMOD => "SMOD".to_string(),
        ADDMOD => "ADDMOD".to_string(),
        MULMOD => "MULMOD".to_string(),
        EXP => "EXP".to_string(),
        SIGNEXTEND => "SIGNEXTEND".to_string(),
        LT => "LT".to_string(),
        GT => "GT".to_string(),
        SLT => "SLT".to_string(),
        SGT => "SGT".to_string(),
        EQ => "EQ".to_string(),
        ISZERO => "ISZERO".to_string(),
        AND => "AND".to_string(),
        OR => "OR".to_string(),
        XOR => "XOR".to_string(),
        NOT => "NOT".to_string(),
        BYTE => "BYTE".to_string(),
        SHL => "SHL".to_string(),
        SHR => "SHR".to_string(),
        SAR => "SAR".to_string(),
        MSTORE => "MSTORE".to_string(),
        MSTORE8 => "MSTORE8".to_string(),
        MSIZE => "MSIZE".to_string(),
        MLOAD => "MLOAD".to_string(),
        SLOAD => "SLOAD".to_string(),
        SSTORE => "SSTORE".to_string(),
        JUMP => "JUMP".to_string(),
        JUMPI => "JUMPI".to_string(),
        PC => "PC".to_string(),
        JUMPDEST => "JUMPDEST".to_string(),
        STOP => "STOP".to_string(),
        BLOCKHASH => "BLOCKHASH".to_string(),
        COINBASE => "COINBASE".to_string(),
        TIMESTAMP => "TIMESTAMP".to_string(),
        NUMBER => "NUMBER".to_string(),
        PREVRANDAO => "PREVRANDAO".to_string(),
        GASLIMIT => "GASLIMIT".to_string(),
        CHAINID => "CHAINID".to_string(),
        SELFBALANCE => "SELFBALANCE".to_string(),
        BASEFEE => "BASEFEE".to_string(),
        operation if operation >= DUP1 && operation <= DUP16 => {
            let num = (operation - DUP1 + 1) as usize;
            format!("DUP{}", num)
        }
        operation if operation >= SWAP1 && operation <= SWAP16 => {
            let num = (operation - SWAP1 + 1) as usize;
            format!("SWAP{}", num)
        }
        SHA3 => "SHA3".to_string(),
        BALANCE => "BALANCE".to_string(),
        EXTCODECOPY => "EXTCODECOPY".to_string(),
        EXTCODEHASH => "EXTCODEHASH".to_string(),
        EXTCODESIZE => "EXTCODESIZE".to_string(),
        operation if operation >= LOG0 && operation <= LOG4 => {
            let num = (operation - LOG0) as usize;
            format!("LOG{}", num)
        }
        RETURN => "RETURN".to_string(),
        RETURNDATACOPY => "RETURNDATACOPY".to_string(),
        RETURNDATASIZE => "RETURNDATASIZE".to_string(),
        REVERT => "REVERT".to_string(),
        INVALID => "INVALID".to_string(),
        CALL => "CALL".to_string(),
        DELEGATECALL => "DELEGATECALL".to_string(),
        STATICCALL => "STATICCALL".to_string(),
        CREATE => "CREATE".to_string(),
        CREATE2 => "CREATE2".to_string(),
        SELFDESTRUCT => "SELFDESTRUCT".to_string(),
        GAS => "GAS".to_string(),
        _ => "UNKNOWN".to_string(),
    }
}
static FAKE_DB_1: Lazy<Mutex<AccountDb>> = Lazy::new(|| Mutex::new(AccountDb::mock()));
static FAKE_DB_2: Lazy<Mutex<AccountDb>> = Lazy::new(|| Mutex::new(AccountDb::mock_2()));

pub static GAS_COSTS: Lazy<HashMap<u8, u32>> = Lazy::new(|| {
    let mut m = HashMap::new();

    // 常见堆栈和算术操作
    m.insert(STOP, 0);
    m.insert(ADD, 3);
    m.insert(MUL, 5);
    m.insert(SUB, 3);
    m.insert(DIV, 5);
    m.insert(SDIV, 5);
    m.insert(MOD, 5);
    m.insert(SMOD, 5);
    m.insert(ADDMOD, 8);
    m.insert(MULMOD, 8);
    m.insert(EXP, 10); // 动态调整成本
    m.insert(SIGNEXTEND, 5);

    // 比较与位操作
    m.insert(LT, 3);
    m.insert(GT, 3);
    m.insert(SLT, 3);
    m.insert(SGT, 3);
    m.insert(EQ, 3);
    m.insert(ISZERO, 3);
    m.insert(AND, 3);
    m.insert(OR, 3);
    m.insert(XOR, 3);
    m.insert(NOT, 3);
    m.insert(BYTE, 3);
    m.insert(SHL, 3);
    m.insert(SHR, 3);
    m.insert(SAR, 3);

    // SHA3（Keccak256）哈希操作
    m.insert(SHA3, 30); // 动态部分成本

    // 环境信息
    m.insert(ADDRESS, 2);
    m.insert(BALANCE, 100); // 后续硬分叉将可能调整
    m.insert(ORIGIN, 2);
    m.insert(CALLER, 2);
    m.insert(CALLVALUE, 2);
    m.insert(CALLDATALOAD, 3);
    m.insert(CALLDATASIZE, 2);
    m.insert(CALLDATACOPY, 3); // 动态部分成本
    m.insert(CODESIZE, 2);
    m.insert(CODECOPY, 3); // 动态部分成本
    m.insert(GASPRICE, 2);
    m.insert(EXTCODESIZE, 100);
    m.insert(EXTCODECOPY, 100); // 动态部分成本
    m.insert(RETURNDATASIZE, 2);
    m.insert(RETURNDATACOPY, 3); // 动态部分成本
    m.insert(EXTCODEHASH, 100);

    // 区块信息
    m.insert(BLOCKHASH, 20);
    m.insert(COINBASE, 2);
    m.insert(TIMESTAMP, 2);
    m.insert(NUMBER, 2);
    m.insert(PREVRANDAO, 2);
    m.insert(GASLIMIT, 2);
    m.insert(CHAINID, 2);
    m.insert(SELFBALANCE, 2);
    m.insert(BASEFEE, 2);

    // 存储操作
    m.insert(SLOAD, 100);
    m.insert(SSTORE, 100); // 动态部分成本

    // 流程控制
    m.insert(JUMP, 8);
    m.insert(JUMPI, 10);
    m.insert(PC, 2);
    m.insert(MSIZE, 2);
    m.insert(GAS, 2);
    m.insert(JUMPDEST, 1);

    // 内存操作
    m.insert(MLOAD, 3);
    m.insert(MSTORE, 3);
    m.insert(MSTORE8, 3);

    // 日志操作
    m.insert(LOG0, 375); // 动态部分成本
    m.insert(LOG0 + 1, 750); // 动态部分成本
    m.insert(LOG0 + 2, 1125); // 动态部分成本
    m.insert(LOG0 + 3, 1500); // 动态部分成本
    m.insert(LOG0 + 4, 1875); // 动态部分成本

    // 系统操作
    m.insert(CREATE, 32000);
    m.insert(CALL, 700); // 动态部分成本
    m.insert(RETURN, 0); // 动态部分成本
    m.insert(DELEGATECALL, 700); // 动态部分成本
    m.insert(CREATE2, 32000); // 动态部分成本
    m.insert(STATICCALL, 700); // 动态部分成本
    m.insert(REVERT, 0); // 动态部分成本
    m.insert(SELFDESTRUCT, 5000); // 动态部分成本

    // 堆栈操作
    for i in 0u8..=32u8 {
        m.insert(PUSH0 + i, 3);
    }
    for i in 0u8..16u8 {
        m.insert(DUP1 + i, 3);
        m.insert(SWAP1 + i, 3);
    }
    m
});
pub fn get_account_db() -> MutexGuard<'static, AccountDb> {
    FAKE_DB_1.lock().unwrap()
}

pub fn get_account_db_2() -> MutexGuard<'static, AccountDb> {
    FAKE_DB_2.lock().unwrap()
}
