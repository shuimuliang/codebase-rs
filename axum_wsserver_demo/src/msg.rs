use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ActionType {
    Join,
    Leave,
    Message(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Msg {
    pub action: ActionType,
}

impl TryFrom<&str> for Msg {
    type Error = serde_json::Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(s)
    }
}

impl TryFrom<&Msg> for String {
    type Error = serde_json::Error;

    fn try_from(msg: &Msg) -> Result<Self, Self::Error> {
        serde_json::to_string(msg)
    }
}

impl Msg {
    pub fn new(action: ActionType) -> Self {
        Self { action }
    }
}

#[cfg(test)]
mod tests {
    use super::{ActionType, Msg};

    #[test]
    fn test_msg_serde() {
        let msg = Msg::new(ActionType::Join);
        let json = serde_json::to_string(&msg).unwrap();
        assert_eq!(json, r#"{"action":"join"}"#);

        let msg = Msg::new(ActionType::Message("hello".to_string()));
        let json = serde_json::to_string(&msg).unwrap();
        assert_eq!(json, r#"{"action":{"message":"hello"}}"#);
    }

    #[test]
    fn test_msg_try_from() {
        let json = r#"{"action":{"message":"hello}}"#;
        let msg = Msg::try_from(json);
        assert!(msg.is_err());
    }

    #[test]
    fn test_msg_from() {
        let json = r#"{"action":{"message":"hello"}}"#;
        let msg = Msg::try_from(json).unwrap();
        assert!(matches!(msg.action, ActionType::Message(_)));
    }
}
