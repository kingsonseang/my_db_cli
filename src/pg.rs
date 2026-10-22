use crate::{endpoint::Endpoint, error::DbError};

pub struct PgDb {
    endpoint: Endpoint,
    client: postgres::Client,
}

impl PgDb {
    pub fn connect(endpoint: Endpoint, password: &str) -> Result<Self, DbError> {
        let cfg = format!(
            "host={} post={} user=play dbname=postgres password={}",
            endpoint.host, endpoint.port, password
        );
        let client = postgres::Client::connect(&cfg, postgres::NoTls)
            .map_err(|e| DbError::Pg(e.to_string()))?;
        Ok(Self { endpoint, client })
    }
}
