use crate::v2_0_1::core::enumerations::vpn_enum_type::VPNEnumType;

/// VPN Configuration settings
/// VPNType is used by: SetNetworkProfileRequest.NetworkConnectionProfileType
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VPNType {
    pub server: String,
    pub user: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    pub password: String,
    pub key: String,
    #[serde(rename = "type")]
    pub kind: VPNEnumType,
}
