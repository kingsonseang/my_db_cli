use thiserror::Error;

use crate::backend::Backend;

// An error ENUM: each variant is a different failure.
// TS mental model: a discriminated union of error shapes,
// except the message template lives right here on each variant.

#[derive(Error, Debug)]
pub enum DbError {
    #[error("prefix must not be empty")]
    EmptyPrefix,

    #[error("database '{name}' not found")]
    NotFound { name: String },

    #[error("{backend} is not reachable on port {port}")]
    NotReachable { backend: Backend, port: u16 },

    #[error("prefix '{prefix}' is invalid")]
    InvalidPrefix { prefix: String },

    #[error("prefix '{prefix}' is reserved")]
    ReservedPrefix { prefix: String },

    #[error(
        "docker is not available; start Docker Desktop or install it, then run: docker compose -f dev-db.compose.yml up -d postgres"
    )]
    NoDocker,

    #[error("keychain error: {0}")]
    Keychain(String),

    #[error("docker compose failed: {0}")]
    Compose(String),

    #[error("postgres error: {0}")]
    Pg(String),
}
