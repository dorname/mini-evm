pub trait Arithmetic {
    // 算术指令方法签名
    fn add(&mut self);
    fn mul(&mut self);
    fn sub(&mut self);
    fn div(&mut self);
    fn sdiv(&mut self);
    fn n_mod(&mut self);
    fn smod(&mut self);
    fn add_mod(&mut self);
    fn mul_mod(&mut self);
    fn exp(&mut self);
    fn sign_extend(&mut self);
}
pub trait Comparison {
    fn lt(&mut self);
    fn gt(&mut self);
    fn eq(&mut self);
    fn is_zero(&mut self);
    fn slt(&mut self);
    fn sgt(&mut self);
}

//位运算指令集
pub trait Bitwise {
    fn and(&mut self);
    fn or(&mut self);
    fn xor(&mut self);
    fn not(&mut self);
    fn byte(&mut self);
    fn shl(&mut self);
    fn shr(&mut self);
    fn sar(&mut self);
}

pub trait Memory {
    fn mstore(&mut self);
    fn mload(&mut self);
    fn msize(&mut self);
    fn mstore8(&mut self);
}

pub trait Storage {
    fn sstore(&mut self);
    fn sload(&mut self);
}

pub trait ControlFlow {
    fn jump(&mut self);
    fn jumpi(&mut self);
    fn pc(&mut self);
    fn jumpdest(&mut self);
}

pub trait CurrentBlockInfo {
    fn blockhash(&mut self);
    fn coinbase(&mut self);
    fn timestamp(&mut self);
    fn number(&mut self);
    fn prevrandao(&mut self);
    fn gaslimit(&mut self);
    fn chainid(&mut self);
    fn selfbalance(&mut self);
    fn basefee(&mut self);
}

pub trait AccountTraits {
    fn balance(&mut self);
    fn extcodesize(&mut self);
    fn extcodecopy(&mut self);
    fn extcodehash(&mut self);
}

pub trait Other {
    fn sha3(&mut self);
    fn log(&mut self, num_topics: usize);

    // return 指令
    fn return_fn(&mut self);
    fn return_datasize(&mut self);
    fn return_datacopy(&mut self);

    // 异常指令
    fn revert(&mut self);
    fn invalid(&mut self);

    //gas 指令
    fn gas(&mut self);
}

pub trait TransactionTraits {
    fn address(&mut self);
    fn origin(&mut self);
    fn caller(&mut self);
    fn callvalue(&mut self);
    fn calldataload(&mut self);
    fn calldatasize(&mut self);
    fn calldatacopy(&mut self);
    fn codesize(&mut self);
    fn codecopy(&mut self);
    fn gasprice(&mut self);
}

pub trait Call {
    fn call(&mut self);
    fn delegatecall(&mut self);
    fn staticcall(&mut self);
}

pub trait Contract {
    fn create(&mut self);
    fn create2(&mut self);
    fn selfdestruct(&mut self);
}
