use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use serde::de::{self, Deserializer, Visitor};
use serde::ser::{self, Serialize, Serializer};
use serde::Deserialize;
use std::fmt;

#[derive(Deserialize)]
pub struct Transaction {
  #[serde(rename = "type")]
  txtype: TxType,
  client: u16,
  tx: u32,
  amount: Amount,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TxType {
  Deposit,
  Withdrawal,
  Dispute,
  Resolve,
  Chargeback,
}

struct Amount {
  value: u64,
}

impl Transaction {
  pub fn new(txtype: TxType, client: u16, tx: u32, amount: u64) -> Transaction {
    Transaction {
      txtype,
      client,
      tx,
      amount: Amount { value: amount },
    }
  }
  pub fn txtype(&self) -> &TxType {
    &self.txtype
  }
  pub fn client(&self) -> &u16 {
    &self.client
  }
  pub fn tx(&self) -> &u32 {
    &self.tx
  }
  pub fn amount(&self) -> &u64 {
    &self.amount.value
  }
}

impl fmt::Display for Transaction {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "Transaction ({:?}, {}, {}, {:?})",
      self.txtype(),
      self.client(),
      self.tx(),
      self.amount()
    )
  }
}

impl Serialize for Amount {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let mut v = match Decimal::from_u64(self.value) {
      Some(u) => u,
      None => return Err(ser::Error::custom("could not serialize amount to decimal")),
    };
    v.rescale(4);
    serializer.serialize_str(&v.to_string())
  }
}

struct AmountVisitor;

impl<'de> Visitor<'de> for AmountVisitor {
  type Value = Amount;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("an integer between -2^31 and 2^31")
  }

  fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
  where
    E: de::Error,
  {
    let dec = match Decimal::from_str(value) {
      Ok(d) => d,
      Err(e) => {
        return Err(de::Error::custom(format!(
          "could not deserialize amount: {}",
          e.to_string()
        )))
      }
    };
    let dec = dec.checked_mul(Decimal::new(10000, 0)).unwrap();
    let value = dec.to_u64().unwrap();
    Ok(Amount { value })
  }
}

impl<'de> Deserialize<'de> for Amount {
  fn deserialize<D>(deserializer: D) -> Result<Amount, D::Error>
  where
    D: Deserializer<'de>,
  {
    deserializer.deserialize_str(AmountVisitor)
  }
}
