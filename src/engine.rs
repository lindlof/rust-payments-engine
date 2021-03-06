use crate::account::Account;
use crate::transaction::Transaction;
use std::collections::HashMap;

pub struct Engine {
  accounts: HashMap<u16, Account>,
}

impl Engine {
  pub fn new() -> Engine {
    Engine {
      accounts: HashMap::new(),
    }
  }
  pub fn process_tx(&mut self, tx: Transaction) {
    let client_id = tx.client();
    if !self.accounts.contains_key(client_id) {
      self.accounts.insert(*client_id, Account::new(*client_id));
    }

    let account = self.accounts.get_mut(client_id).unwrap();
    *account.total_mut() = account.total + *tx.amount();
  }

  pub fn accounts_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &Account> + 'a> {
    let keys = self.accounts.values();
    Box::new(keys)
  }
}
