//! # ElevenLabs API Documentation SDK
//!
//! The official Rust SDK for the ElevenLabs API Documentation.
//!
//! ## Getting Started
//!
//! ```rust
//! use elevenlabs_sdk::prelude::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     let config = ClientConfig {
//!         ..Default::default()
//!     };
//!     let client = ElevenlabsClient::new(config).expect("Failed to build client");
//!     client
//!         .history
//!         .list(
//!             &HistoryListQueryRequest {
//!                 page_size: Some(1),
//!                 start_after_history_item_id: Some("start_after_history_item_id".to_string()),
//!                 voice_id: Some("voice_id".to_string()),
//!                 model_id: Some("model_id".to_string()),
//!                 date_before_unix: Some(1),
//!                 date_after_unix: Some(1),
//!                 sort_direction: Some(HistoryListRequestSortDirection::Asc),
//!                 search: Some("search".to_string()),
//!                 source: Some(HistoryListRequestSource::Tts),
//!                 ..Default::default()
//!             },
//!             None,
//!         )
//!         .await;
//! }
//! ```
//!
//! ## Modules
//!
//! - [`api`] - Core API types and models
//! - [`client`] - Client implementations
//! - [`config`] - Configuration options
//! - [`core`] - Core utilities and infrastructure
//! - [`error`] - Error types and handling
//! - [`prelude`] - Common imports for convenience

pub mod api;
pub mod client;
pub mod config;
pub mod core;
pub mod environment;
pub mod error;
pub mod prelude;

pub use client::*;
pub use config::*;
pub use core::*;
pub use environment::*;
pub use error::{ApiError, BuildError};
