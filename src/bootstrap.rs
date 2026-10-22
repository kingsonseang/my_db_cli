use crate::{backend::Backend, endpoint::Endpoint, error::DbError};

fn reachable(ep: &Endpoint) -> bool {
    let addr = format!("{}:{}", ep.host, ep.port);
    std::net::TcpStream::connect(addr).is_ok()
}

pub fn ensure(backend: Backend) -> Result<Endpoint, DbError> {
    let ep = Endpoint::local(backend);
    // probe via TCP (same connect as Lesson 6, but standalone — no &self needed)
    if reachable(&ep) {
        return Ok(ep);
    }
    if !crate::docker::available() {
        return Err(DbError::NoDocker);
    }
    let _pw =
        crate::creds::load(backend)?.map_or_else(|| crate::creds::prompt_and_save(backend), Ok)?;
    crate::docker::compose_up(backend.compose_service())?;
    // compose returns once containers START, not once the DB accepts connections.
    // Postgres needs ~2-5s. Poll, don't assume.
    for _ in 0..10 {
        std::thread::sleep(std::time::Duration::from_millis(500));
        if reachable(&ep) {
            return Ok(ep);
        }
    }
    // not reachable from here down — for THIS lesson, just return NotReachable:
    Err(DbError::NotReachable {
        backend,
        port: ep.port,
    })
}
