use super::id_token_info_type::IdTokenInfoType;
use crate::v2_0_1::core::datatypes::id_token_type::IdTokenType;

/// Contains the identifier to use for authorization
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AuthorizationData {
    pub id_token_info: IdTokenInfoType,
    pub id_token: IdTokenType,
}
