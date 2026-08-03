use crate::v1_6::types::ConfigurationStatus;
#[cfg(not(feature = "std"))]
use alloc::string::String;

pub const CHANGE_CONFIGURATION_ACTION: &str = "ChangeConfiguration";

/// # From OCPP Specification
/// If this key exists, the Charge Point supports Unknown Offline Authorization. If this key
/// reports a value of true, Unknown Offline Authorization is enabled.
pub const ALLOW_OFFLINE_TX_FOR_UNKNOWN_ID: &str = "AllowOfflineTxForUnknownId";

/// # From OCPP Specification
/// If this key exists, the Charge Point supports an Authorization Cache. If this key reports a
/// value of true, the Authorization Cache is enabled.
pub const AUTHORIZATION_CACHE_ENABLED: &str = "AuthorizationCacheEnabled";

/// # From OCPP Specification
/// Whether a remote request to start a transaction in the form of a RemoteStartTransaction.req
/// message should be authorized beforehand like a local action to start a transaction.
pub const AUTHORIZE_REMOTE_TX_REQUESTS: &str = "AuthorizeRemoteTxRequests";

/// # From OCPP Specification
/// Number of times to blink Charge Point lighting when signalling
pub const BLINK_REPEAT: &str = "BlinkRepeat";

/// # From OCPP Specification
/// Size (in seconds) of the clock-aligned data interval. This is the size (in seconds) of the set
/// of evenly spaced aggregation intervals per day, starting at 00:00:00 (midnight). For example, a
/// value of 900 (15 minutes) indicates that every day should be broken into 96 15-minute intervals.
///
/// When clock aligned data is being transmitted, the interval in question is identified by the
/// start time and (optional) duration interval value, represented according to the ISO8601
/// standard. All "per-period" data (e.g. energy readings) should be accumulated (for "flow" type
/// measurands such as energy), or averaged (for other values) across the entire interval (or
/// partial interval, at the beginning or end of a Transaction), and transmitted (if so enabled) at
/// the end of each interval, bearing the interval start time timestamp.
/// A value of "0" (numeric zero), by convention, is to be interpreted to mean that no
/// clock-aligned data should be transmitted.
pub const CLOCK_ALIGNED_DATA_INTERVAL: &str = "ClockAlignedDataInterval";

/// # From OCPP Specification
/// Interval *from beginning of status: 'Preparing' until incipient Transaction is automatically
/// canceled, due to failure of EV driver to (correctly) insert the charging cable connector(s)
/// into the appropriate socket(s). The Charge Point SHALL go back to the original state, probably:
/// 'Available'.
pub const CONNECTION_TIME_OUT: &str = "ConnectionTimeOut";

/// # From OCPP Specification
/// The phase rotation per connector in respect to the connector’s electrical meter (or if absent,
/// the grid connection). Possible values per connector are:
///
/// NotApplicable (for Single phase or DC Charge Points)
/// Unknown (not (yet) known)
/// RST (Standard Reference Phasing) RTS (Reversed Reference Phasing) SRT (Reversed 240 degree
/// rotation) STR (Standard 120 degree rotation) TRS (Standard 240 degree rotation) TSR (Reversed
/// 120 degree rotation)
/// R can be identified as phase 1 (L1), S as phase 2 (L2), T as phase 3 (L3).
/// If known, the Charge Point MAY also report the phase rotation between the grid connection and
/// the main energymeter by using index number Zero (0).
/// Values are reported in CSL, formatted: 0.RST, 1.RST, 2.RTS
pub const CONNECTOR_PHASE_ROTATION: &str = "ConnectorPhaseRotation";

/// # From OCPP Specification
/// Maximum number of items in a ConnectorPhaseRotation Configuration Key.
pub const CONNECTOR_PHASE_ROTATION_MAX_LENGTH: &str = "ConnectorPhaseRotationMaxLength";

/// # From OCPP Specification
/// Maximum number of requested configuration keys in a GetConfiguration.req PDU.
pub const GET_CONFIGURATION_MAX_KEYS: &str = "GetConfigurationMaxKeys";

/// # From OCPP Specification
/// Interval of inactivity (no OCPP exchanges) with central system after which the Charge Point
/// should send a Heartbeat.req PDU
pub const HEARTBEAT_INTERVAL: &str = "HeartbeatInterval";

/// # From OCPP Specification
/// Percentage of maximum intensity at which to illuminate Charge Point lighting
pub const LIGHT_INTENSITY: &str = "LightIntensity";

