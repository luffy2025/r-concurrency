use anyhow::Result;
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;

#[derive(Debug)]
pub struct AmapMetrics {
    pub data: Arc<HashMap<&'static str, AtomicI64>>,
}

impl Clone for AmapMetrics {
    fn clone(&self) -> Self {
        AmapMetrics {
            data: self.data.clone(),
        }
    }
}

impl Display for AmapMetrics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, value) in self.data.iter() {
            writeln!(f, "{}: {}", key, value.load(Ordering::Relaxed))?;
        }
        Ok(())
    }
}

impl AmapMetrics {
    pub fn new(metric_names: &[&'static str]) -> Self {
        let map = metric_names
            .iter()
            .map(|&name| (name, AtomicI64::new(0)))
            .collect();
        AmapMetrics {
            data: Arc::new(map),
        }
    }

    pub fn inc(&self, key: impl Into<String>) -> Result<()> {
        let key = key.into();
        let counter = self
            .data
            .get(key.as_str())
            .ok_or_else(|| anyhow::anyhow!("key {} not found", key))?;
        counter.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }
}
