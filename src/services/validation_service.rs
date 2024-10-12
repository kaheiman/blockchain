use crate::prelude::*;
use crate::services::blockchain_service::{BlockchainService, BlockchainType};

// Define the struct for deserializing the JSON file
type UserAddresses = HashMap<String, Vec<String>>;

pub struct ValidationService {
  blockchain_service:  Arc<BlockchainService>,
}

impl ValidationService {
  pub fn new(blockchain_service: BlockchainService) -> Self {
      Self {
          blockchain_service: Arc::new(blockchain_service),
      }
  }

  pub async fn validate(&self) {
    let user_addresses = get_user_addresses_data_from_json("./geth/addresses.json").unwrap();
    let contract_address = "0x0000000000000000000000000000000000001111";
    let token_result = self.blockchain_service.get_blockchain_client(BlockchainType::Ethereum).get_token_by_address(contract_address).await.unwrap();
    for (user, addresses) in user_addresses {

        let address_balance_result = self.blockchain_service.get_blockchain_client(BlockchainType::Ethereum).get_account_balance(contract_address, addresses).await.unwrap();

        for account_balance in address_balance_result {
            println!("{}: {} {}", account_balance.address, account_balance.balance, token_result.symbol);
            println!("{}: {} {}", user, account_balance.balance, token_result.symbol);
        }
    }
  }
}

fn get_user_addresses_data_from_json(filename: &str) -> serde_json::Result<UserAddresses> {
  let data = fs::read_to_string(filename).unwrap();
  let users_addresses: UserAddresses = from_str(&data)?;
  Ok(users_addresses)
}

// async fn print_balances(user_addresses: UserAddresses) {
//   let result = app_state
//       .blockchain_facade_service
//       .get_blockchain_client(BlockchainType::Ethereum)
//       .get_token_by_address(&token_address).await;

//   // match result {
//   //     Ok(token_info) => {
//   //         Json(token_info).into_response()  // Return the token info as JSON
//   //     }
//   //     Err(e) => {
//   //         error!("Error fetching token info: {:?}", e);
//   //       }

//     for (user, addresses) in user_addresses.users {
//         let mut total_balance: f64 = 0.0;

//         // Print each address and its token balance
//         for address in &addresses {
//             let balance = get_token_balance(address);
//             println!("{}: {:.2} MTK", address, balance);
//             total_balance += balance;
//         }

//         // Print the total balance for the user
//         println!("{}: {:.2} MTK", user, total_balance);
//         println!("--------------------");
//     }
// }