/// # From OCPP Specification
/// whether the Charge Point, when offline, will start a transaction for locally-authorized
/// identifiers.
pub const LOCAL_AUTHORIZE_OFFLINE: &str = "LocalAuthorizeOffline";

/// # From OCPP Specification
/// whether the Charge Point, when online, will start a transaction for locally-authorized
/// identifiers without waiting for or requesting an Authorize.conf from the Central System
pub const LOCAL_PRE_AUTHORIZE: &str = "LocalPreAuthorize";

/// # From OCPP Specification
/// Maximum energy in Wh delivered when an identifier is invalidated by the Central System after
/// start of a transaction.
pub const MAX_ENERGY_ON_INVALID_ID: &str = "MaxEnergyOnInvalidId";

/// # From OCPP Specification
/// Clock-aligned measurand(s) to be included in a MeterValues.req PDU, every
/// ClockAlignedDataInterval seconds
pub const METER_VALUES_ALIGNED_DATA: &str = "MeterValuesAlignedData";

/// # From OCPP Specification
/// Maximum number of items in a MeterValuesAlignedData Configuration Key.
pub const METER_VALUES_ALIGNED_DATA_MAX_LENGTH: &str = "MeterValuesAlignedDataMaxLength";

/// # From OCPP Specification
/// Sampled measurands to be included in a MeterValues.req PDU, every METER_VALUE_SAMPLE_INTERVAL
/// seconds. Where applicable, the Measurand is combined with the optional phase; for instance:
/// Voltage.L1
///
/// Default: "Energy.Active.Import.Register"
pub const METER_VALUES_SAMPLED_DATA: &str = "MeterValuesSampledData";

/// # From OCPP Specification
/// Maximum number of items in a MeterValuesSampledData Configuration Key.
pub const METER_VALUES_SAMPLED_DATA_MAX_LENGTH: &str = "MeterValuesSampledDataMaxLength";

/// # From OCPP Specification
/// Interval between sampling of metering (or other) data, intended to be transmitted by
/// "MeterValues" PDUs. For charging session data (ConnectorId>0), samples are acquired and
/// transmitted periodically at this interval from the start of the charging transaction.
///
/// A value of "0" (numeric zero), by convention, is to be interpreted to mean that no sampled data
/// should be transmitted.
pub const METER_VALUE_SAMPLE_INTERVAL: &str = "MeterValueSampleInterval";

/// # From OCPP Specification
/// The minimum duration that a Charge Point or Connector status is stable before a
/// StatusNotification.req PDU is sent to the Central System.
pub const MINIMUM_STATUS_DURATION: &str = "MinimumStatusDuration";

/// # From OCPP Specification
/// The number of physical charging connectors of this Charge Point.
pub const NUMBER_OF_CONNECTORS: &str = "NumberOfConnectors";

/// # From OCPP Specification
/// Number of times to retry an unsuccessful reset of the Charge Point.
pub const RESET_RETRIES: &str = "ResetRetries";

/// # From OCPP Specification
/// When set to true, the Charge Point SHALL administratively stop the transaction when the cable
/// is unplugged from the EV.
pub const STOP_TRANSACTION_ON_EV_SIDE_DISCONNECT: &str = "StopTransactionOnEVSideDisconnect";

/// # From OCPP Specification
/// whether the Charge Point will stop an ongoing transaction when it receives a non- Accepted
/// authorization status in a StartTransaction.conf for this transaction
pub const STOP_TRANSACTION_ON_INVALID_ID: &str = "StopTransactionOnInvalidId";

/// # From OCPP Specification
/// Clock-aligned periodic measurand(s) to be included in the TransactionData element of
/// StopTransaction.req MeterValues.req PDU for every ClockAlignedDataInterval of the Transaction
pub const STOP_TXN_ALIGNED_DATA: &str = "StopTxnAlignedData";

/// # From OCPP Specification
/// Maximum number of items in a StopTxnAlignedData Configuration Key.
pub const STOP_TXN_ALIGNED_DATA_MAX_LENGTH: &str = "StopTxnAlignedDataMaxLength";

/// # From OCPP Specification
/// Sampled measurands to be included in the TransactionData element of StopTransaction.req PDU,
///every MeterValueSampleInterval seconds from the start of the charging session
pub const STOP_TXN_SAMPLED_DATA: &str = "StopTxnSampledData";

