mod environment;
mod error;
mod prelude;
mod router;
mod services;
mod domain;

use environment::config::get_app_config;
use prelude::*;
use router::AppServiceLayer;
use services::blockchain_service::BlockchainService;
use services::validation_service::ValidationService;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // Initialize tracing subscriber with environment filter
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let app_config = get_app_config().unwrap();
    info!("App Config: {:?}", app_config);

    let valiadator = ValidationService::new(BlockchainService::new(app_config));
    valiadator.validate().await;

    let app_service_layer = Arc::new(AppServiceLayer::new(BlockchainService::new(app_config)));
    let router = router::init_router(app_service_layer);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", app_config.server.port))
        .await
        .unwrap();
    info!("Server started on port: {}", listener.local_addr().unwrap());

    // Create a Notify instance to signal shutdown
    let shutdown_notify = Arc::new(Notify::new());
    let shutdown_notify_clone = shutdown_notify.clone();

    // Spawn a task to listen for shutdown signals
    tokio::spawn(async move {
        // Create signal streams for SIGINT, SIGTERM, and SIGHUP
        let mut sigint = signal(SignalKind::interrupt()).unwrap();
        let mut sigterm = signal(SignalKind::terminate()).unwrap();
        let mut sighup = signal(SignalKind::hangup()).unwrap();

        // Wait for any of the signals
        tokio::select! {
            _ = sigint.recv() => {
                info!("Received SIGINT");
                shutdown_notify_clone.notify_one();
            },
            _ = sigterm.recv() => {
                info!("Received SIGTERM");
                shutdown_notify_clone.notify_one();
            },
            _ = sighup.recv() => {
                info!("Received SIGHUP");
                shutdown_notify_clone.notify_one();
            },
        }
    });

    // Set up a panic hook to handle uncaught exceptions
    let shutdown_notify_clone = shutdown_notify.clone();
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        error!("Uncaught exception: {}", panic_info);
        // Notify the server to shut down
        shutdown_notify_clone.notify_one();
        // Call the default panic hook
        default_panic(panic_info);
    }));

    // Start the server with graceful shutdown
    axum::serve(listener, router)
        .with_graceful_shutdown(wait_for_shutdown(shutdown_notify))
        .await
        .unwrap();
}

// Future that waits for the shutdown signal
async fn wait_for_shutdown(shutdown_notify: Arc<Notify>) {
    shutdown_notify.notified().await;
    info!("Shutdown signal received");
}