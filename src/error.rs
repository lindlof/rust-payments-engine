#[derive(thiserror::Error, Debug, PartialEq, Clone)]
pub enum PaymentError {
  #[error("Invalid input: {0}")]
  InputError(String),
}
