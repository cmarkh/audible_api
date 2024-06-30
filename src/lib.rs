pub mod auth;
pub mod localization;

pub type Result<T, E = Box<dyn std::error::Error>> = core::result::Result<T, E>;
