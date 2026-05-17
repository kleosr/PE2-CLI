use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

pub fn build_messages(system_content: &str, user_content: &str) -> Vec<Message> {
    vec![
        Message {
            role: "system".to_string(),
            content: system_content.to_string(),
        },
        Message {
            role: "user".to_string(),
            content: user_content.to_string(),
        },
    ]
}

pub fn build_messages_with_system(system: Option<&str>, user: &str) -> Vec<Message> {
    let mut messages = Vec::with_capacity(2);
    if let Some(sys) = system {
        messages.push(Message {
            role: "system".to_string(),
            content: sys.to_string(),
        });
    }
    messages.push(Message {
        role: "user".to_string(),
        content: user.to_string(),
    });
    messages
}
