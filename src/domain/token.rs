use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenInfo {
  pub name: String,
  pub symbol: String,
  pub decimals: i32,
}

impl TokenInfo {
  pub fn new(name: String, symbol: String, decimals: i32) -> Self {
    TokenInfo { name, symbol, decimals }
  }
}