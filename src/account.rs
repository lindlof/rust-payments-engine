use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Account {
  client: u16,
  pub available: u64,
  pub held: u64,
  pub locked: bool,
}

impl Account {
  pub fn new(client: u16, available: u64, held: u64) -> Account {
    Account {
      client,
      available,
      held,
      locked: false,
    }
  }

  pub fn client(&self) -> u16 {
    self.client
  }
  pub fn total(&self) -> u64 {
    // Not handling a possible overflow
    self.available + self.held
  }
}

impl fmt::Display for Account {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "Account {} ({}, {}, {}, {})",
      self.client,
      self.available,
      self.held,
      self.total(),
      self.locked
    )
  }
}
