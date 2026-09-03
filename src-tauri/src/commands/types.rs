use serde::{Deserialize, Serialize};

pub const RICKROLL_URL: &str = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommandResponse {
    pub output: String,
    pub open_url: Option<String>,
    pub sound: Option<String>,
}

impl CommandResponse {
    pub fn text<S: Into<String>>(msg: S) -> Self {
        Self {
            output: msg.into(),
            open_url: None,
            sound: None,
        }
    }

    pub fn with_sound<S: Into<String>, T: Into<String>>(msg: S, sound: T) -> Self {
        Self {
            output: msg.into(),
            open_url: None,
            sound: Some(sound.into()),
        }
    }

    pub fn rickroll<S: Into<String>>(msg: S, sound: Option<&str>) -> Self {
        Self {
            output: msg.into(),
            open_url: Some(RICKROLL_URL.to_string()),
            sound: sound.map(|s| s.to_string()),
        }
    }

    pub fn clear() -> Self {
        Self {
            output: "CLEAR_TERMINAL_EVENT".to_string(),
            open_url: None,
            sound: None,
        }
    }
}
