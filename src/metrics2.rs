use anyhow::Result;
use dashmap::DashMap;
use std::fmt;
use std::fmt::Formatter;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Metrics2 {
    pub data: Arc<DashMap<String, i64>>,
}

impl Default for Metrics2 {
    fn default() -> Self {
        Self::new()
    }
}

impl Metrics2 {
    pub fn new() -> Self {
        Metrics2 {
            data: Arc::new(DashMap::new()),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let mut counter = self.data.entry(key.into()).or_insert(0);
        *counter += 1;
        Ok(())
    }
}

impl fmt::Display for Metrics2 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for entry in self.data.iter() {
            writeln!(f, "{}: {}", entry.key(), entry.value())?;
        }
        Ok(())
    }
}
