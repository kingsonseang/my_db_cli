use clap::ValueEnum;
use std::fmt;

// ---------- domain ----------

#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum Backend {
    Mongo,
    Postgres,
}

impl fmt::Display for Backend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Backend::Mongo => write!(f, "mongodb"),
            Backend::Postgres => write!(f, "postgres"),
        }
    }
}

impl Backend {
    pub fn default_port(&self) -> u16 {
        match self {
            Backend::Mongo => 27017,
            Backend::Postgres => 5432,
        }
    }
    pub fn compose_service(&self) -> &'static str {
        match self {
            Backend::Mongo => "mongo",
            Backend::Postgres => "postgres",
        }
    }
}
