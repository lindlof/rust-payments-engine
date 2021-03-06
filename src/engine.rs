use crate::account::Account;
use crate::transaction::{Transaction, TxType};
use std::collections::HashMap;

pub struct Engine {
  accounts: HashMap<u16, Account>,
  deposits: HashMap<u32, Transaction>,
  disputes: HashMap<u32, bool>,
}

impl Engine {
  pub fn new() -> Engine {
    Engine {
      accounts: HashMap::new(),
      deposits: HashMap::new(),
      disputes: HashMap::new(),
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

    if account.locked {
      return;
    }

    match tx.txtype() {
      TxType::Deposit => {
        account.available += *tx.amount();
        self.deposits.insert(*tx.tx(), tx);
      }
      TxType::Withdrawal => {
        if account.available >= *tx.amount() {
          account.available -= *tx.amount();
        }
      }
      TxType::Dispute => match self.deposits.get(tx.tx()) {
        Some(depo) => {
          // Assuming partner error if deposit was withdrawn before dispute
          if account.available >= *depo.amount() {
            // Assuming no multiple disputes for tx
            account.available -= *depo.amount();
            account.held += *depo.amount();
            self.disputes.insert(*tx.tx(), true);
          }
        }
        None => {}
      },
      TxType::Resolve => match self.deposits.get(tx.tx()) {
        Some(depo) => {
          // Assuming no multiple disputes or resolutions for tx
          if self.disputes.get(tx.tx()).is_none() {
            return;
          }
          account.available += *depo.amount();
          account.held -= *depo.amount();
        }
        None => {}
      },
      TxType::Chargeback => match self.deposits.get(tx.tx()) {
        Some(depo) => {
          // Assuming no multiple disputes for tx
          if self.disputes.get(tx.tx()).is_none() {
            return;
          }
          account.locked = true;
          account.held -= *depo.amount();
        }
        None => {}
      },
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

  #[test]
  fn test_dispute() {
    let mut eng = Engine::new();
    eng.process_tx(Transaction::new(TxType::Deposit, 1, 1, 5));
    eng.process_tx(Transaction::new(TxType::Deposit, 1, 2, 15));
    eng.process_tx(Transaction::new(TxType::Dispute, 1, 2, 0));
    let value = eng.accounts_iter().next().unwrap();
    assert_eq!(value, &Account::new(1, 5, 15));
  }

  #[test]
  fn test_resolve() {
    let mut eng = Engine::new();
    eng.process_tx(Transaction::new(TxType::Deposit, 1, 1, 5));
    eng.process_tx(Transaction::new(TxType::Deposit, 1, 2, 15));
    eng.process_tx(Transaction::new(TxType::Dispute, 1, 2, 0));
    eng.process_tx(Transaction::new(TxType::Resolve, 1, 2, 0));
    let value = eng.accounts_iter().next().unwrap();
    assert_eq!(value, &Account::new(1, 20, 0));
  }

  #[test]
  fn test_chargeback() {
    let mut eng = Engine::new();
    eng.process_tx(Transaction::new(TxType::Deposit, 1, 1, 5));
    eng.process_tx(Transaction::new(TxType::Deposit, 1, 2, 15));
    eng.process_tx(Transaction::new(TxType::Dispute, 1, 2, 0));
    eng.process_tx(Transaction::new(TxType::Chargeback, 1, 2, 0));
    let value = eng.accounts_iter().next().unwrap();
    let mut expected = Account::new(1, 5, 0);
    expected.locked = true;
    assert_eq!(value, &expected);
  }

  #[test]
  fn test_withdraw_after_chargeback() {
    let mut eng = Engine::new();
    eng.process_tx(Transaction::new(TxType::Deposit, 1, 1, 5));
    eng.process_tx(Transaction::new(TxType::Deposit, 1, 2, 15));
    eng.process_tx(Transaction::new(TxType::Dispute, 1, 2, 0));
    eng.process_tx(Transaction::new(TxType::Chargeback, 1, 2, 0));
    // Withdrawal should not work
    eng.process_tx(Transaction::new(TxType::Withdrawal, 1, 3, 1));
    let value = eng.accounts_iter().next().unwrap();
    let mut expected = Account::new(1, 5, 0);
    expected.locked = true;
    assert_eq!(value, &expected);
  }
}
