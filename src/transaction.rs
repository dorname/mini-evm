use std::collections::HashMap;

use num_bigint::BigUint;
use num_traits::zero;

#[derive(Debug, Clone)]
pub struct Transaction {
    nonce: BigUint,
    gas_price: BigUint,
    gas_limit: BigUint,
    to: String,
    value: BigUint,
    data: String,
    caller: String,
    origin: String,
    this_addr: String,
    v: BigUint,
    r: BigUint,
    s: BigUint,
}
impl Transaction {
    pub fn mock() -> Self {
        Self {
            nonce: zero(),
            v: zero(),
            r: zero(),
            s: zero(),
            value: BigUint::from(10u8),
            data: "".to_string(),
            // this_addr: "0x9bbfed6889322e016e0a02ee459d306fc19545d8".to_string(),
            // caller: "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045".to_string(),
            // origin: "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045".to_string(),
            // to: "0x9bbfed6889322e016e0a02ee459d306fc19545d8".to_string(),
            this_addr: "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045".to_string(),
            caller: "0x9bbfed6889322e016e0a02ee459d306fc19545d8".to_string(),
            origin: "0x9bbfed6889322e016e0a02ee459d306fc19545d8".to_string(),
            to: "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045".to_string(),
            gas_limit: BigUint::from(10000u32),
            gas_price: BigUint::from(1u8),
        }
    }
    pub fn init(
        nonce: BigUint,
        gas_price: BigUint,
        gas_limit: BigUint,
        to: String,
        value: BigUint,
        data: String,
        caller: String,
        origin: String,
        this_addr: String,
        v: BigUint,
        r: BigUint,
        s: BigUint,
    ) -> Self {
        Self {
            nonce: nonce,
            v: v,
            r: r,
            s: s,
            this_addr: this_addr,
            value: value,
            data: data,
            caller: caller,
            origin: origin,
            to: to,
            gas_limit: gas_limit,
            gas_price: gas_price,
        }
    }
    pub fn get_nonce(&self) -> &BigUint {
        &self.nonce
    }
    pub fn get_v(&self) -> &BigUint {
        &self.v
    }
    pub fn get_r(&self) -> &BigUint {
        &self.r
    }
    pub fn get_s(&self) -> &BigUint {
        &self.s
    }
    pub fn get_this_addr(&self) -> String {
        self.this_addr.to_string()
    }
    pub fn get_value(&self) -> &BigUint {
        &self.value
    }
    pub fn get_data(&self) -> String {
        self.data.to_string()
    }
    pub fn get_caller(&self) -> String {
        self.caller.to_string()
    }
    pub fn get_origin(&self) -> String {
        self.origin.to_string()
    }
    pub fn get_to(&self) -> String {
        self.to.to_string()
    }
    pub fn get_gas_limit(&self) -> &BigUint {
        &self.gas_limit
    }
    pub fn get_gas_price(&self) -> &BigUint {
        &self.gas_price
    }
}