/// # From OCPP Specification
/// Maximum number of items in a StopTxnSampledData Configuration Key.
pub const STOP_TXN_SAMPLED_DATA_MAX_LENGTH: &str = "StopTxnSampledDataMaxLength";

/// # From OCPP Specification
/// A list of supported Feature Profiles. Possible profile identifiers: Core, FirmwareManagement,
/// LocalAuthListManagement, Reservation, SmartCharging and RemoteTrigger.
pub const SUPPORTED_FEATURE_PROFILES: &str = "SupportedFeatureProfiles";

/// # From OCPP Specification
/// Maximum number of items in a SupportedFeatureProfiles Configuration Key.
pub const SUPPORTED_FEATURE_PROFILES_MAX_LENGTH: &str = "SupportedFeatureProfilesMaxLength";

/// # From OCPP Specification
/// How often the Charge Point should try to submit a transaction-related message when the Central
/// System fails to process it.
pub const TRANSACTION_MESSAGE_ATTEMPTS: &str = "TransactionMessageAttempts";

/// # From OCPP Specification
/// How long the Charge Point should wait before resubmitting a transaction-related message that
/// the Central System failed to process.
pub const TRANSACTION_MESSAGE_RETRY_INTERVAL: &str = "TransactionMessageRetryInterval";

/// # From OCPP Specification
/// When set to true, the Charge Point SHALL unlock the cable on Charge Point side when the cable
/// is unplugged at the EV.
pub const UNLOCK_CONNECTOR_ON_EV_SIDE_DISCONNECT: &str = "UnlockConnectorOnEVSideDisconnect";

/// # From OCPP Specification
/// Only relevant for websocket implementations. 0 disables client side websocket Ping/Pong. In
/// this case there is either no ping/pong or the server initiates the ping and client responds
/// with Pong. Positive values are interpreted as number of seconds between pings. Negative values
/// are not allowed. ChangeConfiguration is expected to return a REJECTED result.
pub const WEB_SOCKET_PING_INTERVAL: &str = "WebSocketPingInterval";

/// # From OCPP Specification
/// whether the Local Authorization List is enabled
pub const LOCAL_AUTH_LIST_ENABLED: &str = "LocalAuthListEnabled";

/// # From OCPP Specification
/// Maximum number of identifications that can be stored in the Local Authorization List
pub const LOCAL_AUTH_LIST_MAX_LENGTH: &str = "LocalAuthListMaxLength";

/// # From OCPP Specification
/// Maximum number of identifications that can be send in a single SendLocalList.req
pub const SEND_LOCAL_LIST_MAX_LENGTH: &str = "SendLocalListMaxLength";

/// # From OCPP Specification
/// If this configuration key is present and set to true: Charge Point support reservations on
/// connector 0.
pub const RESERVE_CONNECTOR_ZERO_SUPPORTED: &str = "ReserveConnectorZeroSupported";

/// # From OCPP Specification
/// Max StackLevel of a ChargingProfile. The number defined also indicates the max allowed number
/// of installed charging schedules per Charging Profile Purposes.
pub const CHARGE_PROFILE_MAX_STACK_LEVEL: &str = "ChargeProfileMaxStackLevel";

/// # From OCPP Specification
/// A list of supported quantities for use in a ChargingSchedule. Allowed values: 'Current' and
/// 'Power'
pub const CHARGING_SCHEDULE_ALLOWED_CHARGING_RATE_UNIT: &str =
    "ChargingScheduleAllowedChargingRateUnit";

/// # From OCPP Specification
/// Maximum number of periods that may be defined per ChargingSchedule.
pub const CHARGING_SCHEDULE_MAX_PERIODS: &str = "ChargingScheduleMaxPeriods";

/// # From OCPP Specification
/// If defined and true, this Charge Point support switching from 3 to 1 phase during a Transaction.
pub const CONNECTOR_SWITCH_3_TO_1_PHASE_SUPPORTED: &str = "ConnectorSwitch3to1PhaseSupported";

/// # From OCPP Specification
/// Maximum number of Charging profiles installed at a time
pub const MAX_CHARGING_PROFILES_INSTALLED: &str = "MaxChargingProfilesInstalled";

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
pub struct ChangeConfigurationRequest {
    #[cfg_attr(feature = "std", validate(length(min = 1, max = 50)))]
    pub key: String,
    #[cfg_attr(feature = "std", validate(length(min = 1, max = 500)))]
    pub value: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "std", derive(validator::Validate))]
pub struct ChangeConfigurationResponse {
    pub status: ConfigurationStatus,
}
