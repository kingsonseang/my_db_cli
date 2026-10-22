use crate::error::DbError;

pub fn available() -> bool {
    std::process::Command::new("docker")
        .arg("info")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

pub fn compose_up(service: &str) -> Result<(), DbError> {
    let out = std::process::Command::new("docker")
        .args(["compose", "-f", "dev-db.compose.yml", "up", "-d", service])
        .output()
        .map_err(|e| DbError::Compose(e.to_string()))?;
    if out.status.success() {
        Ok(())
    } else {
        Err(DbError::Compose(
            String::from_utf8_lossy(&out.stderr).into_owned(),
        ))
    }
}
