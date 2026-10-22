use crate::backend::Backend;
use crate::db::Database;
use crate::error::DbError;
use crate::report::Report;
use crate::scope::Scope;
use std::collections::HashMap;

// ---------- mock: a real implementation with no database ----------

pub struct MockDb {
    kind: Backend,
    // db name -> list of collection/table names
    dbs: HashMap<String, Vec<String>>,
}

impl MockDb {
    pub fn new(kind: Backend) -> Self {
        Self {
            kind,
            dbs: HashMap::new(),
        }
    }

    // Non-trait helper, just for display. Takes &self: reads, doesn't mutate.
    pub fn summary(&self) -> String {
        format!("{}: {} db(s)", self.kind, self.dbs.len())
    }
}

impl Database for MockDb {
    fn kind(&self) -> Backend {
        self.kind
    }

    fn list(&self) -> Result<Vec<String>, DbError> {
        Ok(self.dbs.keys().cloned().collect())
    }

    fn setup(&mut self, scope: &Scope) -> Result<Report, DbError> {
        // `format!` builds a String (owned). `insert` takes ownership of the key.
        let name = format!("{}mydb", scope.prefix());
        // `or_insert` borrows the map mutably, returns &mut Vec<String>.
        // If the key is missing, inserts an empty vec first.
        self.dbs
            .entry(name.clone())
            .or_insert(vec!["items".to_string()]);
        Ok(Report {
            backend: self.kind,
            prefix: scope.prefix().to_string(),
            touched: vec![name],
            skipped: vec![],
        })
    }

    fn reset(&mut self, scope: &Scope) -> Result<Report, DbError> {
        // Snapshot the key list first. `keys()` lends from the map;
        // `.cloned().collect()` turns those lends into owned Strings
        // so the lend ends BEFORE we start mutating below.
        let names: Vec<String> = self.dbs.keys().cloned().collect();

        let mut touched = Vec::new();
        let mut skipped = Vec::new();

        for name in names {
            if scope.permits(&name) {
                // Permitted: safe to unwrap — the key came from the map itself.
                // `if let Some` guards anyway; cheap insurance, no crash possible.
                if let Some(cols) = self.dbs.get_mut(&name) {
                    cols.clear();
                    cols.push("items".to_string());
                    touched.push(name)
                }
            } else {
                skipped.push(name);
            }
        }

        if touched.is_empty() {
            return Err(DbError::NotFound {
                name: scope.prefix().to_string(),
            });
        }

        Ok(Report {
            backend: self.kind,
            prefix: scope.prefix().to_string(),
            touched,
            skipped,
        })
    }

    fn teardown(&mut self, scope: &Scope) -> Result<Report, DbError> {
        let names: Vec<String> = self.dbs.keys().cloned().collect();

        let mut touched = Vec::new();
        let mut skipped = Vec::new();

        for name in names {
            if scope.permits(&name) {
                self.dbs.remove(&name);
                touched.push(name);
            } else {
                skipped.push(name);
            }
        }

        if touched.is_empty() {
            return Err(DbError::NotFound {
                name: scope.prefix().to_string(),
            });
        }

        Ok(Report {
            backend: self.kind,
            prefix: scope.prefix().to_string(),
            touched,
            skipped,
        })
    }
}
