use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountBalance {
  pub address: String,
  pub balance: String,
}

impl AccountBalance {
  pub fn new(address: String, balance: String) -> Self {
    AccountBalance { address, balance }
  }
}
