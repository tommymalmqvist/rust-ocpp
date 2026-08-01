use super::apn_type::APNType;
use super::vpn_type::VPNType;
use crate::v2_0_1::enumerations::ocpp_interface_enum_type::OCPPInterfaceEnumType;
use crate::v2_0_1::enumerations::ocpp_transport_enum_type::OCPPTransportEnumType;
use crate::v2_0_1::enumerations::ocpp_version_enum_type::OCPPVersionEnumType;
#[cfg(not(feature = "std"))]
use alloc::string::String;

/// The NetworkConnectionProfile defines the functional and technical parameters of a communication link.
/// NetworkConnectionProfileType is used by: SetNetworkProfileRequest
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct NetworkConnectionProfileType {
    pub ocpp_version: OCPPVersionEnumType,
    pub ocpp_transport: OCPPTransportEnumType,
    pub ocpp_csms_url: String,
    pub message_timeout: i32,
    pub security_profile: i32,
    pub ocpp_interface: OCPPInterfaceEnumType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpn: Option<VPNType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apn: Option<APNType>,
}
