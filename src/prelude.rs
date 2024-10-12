// Async runtime and utilities
pub use tokio::signal::unix::{signal, SignalKind};
pub use tokio::sync::Notify;

pub use thiserror::Error;
pub use tracing_subscriber::EnvFilter;

// Trait to define async behavior in traits
pub use async_trait::async_trait;

// JSON handling
pub use serde::{Deserialize, Serialize};
pub use serde_json::{json, from_str};

// Configuration utilities
pub use config::{Config, File};

pub use std::io::{self};

// Environment variables initialization
pub use once_cell::sync::OnceCell;

pub use dotenv::dotenv;
pub use std::collections::{HashSet, HashMap};
pub use std::env;
pub use std::fs;
pub use std::sync::Arc;
pub use std::time::Duration;

// Logging (optional, but useful for API servers)
pub use tracing::{info, error, Level, Span};

// Axum prelude
pub use axum::{
    body::Body,
    error_handling::HandleErrorLayer,
    extract::{Path, State, Query},
    http::{Request as AxumHttpRequest, StatusCode},
    response::{IntoResponse, Json},
    routing::get,
    Router,
};

pub use tower::{
    timeout::{error::Elapsed, TimeoutLayer},
    BoxError, ServiceBuilder,
};
pub use tower_http::trace::{DefaultOnResponse, TraceLayer};

pub use uuid::Uuid;
