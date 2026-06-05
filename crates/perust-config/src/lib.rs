//! # perust-config
//!
//! Server configuration management for PeRust.
//!
//! This crate provides:
//! - **ServerProperties**: Main server configuration (server.properties)
//! - **OpsList**: Server operator list
//! - **Whitelist**: Player whitelist management
//! - **BanList**: Player/IP/CID ban management

pub mod error;
pub mod server_properties;
pub mod ops;
pub mod whitelist;
pub mod ban_list;

pub use error::ConfigError;
pub use server_properties::ServerProperties;
pub use ops::OpsList;
pub use whitelist::Whitelist;
pub use ban_list::BanList;
