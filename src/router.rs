use crate::prelude::*;
use crate::services::blockchain_service::{BlockchainService, BlockchainType};

pub struct AppServiceLayer {
    pub blockchain_service: Arc<BlockchainService>,
}

impl AppServiceLayer {
    pub fn new(blockchain_service: BlockchainService) -> Self {
        Self {
            blockchain_service: Arc::new(blockchain_service),
        }
    }
}

pub fn init_router(app_state: Arc<AppServiceLayer>) -> Router {
    Router::new()
        .route("/token/info/:token_address", get(get_token_info))
        .route("/token/balances/:token_address", get(get_token_balances))
        .with_state(app_state)
        .layer(
        ServiceBuilder::new()
            .layer(HandleErrorLayer::new(handle_timeout_error))
            .layer(TimeoutLayer::new(Duration::from_secs(30))) // Add timeout middleware
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(|_request: &AxumHttpRequest<Body>| {
                        let transaction_id = Uuid::new_v4().to_string();
                        tracing::info_span!(
                            "",
                            transaction_id
                        )
                    })
                    .on_request(|request: &AxumHttpRequest<_>, span: &Span| {
                        let method = request.method();
                        let headers = request.headers();
                        let uri = request.uri();
                        let query = uri.query().unwrap_or("NONE");
                        info!(parent: span, "methods={}, uri={}, params={} headers={:?}", method, uri, query, headers);
                    })
                    .on_response(DefaultOnResponse::new().level(Level::INFO)), // Response logging

            )
        )
}

// Handler for GET /token/info
async fn get_token_info(
    Path(token_address): Path<String>,
    State(app_state): State<Arc<AppServiceLayer>>,
) -> impl IntoResponse {
    if token_address.is_empty() {
        let response_body  = json!({
            "error": "Invalid token address",
            "message": "empty string",
        });
        return (StatusCode::BAD_REQUEST, Json(response_body)).into_response();
    }

    let result = app_state
        .blockchain_service
        .get_blockchain_client(BlockchainType::Ethereum)
        .get_token_by_address(&token_address).await;

    match result {
        Ok(token_info) => {
            Json(token_info).into_response()  // Return the token info as JSON
        }
        Err(e) => {
            error!("Error fetching token info: {:?}", e);
            let response_body  = json!({
                "error": "Blockchain service error",
                "message": format!("{:?}", e),
            });
            (StatusCode::UNPROCESSABLE_ENTITY, Json(response_body)).into_response()
        }
    }
}


#[derive(Debug, Deserialize)]
struct AddressQuery {
    addresses: String,
}

// Handler for GET /token/balances/:token_address
async fn get_token_balances(
    Path(token_address): Path<String>,
    Query(params): Query<AddressQuery>,
    State(app_state): State<Arc<AppServiceLayer>>,
) -> impl IntoResponse {
    if token_address.is_empty() || token_address.parse::<ethers::types::Address>().is_err() {
        let response_body  = json!({
            "error": "Invalid token address",
            "message": "empty string",
        });
        return (StatusCode::BAD_REQUEST, Json(response_body)).into_response();
    }

    let vec_addresses = params.addresses
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    // Call the blockchain facade to get the token balances for the valid addresses
    let result = app_state
        .blockchain_service
        .get_blockchain_client(BlockchainType::Ethereum)
        .get_account_balance(&token_address, vec_addresses)
        .await;

    match result {
        Ok(balances) => {
            // Return the balances as JSON
            Json(balances).into_response()
        }
        Err(e) => {
            error!("Error fetching token balances: {:?}", e);
            let response_body  = json!({
                "error": "Blockchain service error",
                "message": format!("{:?}", e),
            });
            (StatusCode::UNPROCESSABLE_ENTITY, Json(response_body)).into_response()
        }
    }
}

async fn handle_timeout_error(err: BoxError) -> (StatusCode, String) {
    if err.is::<Elapsed>() {
        (
            StatusCode::REQUEST_TIMEOUT,
            "Request took too long".to_string(),
        )
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {err}"),
        )
    }
}
