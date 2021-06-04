use crate::v2_0_1::core::enumerations::get_certificate_id_use_enum_type::GetCertificateIdUseEnumType;

use super::certificate_hash_data_type::CertificateHashDataType;

/// CertificateHashDataChainType is used by: GetInstalledCertificateIdsResponse
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CertificateHashDataChainType {
    pub certificate_type: GetCertificateIdUseEnumType,
    pub certificate_hash_data: CertificateHashDataType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_certificate_hash_data: Option<CertificateHashDataType>,
}
