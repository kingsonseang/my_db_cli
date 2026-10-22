use std::io::Write;

use keyring::Entry;

use crate::{backend::Backend, error::DbError};

const SERVICE: &str = "my_db_cli";

fn key_for(backend: Backend) -> String {
    format!("{}:play", backend.compose_service())
}

pub fn load(backend: Backend) -> Result<Option<String>, DbError> {
    // key = "postgres:play" — backend + user, so mongo and postgres never collide
    // get_password errors when missing; missing = Ok(None), real failure = Err
    let entry =
        Entry::new(SERVICE, &key_for(backend)).map_err(|e| DbError::Keychain(e.to_string()))?;
    match entry.get_password() {
        Ok(pw) => Ok(Some(pw)),
        Err(keyring::Error::NoEntry) => Ok(None),
        Err(e) => Err(DbError::Keychain(e.to_string())),
    }
}

pub fn prompt_and_save(backend: Backend) -> Result<String, DbError> {
    // print "Password for postgres user 'play': ", flush stdout (else prompt hides behind buffering),
    // read_line, trim, set_password, return it
    print!("Password for {} user 'play':", backend.compose_service());
    std::io::stdout()
        .flush()
        .map_err(|e| DbError::Keychain(e.to_string()))?;

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .map_err(|e| DbError::Keychain(e.to_string()))?;

    let password = input.trim().to_string();
    Entry::new(SERVICE, &key_for(backend))
        .map_err(|e| DbError::Keychain(e.to_string()))?
        .set_password(&password)
        .map_err(|e| DbError::Keychain(e.to_string()))?;
    Ok(password)
}
