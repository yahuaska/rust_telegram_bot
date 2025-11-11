use crate::core::commands::CommandRegistry;
use std::sync::atomic::AtomicI64;
use std::sync::Arc;

pub struct Bot {
    pub token: String,
    pub offset: AtomicI64,
    pub polling_timeout: i64,
    pub base_url: String,
    pub handlers: Arc<dyn CommandRegistry>,
}

impl Bot {
    pub fn new(
        token: String,
        offset: AtomicI64,
        polling_timeout: i64,
        base_url: String,
        handlers: Arc<dyn CommandRegistry>,
    ) -> Self {
        Self {
            token,
            offset,
            polling_timeout,
            base_url,
            handlers,
        }
    }

    pub fn update_offset(&self, offset: i64) {
        if offset >= self.offset.load(std::sync::atomic::Ordering::Relaxed) {
            self.offset
                .store(offset + 1, std::sync::atomic::Ordering::Relaxed);
        }
    }

    pub fn url(&self, method: &str) -> String {
        format!("{}/bot{}/{method}", self.base_url, self.token)
    }
}
