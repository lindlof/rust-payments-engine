use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct Account {
  client: u16,
  pub available: u64,
  pub held: u64,
  pub total: u64,
  pub locked: bool,
}

impl Account {
  pub fn new(client: u16) -> Account {
    Account {
      client,
      available: 0,
      held: 0,
      total: 0,
      locked: false,
    }
  }
  pub fn total_mut(&mut self) -> &mut u64 {
    &mut self.total
  }
}

impl fmt::Display for Account {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "Account {} ({}, {}, {}, {})",
      self.client, self.available, self.held, self.total, self.locked
    )
  }
}
