#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum PaymentError {
  #[error("invalid input: {0}")]
  InputError(String),
  #[error("error serializing: {0}")]
  SerializeError(String),
}
