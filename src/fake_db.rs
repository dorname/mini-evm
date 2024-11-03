use num_bigint::BigUint;
use once_cell::sync::Lazy;
use std::{collections::HashMap, sync::Mutex};

#[derive(Debug, Clone)]
pub struct Account {
    pub balance: BigUint,
    pub nonce: BigUint,
    pub storage: HashMap<BigUint, (BigUint, u8)>,
    pub code: Vec<u8>,
}
#[derive(Debug, Clone)]
pub struct AccountDb {
    data: HashMap<String, Account>,
}
impl Account {
    pub fn new(
        balance: BigUint,
        nonce: BigUint,
        storage: HashMap<BigUint, (BigUint, u8)>,
        code: Vec<u8>,
    ) -> Self {
        Self {
            balance,
            nonce,
            storage,
            code,
        }
    }
}
impl AccountDb {
    pub fn mock() -> Self {
        let account = Account {
            balance: BigUint::from(100u8),
            nonce: BigUint::from(1u8),
            storage: HashMap::new(),
            code: hex::decode("60006000").unwrap(),
        };
        let account2 = Account {
            balance: BigUint::from(100u8),
            nonce: BigUint::from(1u8),
            storage: HashMap::new(),
            code: hex::decode("60006000").unwrap(),
        };
        let mut accounts: HashMap<String, Account> = HashMap::new();
        accounts.insert(
            "0x9bbfed6889322e016e0a02ee459d306fc19545d8".to_string(),
            account,
        );
        accounts.insert(
            "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045".to_string(),
            account2,
        );
        Self { data: accounts }
    }
    pub fn mock_2() -> Self {
        let account_1 = Account {
            balance: BigUint::from(100u8),
            nonce: BigUint::from(1u8),
            storage: HashMap::new(),
            code: hex::decode("").unwrap(),
        };
        let account_2 = Account {
            balance: BigUint::from(0u8),
            nonce: BigUint::from(0u8),
            storage: HashMap::new(),
            code: hex::decode("60426000526001601ff3").unwrap(),
        };
        let mut accounts: HashMap<String, Account> = HashMap::new();
        accounts.insert(
            "0x9bbfed6889322e016e0a02ee459d306fc19545d8".to_string(),
            account_1,
        );
        accounts.insert(
            "0x1000000000000000000000000000000000000c42".to_string(),
            account_2,
        );
        Self { data: accounts }
    }
    pub fn get_account(&self, address: String) -> &Account {
        self.data.get(&address).unwrap()
    }
    pub fn get_account_mut(&mut self, address: String) -> &mut Account {
        self.data.get_mut(&address).unwrap()
    }
    pub fn insert(&mut self, address: String, account: Account) {
        self.data.insert(address, account);
    }
    pub fn contains(&mut self, address: String) -> bool {
        self.data.contains_key(&address)
    }
    pub fn remove(&mut self, address: String) {
        self.data.remove(&address);
    }
}
