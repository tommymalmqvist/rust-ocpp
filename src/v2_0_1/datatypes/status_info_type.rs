/// Element providing more information about the status. StatusInfoType is used by: Common:ClearMonitoringResultType , BootNotificationResponse , CancelReservationResponse, TriggerMessageResponse , UnlockConnectorResponse , UpdateFirmwareResponse , ClearDisplayMessageResponse , Get15118EVCertificateResponse , GetCompositeScheduleResponse , ChangeAvailabilityResponse , GetLogResponse , ClearChargingProfileResponse , NotifyEVChargingNeedsResponse , ClearCacheResponse , NotifyEVChargingScheduleResponse , RequestStartTransactionResponse , RequestStopTransactionResponse , SetChargingProfileResponse , SetDisplayMessageResponse , SetNetworkProfileResponse , SignCertificateResponse , DataTransferResponse ,CertificateSignedResponse , DeleteCertificateResponse , GetChargingProfilesResponse , GetInstalledCertificateIdsResponse ,InstallCertificateResponse , GetBaseReportResponse , GetMonitoringReportResponse , GetReportResponse ,GetVariablesResponse.GetVariableResultType , ReserveNowResponse , SetMonitoringBaseResponse ,SetMonitoringLevelResponse , SetVariableMonitoringResponse.SetMonitoringResultType ,SetVariablesResponse.SetVariableResultType , PublishFirmwareResponse , GetCertificateStatusResponse , ResetResponse ,GetDisplayMessagesResponse , CustomerInformationResponse, SendLocalListResponse
#[cfg(not(feature = "std"))]
use alloc::string::String;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct StatusInfoType {
    pub reason_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
}
