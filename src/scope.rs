use crate::error::DbError;

pub struct Scope {
    prefix: String,
}

impl Scope {
    pub fn parse(raw: &str) -> Result<Self, DbError> {
        // 1. trim whitespace; empty after trim -> Err(DbError::EmptyPrefix)
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            return Err(DbError::EmptyPrefix);
        }

        // 2. add trailing '_' if missing: "darth" -> "darth_"
        let normalized = if trimmed.ends_with("_") {
            trimmed.to_string()
        } else {
            format!("{trimmed}_")
        };

        // 3. Character check. `char::is_alphanumeric` covers unicode letters too;
        //    fine for DB names, stricter (ASCII-only) can come later.
        let valid = normalized.chars().all(|c| c.is_alphanumeric() || c == '_');
        if !valid {
            return Err(DbError::InvalidPrefix {
                prefix: raw.to_string(),
            }); // report what the USER typed, not the normalized form
        }

        // 4. Reserved names. Compare against the normalized form minus its trailing '_',
        //    plus the pg_ system prefix.
        let stem = normalized.trim_end_matches("_");
        const RESERVED: &[&str] = &[
            "postgres",
            "template0",
            "template1",
            "admin",
            "config",
            "local",
        ];
        if RESERVED.contains(&stem) || stem.starts_with("pg_") {
            return Err(DbError::ReservedPrefix {
                prefix: raw.to_string(),
            });
        }

        Ok(Self { prefix: normalized })
    }

    pub fn prefix(&self) -> &str {
        &self.prefix
    }

    pub fn permits(&self, name: &str) -> bool {
        name.starts_with(&self.prefix)
    }
}
