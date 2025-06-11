use actix_session::Session;
use serde::{Deserialize, Serialize};

const FLASH_MESSAGE_KEY: &str = "flash_message";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlashMessage {
    pub message: String,
    pub message_type: FlashMessageType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FlashMessageType {
    Success,
    Error,
    Warning,
    Info,
}

impl FlashMessage {
    pub fn new(message: String, message_type: FlashMessageType) -> Self {
        Self {
            message,
            message_type,
        }
    }

    pub fn success(message: String) -> Self {
        Self::new(message, FlashMessageType::Success)
    }

    pub fn error(message: String) -> Self {
        Self::new(message, FlashMessageType::Error)
    }

    pub fn warning(message: String) -> Self {
        Self::new(message, FlashMessageType::Warning)
    }

    pub fn info(message: String) -> Self {
        Self::new(message, FlashMessageType::Info)
    }
}

/// セッションにFlashMessageを保存する
pub fn set_flash_message(session: &Session, flash_message: FlashMessage) -> Result<(), actix_session::SessionInsertError> {
    session.insert(FLASH_MESSAGE_KEY, flash_message)
}

/// セッションからFlashMessageを取得し、自動的に削除する
pub fn get_flash_message(session: &Session) -> Option<FlashMessage> {
    match session.get::<FlashMessage>(FLASH_MESSAGE_KEY) {
        Ok(Some(message)) => {
            // メッセージを取得したら即座に削除（一度だけ表示）
            let _ = session.remove(FLASH_MESSAGE_KEY);
            Some(message)
        }
        _ => None,
    }
}

/// FlashMessageのCSS class名を取得
impl FlashMessageType {
    pub fn css_class(&self) -> &'static str {
        match self {
            FlashMessageType::Success => "flash-success",
            FlashMessageType::Error => "flash-error",
            FlashMessageType::Warning => "flash-warning",
            FlashMessageType::Info => "flash-info",
        }
    }
}