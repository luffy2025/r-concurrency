use anyhow::Result;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Formatter;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone)]
pub struct RwMetrics {
    pub data: Arc<RwLock<HashMap<String, i64>>>,
}

impl Default for RwMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl RwMetrics {
    pub fn new() -> Self {
        RwMetrics {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut data = self
            .data
            .write()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        let counter = data.entry(key.into()).or_insert(0);
        *counter += 1;

        Ok(())
    }

    pub fn snapshot(&self) -> Result<HashMap<String, i64>> {
        Ok(self
            .data
            .read()
            .map_err(|e| anyhow::anyhow!(e.to_string()))?
            .clone())
    }
}

impl fmt::Display for RwMetrics {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let data = self.data.read().map_err(|_e| fmt::Error)?;
        for (key, value) in data.iter() {
            writeln!(f, "{}: {}", key, value)?;
        }
        Ok(())
    }
}
