use crate::prelude::*;
use ethers::prelude::*;
use ethers::providers::{Provider, Http};
use ethers::providers::ProviderError;
use ethers::utils::ConversionError;

#[derive(Error, Debug)]
pub enum AppServerError {
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("Blockchain service error: {0}")]
    BlockchainServiceError(#[from] BlockchainServiceError),
}

#[derive(Error, Debug)]
pub enum BlockchainServiceError {
    #[error("Provider error: {0}")]
    EthProviderError(#[from] ProviderError),

    #[error("Contract error: {0}")]
    EthContractError(#[from] ContractError<Provider<Http>>),

    #[error("Conversion error: {0}")]
    EthConversionError(#[from] ConversionError),

    #[error("Invalid address: {0}")]
    InvalidAddress(String),

    #[error("Parse error: {0}")]
    ParseError(#[from] std::num::ParseIntError),
}