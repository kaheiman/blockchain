use ethers::providers::{Http, Provider};
use ethers::types::{Address, U256};
use std::convert::TryFrom;
use crate::prelude::*;
use crate::domain::token::TokenInfo;
use crate::domain::account::AccountBalance;
use crate::error::BlockchainServiceError;
use crate::services::blockchain_service::BlockchainAdapter;
use ethers::contract::abigen;
use ethers::utils::format_units;

abigen!(
    IERC20,
    "./src/abi/erc20.json",
);

// Ethereum-specific implementation of the blockchain service
pub struct EthereumClient {
    provider: Arc<Provider<Http>>,
}

impl EthereumClient {
    pub fn new(provider_url: &str) -> Self {
        // Connect to the Ethereum node
        let provider = Arc::new(Provider::<Http>::try_from(provider_url).expect("Failed to connect to Ethereum node"));

        EthereumClient { provider }
    }
}

// Implement the common interface for Ethereum
#[async_trait]
impl BlockchainAdapter for EthereumClient {
    async fn get_token_by_address(&self, token_address: &str) -> Result<TokenInfo, BlockchainServiceError> {
        let parsed_token_address: Address = token_address.parse() .map_err(|e| {
            BlockchainServiceError::InvalidAddress(format!("{:?}", e))
        })?;
        // Create a contract instance
        let contract: IERC20<Provider<Http>> = IERC20::new(parsed_token_address, self.provider.clone());
        let symbol = contract.symbol().call().await.map_err(|e| { BlockchainServiceError::EthContractError(e) })?;
        // Call the functions to get name, symbol, decimals
        let decimals = contract.decimals().call().await.map_err(|e| { BlockchainServiceError::EthContractError(e) })?;
        let name = contract.name().call().await.map_err(|e| { BlockchainServiceError::EthContractError(e) })?;

        Ok(TokenInfo::new(name, symbol, decimals.into()))
    }

    async fn get_account_balance(&self, token_address: &str, wallet_addresses: Vec<String>) -> Result<Vec<AccountBalance>, BlockchainServiceError> {
        let token_result = self.get_token_by_address(token_address).await.map_err(|e| { e })?;
        let token_address: Address = token_address.parse().map_err(|e| {
            BlockchainServiceError::InvalidAddress(format!("{:?}", e))
        })?;

        // Create a contract instance
        let contract = IERC20::new(token_address, self.provider.clone());

        let mut account_balances = Vec::new();
        for address_str in wallet_addresses {
            let address: Address = address_str.parse().map_err(|e| {
                BlockchainServiceError::InvalidAddress(format!("wallet address [{}] {:?}", address_str, e))
            })?;
            let balance: U256 = contract.balance_of(address).call().await.map_err(|e| {BlockchainServiceError::EthContractError(e) })?;
            let balance_str = format_units(balance, token_result.decimals).map_err(|e| { BlockchainServiceError::EthConversionError(e) })?;

            account_balances.push(AccountBalance::new(address_str.to_string(), balance_str));
        }

        Ok(account_balances)
    }
}