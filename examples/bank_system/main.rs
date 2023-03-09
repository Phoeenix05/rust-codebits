// struct Id {}

// impl Id {
//     fn new() -> Self {
//         todo!()
//     }
// }

use std::{default::Default, fmt::Display, sync::Mutex};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// const DATA_FILE_PATH: &str = "./banks-data.json";

#[derive(Serialize, Deserialize)]
struct Data {
    banks: Vec<Bank>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct Account {
    name: String,
    uuid: String,
    balance: i128
}

impl Account {
    fn new(name: String) -> Self {
        Self {
            name,
            uuid: Uuid::new_v4().to_string(),
            balance: 0
        }
    }

    fn deposit(&mut self, amount: i128) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: i128) {
        self.balance -= amount;
    }
}

impl Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.uuid)
    }
}

lazy_static! {
    pub static ref BANKS: Mutex<Vec<Bank>> = Mutex::new(Vec::new());
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Bank {
    name: String,
    uuid: String,
    accounts: Vec<Account>,
}

impl Bank {
    fn new(name: String) -> Self {
        Self {
            name,
            uuid: Uuid::new_v4().to_string(),
            accounts: Vec::new(),
            // ..Default::default()
        }
    }

    fn add_account(&mut self, account: Account) {
        // self.accounts.insert(self.accounts.len(), account);
        self.accounts.insert(0, account);
    }

    // fn update() {}

    // fn load() {
    //     let banks = BANKS.lock().unwrap();
    // }

    // fn save_all() {
    //     let banks = BANKS.lock().unwrap();
    //     // let abs = std::path::absolute(DATA_FILE_PATH).unwrap();
    //     // let banks = std::fs::read("./banks-data.json").unwrap();
    //     let msg = match std::fs::write(DATA_FILE_PATH, serde_json::json!(*banks).to_string()) {
    //         Ok(_) => format!("Wrote data to ({})", DATA_FILE_PATH),
    //         Err(_) => "\x1b[1;31mError occurred while trying to save data\x1b[0m".to_string(),
    //     };
    // }

    // fn save(&self) {}
}

impl Display for Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({})\nAccounts: {}\n{:#?}",
            self.name,
            self.uuid,
            self.accounts.len(),
            self.accounts.iter().map(|a| a)
        )
    }
}

fn main() {
    // Bank::load();
    // println!("\x1b[1;31mError occurred while trying to save data\x1b[0m");

    // let mut bank = Bank::new("".to_string(), Uuid::new_v4());

    let mut bank = Bank::new("Ok".to_string());
    bank.add_account(Account::new("Anton".to_string()));
    println!("{:#?}", bank)
}
