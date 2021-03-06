use crate::account::Account;
use crate::transaction::{Transaction, TxType};
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
      self
        .accounts
        .insert(*client_id, Account::new(*client_id, 0, 0));
    }

    let account = self.accounts.get_mut(client_id).unwrap();

    match tx.txtype() {
      TxType::Deposit => account.available += *tx.amount(),
      TxType::Withdrawal => {
        if account.available >= *tx.amount() {
          account.available -= *tx.amount();
        }
      }
      TxType::Dispute => {}
      TxType::Resolve => {}
      TxType::Chargeback => {}
    }
  }

  pub fn accounts_iter<'a>(&'a self) -> Box<dyn Iterator<Item = &Account> + 'a> {
    let keys = self.accounts.values();
    Box::new(keys)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_deposit() {
    let mut eng = Engine::new();
    eng.process_tx(Transaction::new(TxType::Deposit, 1, 1, 1));
    let value = eng.accounts_iter().next().unwrap();
    assert_eq!(value, &Account::new(1, 1, 0));
  }

  #[test]
  fn test_withdrawal() {
    let mut eng = Engine::new();
    eng.process_tx(Transaction::new(TxType::Deposit, 1, 1, 15));
    eng.process_tx(Transaction::new(TxType::Withdrawal, 1, 1, 10));
    let value = eng.accounts_iter().next().unwrap();
    assert_eq!(value, &Account::new(1, 15 - 10, 0));
  }

  #[test]
  fn test_over_withdrawal() {
    let mut eng = Engine::new();
    eng.process_tx(Transaction::new(TxType::Deposit, 1, 1, 15));
    eng.process_tx(Transaction::new(TxType::Withdrawal, 1, 1, 20));
    let value = eng.accounts_iter().next().unwrap();
    assert_eq!(value, &Account::new(1, 15, 0));
  }

  #[test]
  fn test_multiple_clients() {
    let mut eng = Engine::new();
    eng.process_tx(Transaction::new(TxType::Deposit, 1, 1, 15));
    eng.process_tx(Transaction::new(TxType::Deposit, 2, 1, 15));
    eng.process_tx(Transaction::new(TxType::Deposit, 3, 1, 15));
    eng.process_tx(Transaction::new(TxType::Deposit, 1, 1, 15));
    let iter = eng.accounts_iter();
    assert_eq!(iter.count(), 3);
  }
}
