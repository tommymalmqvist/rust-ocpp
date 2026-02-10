#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
pub enum UploadLogStatusEnumType {
    #[default]
    #[serde(rename = "BadMessage")]
    BadMessage,
    #[serde(rename = "Idle")]
    Idle,
    #[serde(rename = "NotSupportedOperation")]
    NotSupportedOperation,
    #[serde(rename = "PermissionDenied")]
    PermissionDenied,
    #[serde(rename = "Uploaded")]
    Uploaded,
    #[serde(rename = "UploadFailure")]
    UploadFailure,
    #[serde(rename = "Uploading")]
    Uploading,
    #[serde(rename = "AcceptedCanceled")]
    AcceptedCanceled,
}
