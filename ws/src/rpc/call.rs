#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Call {
    pub message_type_id: i64,
    pub message_id: String,
    pub action: String,
    pub payload: serde_json::Value,
}

impl Call {
    pub fn new(
        message_type_id: i64,
        message_id: String,
        action: String,
        payload: serde_json::Value,
    ) -> Self {
        Self {
            message_type_id,
            message_id,
            action,
            payload,
        }
    }
}
