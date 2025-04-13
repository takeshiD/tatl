use crate::message::CommitMessage;
use crate::prompt::{context_content, preamble_content};

use rig::extractor::ExtractionError;
use rig::providers::openai;

#[allow(dead_code)]
pub struct MessageProvider {
    model_name: String,
    client: openai::Client,
}

impl MessageProvider {
    pub fn new(model_name: &str) -> Self {
        Self {
            model_name: model_name.to_string(),
            client: openai::Client::from_env(),
        }
    }
    pub async fn generate_message(
        &self,
        diff_content: &str,
        locale: &str,
    ) -> Result<CommitMessage, ExtractionError> {
        let extractor = self
            .client
            .extractor::<CommitMessage>(openai::GPT_4O)
            .preamble(preamble_content())
            .context(context_content(locale).as_str())
            .build();
        extractor.extract(diff_content).await
    }
}
