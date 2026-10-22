use crate::backend::Backend;

pub struct Endpoint {
    pub backend: Backend,
    pub host: String,
    pub port: u16,
}

impl Endpoint {
    pub fn local(backend: Backend) -> Self {
        Self {
            backend,
            host: "127.0.0.1".to_string(),
            port: backend.default_port(),
        }
    }
}
