use std::collections::HashMap;
use crate::stack::Stack;
use crate::stack::StackData;
use crate::const_var::*;
use crate::curr_block::*;
use crate::log_entry::LogEntry;
use crate::ops::traits::*;
use crate::transaction::*;
use crate::utils::*;
use hex::decode;
use log::*;
use num_bigint::BigUint;
use num_traits::zero;
use num_traits::{ToPrimitive, Zero};

#[derive(Debug)]
pub struct Evm {
    //以太坊虚拟机字节码
    code: Vec<u8>,
    //程序计数器
    pub pc: usize,
    //堆栈
    //每个元素长度为256位（32字节），最大深度为1024元素，但是每个操作只能操作堆栈顶的16个元素
    pub stack: Stack,
    //存储
    pub storage: HashMap<BigUint, (BigUint, u8)>,
    //内存
    pub memory: Vec<u8>,
    // 有效指令
    pub valid_jumpdest: HashMap<usize, bool>,

    pub current_block: CurrentBlock,

    pub txn: Transaction,

    pub logs: Vec<LogEntry>,

    pub return_data: Vec<u8>,

    pub success: bool,

    pub is_static: bool,

    pub gas_used: BigUint,
}

/// 为虚拟机实现其特征行为和方法
/// 1）初始化方法，
///    字节码为入参
///    计数器初始值为0
///    堆栈初始化为空
impl Evm {
    /// 初始化虚拟机
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x01, 0x60, 0x01,0x50];
    /// let mut evm_test = Evm::new(bytes);
    /// ```
    pub fn new(code: Vec<u8>) -> Self {
        init_log();

        // 初始化valid_jumpdest
        // let mut vaild_jumpdest: HashMap<usize, bool> = HashMap::new();
        let valid_jumpdest = code
            .iter()
            .fold(
                (HashMap::<usize, bool>::new(), 0 as usize),
                |(mut mp, idx), &val| {
                    let mut step = 1 as usize;
                    if val == JUMPDEST {
                        mp.insert(idx, true);
                    }
                    if PUSH1 <= val && val <= PUSH32 {
                        step = (val - PUSH1 + 1) as usize;
                    }
                    (mp, idx + step)
                },
            )
            .0;

        Evm {
            code: code,
            pc: 0,
            stack: Stack::new(),
            memory: Vec::<u8>::new(),
            storage: HashMap::new(),
            valid_jumpdest: valid_jumpdest,
            current_block: CurrentBlock::init(),
            txn: Transaction::mock(),
            logs: Vec::<LogEntry>::new(),
            return_data: Vec::<u8>::new(),
            success: true,
            is_static: true,
            gas_used: zero(),
        }
    }
    /// 初始化虚拟机并设置上下文txn
    /// 后续将new替换成init_evm
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x01, 0x60, 0x01,0x50];
    /// let mut evm_test = Evm::new(bytes);
    /// ```
    pub fn init_evm(code: Vec<u8>, txn: Transaction) -> Self {
        // init_log();

        // 初始化valid_jumpdest
        // let mut vaild_jumpdest: HashMap<usize, bool> = HashMap::new();
        let valid_jumpdest = code
            .iter()
            .fold(
                (HashMap::<usize, bool>::new(), 0 as usize),
                |(mut mp, idx), &val| {
                    let mut step = 1 as usize;
                    if val == JUMPDEST {
                        mp.insert(idx, true);
                    }
                    if PUSH1 <= val && val <= PUSH32 {
                        step = (val - PUSH1 + 1) as usize;
                    }
                    (mp, idx + step)
                },
            )
            .0;

        Evm {
            code: code,
            pc: 0,
            stack: Stack::new(),
            memory: Vec::<u8>::new(),
            storage: HashMap::new(),
            valid_jumpdest: valid_jumpdest,
            current_block: CurrentBlock::init(),
            txn: txn,
            logs: Vec::<LogEntry>::new(),
            return_data: Vec::<u8>::new(),
            success: true,
            is_static: true,
            gas_used: zero(),
        }
    }
    /// 合约间调用，用于上一组指令执行完后，保留返回的结果并执行下一组指令
    /// 仅用于returncopy的测试
    pub fn next_codes(&mut self, code: Vec<u8>) {
        self.code = code;
        self.pc = 0;
    }

    /// 获取当前待执行的指令
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x01, 0x60, 0x01,0x50];
    /// let mut evm_test = Evm::new(bytes);
    /// let op:u8 = evm_test.get_current_instruction();
    /// ```
    pub fn get_current_instruction(&mut self) -> u8 {
        let &op: &u8 = self.code.get(self.pc).unwrap();
        info!("当前的程序计数器为{}", self.pc);
        info!("当前执行的指令为{}", get_instruction_name(op));
        //程序计数器累加，代表当前指令已取出并准备执行，计数器指向下一个指令。
        self.pc += 1;
        info!(
            "下一个程序计数器值:{}(获取当前指令后,程序计数器指向下一个元素索引故pc+1)",
            self.pc
        );
        return op.clone();
    }
    pub fn is_state_change_code(&mut self, code: u8) -> bool {
        [0xf0, 0xf5, 0xff, 0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0x55].contains(&code)
    }
    /// 执行所有指令
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x01, 0x60, 0x01,0x50];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.run();
    /// ```
    pub fn run(&mut self) {
        while self.pc < self.code.len() {
            let op: u8 = self.get_current_instruction();
            if GAS_COSTS.contains_key(&op) {
                info!(
                    "{}:{} gas",
                    get_instruction_name(op.clone()),
                    GAS_COSTS.get(&op).unwrap().clone()
                );
                self.gas_used += BigUint::from(GAS_COSTS.get(&op).unwrap().clone());
            }
            match op {
                op if (PUSH1 <= op && op <= PUSH32) => {
                    let size = (op - PUSH1 + 1) as usize;
                    self.push(size);
                }
                op if (DUP1 <= op && op <= DUP16) => {
                    let index = (op - DUP1 + 1) as usize;
                    self.dup(index);
                }
                op if (SWAP1 <= op && op <= SWAP16) => {
                    let index = (op - SWAP1 + 1) as usize;
                    self.swap(index);
                }
                PUSH0 => {
                    self.stack.push(StackData::new(0u8.to_be_bytes().to_vec(),0u8));
                    // self.pc += size; // 此行应当被删除或者注释掉，因为在 PUSH0 的情况下并未定义 size
                }
                POP => {
                    self.pop();
                }
                ADD => {
                    self.add();
                }
                MUL => {
                    self.mul();
                }
                SUB => {
                    self.sub();
                }
                DIV => {
                    self.div();
                }
                SDIV => {
                    self.sdiv();
                }
                MOD => {
                    self.n_mod();
                }
                SMOD => {
                    self.smod();
                }
                ADDMOD => {
                    self.add_mod();
                }
                MULMOD => {
                    self.mul_mod();
                }
                EXP => {
                    self.exp();
                }
                SIGNEXTEND => {
                    self.sign_extend();
                }
                LT => {
                    self.lt();
                }
                GT => {
                    self.gt();
                }
                SLT => {
                    self.slt();
                }
                SGT => {
                    self.sgt();
                }
                EQ => {
                    self.eq();
                }
                ISZERO => {
                    self.is_zero();
                }
                AND => {
                    self.and();
                }
                OR => {
                    self.or();
                }
                XOR => {
                    self.xor();
                }
                NOT => {
                    self.not();
                }
                BYTE => {
                    self.byte();
                }
                SHL => {
                    self.shl();
                }
                SHR => {
                    self.shr();
                }
                SAR => {
                    self.sar();
                }
                MSTORE => {
                    self.mstore();
                }
                MSTORE8 => {
                    self.mstore8();
                }
                MSIZE => {
                    self.msize();
                }
                MLOAD => {
                    self.mload();
                }
                SSTORE => {
                    self.sstore();
                }
                SLOAD => {
                    self.sload();
                }
                STOP => {
                    info!("stop");
                    break;
                }
                JUMPDEST => {
                    self.jumpdest();
                }
                JUMP => {
                    self.jump();
                }
                JUMPI => {
                    self.jumpi();
                }
                PC => {
                    self.pc();
                }
                BLOCKHASH => {
                    self.blockhash();
                }
                COINBASE => {
                    self.coinbase();
                }
                TIMESTAMP => {
                    self.timestamp();
                }
                NUMBER => {
                    self.number();
                }
                PREVRANDAO => {
                    self.prevrandao();
                }
                GASLIMIT => {
                    self.gaslimit();
                }
                CHAINID => {
                    self.chainid();
                }
                SELFBALANCE => {
                    self.selfbalance();
                }
                BASEFEE => {
                    self.basefee();
                }
                SHA3 => {
                    self.sha3();
                }
                BALANCE => {
                    self.balance();
                }
                EXTCODESIZE => {
                    self.extcodesize();
                }
                EXTCODECOPY => {
                    self.extcodecopy();
                }
                EXTCODEHASH => {
                    self.extcodehash();
                }
                ADDRESS => {
                    self.address();
                }
                ORIGIN => {
                    self.origin();
                }
                CALLER => {
                    self.caller();
                }
                CALLVALUE => {
                    self.callvalue();
                }
                CALLDATALOAD => {
                    self.calldataload();
                }
                CALLDATASIZE => {
                    self.calldatasize();
                }
                CALLDATACOPY => {
                    self.calldatacopy();
                }
                CODESIZE => {
                    self.codesize();
                }
                CODECOPY => {
                    self.codecopy();
                }
                GASPRICE => {
                    self.gasprice();
                }
                operation if operation >= LOG0 && operation <= LOG4 => {
                    let num_topics = operation - LOG0;
                    self.log(num_topics.into());
                }
                RETURN => {
                    self.return_fn();
                }
                RETURNDATASIZE => {
                    self.return_datasize();
                }
                RETURNDATACOPY => {
                    self.return_datacopy();
                }
                REVERT => {
                    self.revert();
                }
                INVALID => {
                    self.invalid();
                }
                CALL => {
                    self.call();
                }
                DELEGATECALL => {
                    self.delegatecall();
                }
                STATICCALL => {
                    if self.is_state_change_code(op.clone()) && self.is_static {
                        self.success = false;
                        panic!("State changing operation detected during STATICCALL!");
                    }
                    self.staticcall();
                }
                CREATE => {
                    self.create();
                }
                CREATE2 => {
                    self.create2();
                }
                SELFDESTRUCT => {
                    self.selfdestruct();
                }
                GAS => {
                    self.gas();
                }
                _ => {
                    // 处理其他未覆盖到的操作
                }
            }
            if &self.gas_used > self.txn.get_gas_limit() {
                panic!("Out of gas!");
            }
        }
    }

    /// 堆栈行为
    /// 出栈
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x01, 0x60, 0x01,0x50];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.pop();
    /// ```
    pub fn pop(&mut self) {
        if self.stack.len() != 0 {
            self.stack.pop();
        } else {
            warn!("栈空无法调用pop方法");
        }
    }

    /// 入栈
    /// ```
    /// use evm_lab::evm::Evm;
    /// let bytes = vec![0x60, 0x01, 0x60, 0x01,0x50];
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.push(0 as usize);
    /// ```
    pub fn push(&mut self, size: usize) {
        let ops: Vec<u8> = self.code[self.pc..self.pc + size].to_vec();
        let result = ops
            .iter()
            .fold((ops.len() - 1, BigUint::zero()), |(pos, mut sum), &x| {
                let mut value: u32 = u32::from_str_radix(x.to_string().as_str(), 16).unwrap();
                if x > 0x09 {
                    value = x.clone() as u32;
                }
                sum += BigUint::from(value) << (8 * pos);
                if pos == 0 {
                    return (pos, sum);
                }
                (pos - 1, sum)
                //
            })
            .1;
        info!("PUSH的值为:{}", vec_to_hex_string(result.to_radix_be(16)));
    
        self.stack.push(StackData::new(result.to_bytes_be(),0u8));
        // 入栈时程序计数器累加，size为入栈元素的个数
        info!("程序计数器:{}(将size个元素入栈，pc+size)", self.pc + size);
        self.pc += size
    }
    /// 复制操作
    /// 操作指令为 80-8F
    /// ```
    /// use evm_lab::evm::Evm;
    /// let excute_codes = "62ff0080";
    /// let bytes = hex::decode(excute_codes).unwrap();
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.push(0 as usize);
    /// ```
    pub fn dup(&mut self, index: usize) {
        if self.stack.len() < index {
            panic!("Stack underflow");
        }
        info!("复制栈顶元素，并压入栈顶");
        let top_element = self.stack.get(index);
        self.stack.push(top_element);
    }
    /// 交换指令
    /// 操作指令为 90-9F
    /// ```
    /// use evm_lab::evm::Evm;
    /// let excute_codes = "60016011600291";
    /// let bytes = hex::decode(excute_codes).unwrap();
    /// let mut evm_test = Evm::new(bytes);
    /// evm_test.push(0 as usize);
    /// ```
    pub fn swap(&mut self, index: usize) {
        if self.stack.len() < index + 1 {
            panic!("Stack underflow");
        }
        info!("交换栈顶元素和第{}个元素", index);
        let len = self.stack.len();
        self.stack.swap(len, index);
    }
    pub fn fill_memory(&mut self) {
        // 获取当前内存长度
        let current_len = self.memory.len();

        // 计算到32的下一个倍数需要的字节数
        let padding_needed = (32 - (current_len % 32)) % 32;

        // 如果需要填充，则扩展内存
        if padding_needed > 0 {
            // 扩展内存，将内存填充到32的倍数
            self.memory.resize(current_len + padding_needed, 0);
        }
    }
}

pub fn init_log() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_push() {
        let excute_codes = "62ff0011";
        let bytes = hex::decode(excute_codes).unwrap();
        // let bytes = vec![0x61, 0xff,0x00];
        let mut evm_test = Evm::new(bytes);
        evm_test.run();
        println!("{:?}", evm_test.stack);
    }
    
    #[test]
    fn test_idx() {
        let mut value: u32 = u32::from_str_radix(0xff.to_string().as_str(), 16).unwrap();
        if 0xff > 0x09 {
            value = 0xff.clone() as u32;
        }
        let result: BigUint = BigUint::from(value) << 8;
        println!("PUSH的值为:{}", vec_to_hex_string(result.to_radix_be(16)));
    }
    
    #[test]
    fn test_dup() {
        let excute_codes = "62ff001180";
        let bytes = hex::decode(excute_codes).unwrap();
        let mut evm_test = Evm::new(bytes);
        evm_test.run();
        println!("{:?}", evm_test.stack);
    }
    
    #[test]
    fn test_swap() {
        let excute_codes = "60016011600291";
        let bytes = hex::decode(excute_codes).unwrap();
        let mut evm_test = Evm::new(bytes);
        evm_test.run();
        println!("{:?}", evm_test.stack);
    }
}


