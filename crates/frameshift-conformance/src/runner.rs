use crate::error::ConformanceError;
use async_trait::async_trait;

/// Abstracts the model adapter that turns a prompt into a response.
///
/// The runtime supplies a real impl (HTTP call to Anthropic/OpenAI/etc.);
/// tests use [`MockRunner`].
#[async_trait]
pub trait Runner: Send + Sync {
    async fn run(&self, prompt: &str) -> Result<String, ConformanceError>;
}

/// Always returns the same canned response. Used by tests and for offline
/// development of the harness itself.
pub struct MockRunner {
    pub canned_response: String,
}

impl MockRunner {
    pub fn new(canned_response: impl Into<String>) -> Self {
        Self {
            canned_response: canned_response.into(),
        }
    }
}

#[async_trait]
impl Runner for MockRunner {
    async fn run(&self, _prompt: &str) -> Result<String, ConformanceError> {
        Ok(self.canned_response.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn mock_runner_returns_canned_response() {
        let runner = MockRunner::new("hello world");
        let response = runner.run("anything").await.expect("runner");
        assert_eq!(response, "hello world");
    }
}
