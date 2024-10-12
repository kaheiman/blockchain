use crate::environment::config::AppConfig;
use crate::error::BlockchainServiceError;
use crate::prelude::*;
use crate::services::ethereum::EthereumClient;
use crate::domain::token::TokenInfo;
use crate::domain::account::AccountBalance;

pub enum BlockchainType {
    Ethereum,
}

// Define a common interface for blockchain services
#[async_trait]
pub trait BlockchainAdapter {
    async fn get_token_by_address(&self, token_address: &str) -> Result<TokenInfo, BlockchainServiceError>;
    async fn get_account_balance(&self, token_address: &str, wallet_addresses:Vec<String>) -> Result<Vec<AccountBalance>, BlockchainServiceError>;
}

// Facade to abstract blockchain interaction
pub struct BlockchainService {
    eth_client: EthereumClient,
    // bsc_client: BinanceClient, // if we want to extend to other blockchains
}

impl BlockchainService {
    pub fn new(app_config: &AppConfig) -> Self {
        BlockchainService {
            eth_client: EthereumClient::new(app_config.environment.provider_url.as_str()), // Adjust the provider URL as needed
            // bsc_client: BinanceClient::new("https://bsc-dataseed.binance.org/"), // for Binance Smart Chain
        }
    }

    // Determine the correct blockchain client based on some condition
    pub fn get_blockchain_client(&self, blockchain: BlockchainType) -> &dyn BlockchainAdapter {
        match blockchain {
            BlockchainType::Ethereum => &self.eth_client,
        }
    }
}