use crate::{backend::Backend, error::DbError, report::Report, scope::Scope};

// ---------- trait: the contract ----------
// TS mental model: an interface. Any type that implements
// these methods can be used wherever a `Database` is expected.

pub trait Database {
    fn kind(&self) -> Backend;

    fn list(&self) -> Result<Vec<String>, DbError>;

    fn setup(&mut self, scope: &Scope) -> Result<Report, DbError>;
    fn reset(&mut self, scope: &Scope) -> Result<Report, DbError>;
    fn teardown(&mut self, scope: &Scope) -> Result<Report, DbError>;

    // Default method: implementors get this for free.
    // Uses only the methods above, so it works for ANY backend.
    fn contains(&self, name: &str) -> Result<bool, DbError> {
        Ok(self.list()?.iter().any(|n| n == name))
    }

    fn probe(&self) -> Result<bool, DbError> {
        let addr = format!("127.0.0.1:{}", self.kind().default_port());
        match std::net::TcpStream::connect(addr) {
            Ok(_) => Ok(true),   // something answered — discard the stream with _
            Err(_) => Ok(false), // refused/timeout counts as "not there", not a hard error
        }
    }
}
