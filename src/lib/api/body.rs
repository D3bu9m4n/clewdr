use axum::extract::FromRequestParts;
use rquest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::warn;

use crate::{
    state::ClientState,
    types::message::{ContentBlock, ImageSource, Message, Role},
};

/// Claude.ai attachment
#[derive(Deserialize, Serialize, Debug)]
pub struct Attachment {
    extracted_content: String,
    file_name: String,
    file_type: String,
    file_size: u64,
}

impl Attachment {
    pub fn new(content: String) -> Self {
        Attachment {
            file_size: content.len() as u64,
            extracted_content: content,
            file_name: "paste.txt".to_string(),
            file_type: "txt".to_string(),
        }
    }
}

/// Request body to be sent to the Claude.ai
#[derive(Deserialize, Serialize, Debug)]
pub struct RequestBody {
    pub max_tokens_to_sample: u64,
    pub attachments: Vec<Attachment>,
    pub files: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    pub rendering_mode: String,
    pub prompt: String,
    pub timezone: String,
    #[serde(skip)]
    pub images: Vec<ImageSource>,
}

fn max_tokens() -> u64 {
    4096
}

/// Request body sent from the client
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ClientRequestBody {
    #[serde(default = "max_tokens")]
    pub max_tokens: u64,
    pub messages: Vec<Message>,
    #[serde(default)]
    pub stop_sequences: Vec<String>,
    pub model: String,
    #[serde(default)]
    pub stream: bool,
    #[serde(default)]
    pub thinking: Option<Thinking>,
    #[serde(default)]
    pub system: Value,
    #[serde(default)]
    pub temperature: f32,
    #[serde(default)]
    pub top_p: f32,
    #[serde(default)]
    pub top_k: u64,
}

/// Thinking mode in Claude API Request
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Thinking {
    budget_tokens: u64,
    r#type: String,
}

pub struct KeyAuth(pub String);

impl FromRequestParts<ClientState> for KeyAuth {
    type Rejection = StatusCode;
    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &ClientState,
    ) -> Result<Self, Self::Rejection> {
        let key = parts
            .headers
            .get("x-api-key")
            .and_then(|v| v.to_str().ok())
            .unwrap_or_default();
        if !state.config.auth(key) {
            warn!("Invalid password: {}", key);
            return Err(StatusCode::UNAUTHORIZED);
        }
        Ok(KeyAuth(key.to_string()))
    }
}

/// Transform a string to a message
pub fn non_stream_message(str: String) -> Message {
    Message::new_blocks(Role::Assistant, vec![ContentBlock::Text { text: str }])
}
