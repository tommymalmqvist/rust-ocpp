//! `no_std` (alloc-only) compatible serde helpers that (de)serialize [`Decimal`]
//! values as arbitrary-precision JSON numbers, mirroring the wire format produced
//! by `rust_decimal::serde::arbitrary_precision` / `arbitrary_precision_option`.
//!
//! `rust_decimal`'s own `serde-with-arbitrary-precision` feature unconditionally
//! forces on `serde_json/std` (via `serde_json/arbitrary_precision`), which is
//! incompatible with a `no_std` build. This module reproduces the same wire
//! format - a JSON *number* token (not a quoted string) with full precision
//! preserved - using `serde_json`'s `raw_value` feature instead, which has no
//! `std` requirement.
//!
//! Used automatically instead of the upstream helper when the `std` feature is
//! disabled; see the `#[cfg_attr(feature = "std", ...)]` / `#[cfg_attr(not(feature =
//! "std"), ...)]` pairs on `Decimal` fields throughout `v1_6`, `v2_0_1` and
//! `wip_v2_1` datatypes.

use alloc::string::ToString;
use core::str::FromStr;
use rust_decimal::Decimal;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::value::RawValue;

/// Parses the raw JSON text captured for a `Decimal` field - either a bare
/// number token (`123.400`) or a quoted string (`"123.400"`) - into a
/// [`Decimal`], mirroring `rust_decimal`'s own `DecimalVisitor` fallback logic
/// (`Decimal::from_str`, falling back to `Decimal::from_scientific`).
fn parse_raw_decimal<E: serde::de::Error>(raw: &str) -> Result<Decimal, E> {
    let text = if let Some(inner) = raw.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
        inner
    } else {
        raw
    };

    Decimal::from_str(text)
        .or_else(|_| Decimal::from_scientific(text))
        .map_err(|_| E::custom(alloc::format!("invalid decimal value: {text}")))
}

/// Drop-in `no_std` replacement for `rust_decimal::serde::arbitrary_precision`.
pub fn serialize<S>(value: &Decimal, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let raw = RawValue::from_string(value.to_string()).map_err(serde::ser::Error::custom)?;
    raw.serialize(serializer)
}

/// Drop-in `no_std` replacement for `rust_decimal::serde::arbitrary_precision`.
pub fn deserialize<'de, D>(deserializer: D) -> Result<Decimal, D::Error>
where
    D: Deserializer<'de>,
{
    let raw = <&RawValue>::deserialize(deserializer)?;
    parse_raw_decimal(raw.get())
}

/// Drop-in `no_std` replacement for `rust_decimal::serde::arbitrary_precision_option`.
///
/// Use via `#[serde(with = "crate::helpers::decimal_arbitrary_precision::option")]`
/// on `Option<Decimal>` fields.
pub mod option {
    use super::{
        parse_raw_decimal, Decimal, Deserialize, Deserializer, RawValue, Serializer, ToString,
    };

    pub fn serialize<S>(value: &Option<Decimal>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(value) => {
                let raw =
                    RawValue::from_string(value.to_string()).map_err(serde::ser::Error::custom)?;
                serializer.serialize_some(&raw)
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Decimal>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw: Option<&RawValue> = Option::deserialize(deserializer)?;
        match raw {
            Some(raw) => parse_raw_decimal(raw.get()).map(Some),
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::str::FromStr;

    #[derive(Serialize, Deserialize)]
    struct Wrapper {
        #[serde(with = "super")]
        value: Decimal,
    }

    #[derive(Serialize, Deserialize)]
    struct OptionWrapper {
        #[serde(
            with = "super::option",
            skip_serializing_if = "Option::is_none",
            default
        )]
        value: Option<Decimal>,
    }

    #[test]
    fn round_trips_option_decimal() {
        let value = Decimal::from_str("42.5").unwrap();
        let wrapper = OptionWrapper { value: Some(value) };
        let json = serde_json::to_string(&wrapper).unwrap();
        assert_eq!(json, r#"{"value":42.5}"#);

        let round_tripped: OptionWrapper = serde_json::from_str(&json).unwrap();
        assert_eq!(round_tripped.value, Some(value));

        let none_wrapper = OptionWrapper { value: None };
        let json = serde_json::to_string(&none_wrapper).unwrap();
        assert_eq!(json, r#"{}"#);
        let round_tripped: OptionWrapper = serde_json::from_str(&json).unwrap();
        assert_eq!(round_tripped.value, None);
    }

    #[test]
    fn round_trips_high_precision_decimal() {
        let value = Decimal::from_str("123.456789012345678").unwrap();
        let wrapper = Wrapper { value };
        let json = serde_json::to_string(&wrapper).unwrap();
        assert_eq!(json, r#"{"value":123.456789012345678}"#);

        let round_tripped: Wrapper = serde_json::from_str(&json).unwrap();
        assert_eq!(round_tripped.value, value);
    }

    #[test]
    fn matches_std_arbitrary_precision_output() {
        let value = Decimal::from_str("123.456789012345678").unwrap();

        #[derive(Serialize)]
        struct StdWrapper {
            #[cfg_attr(
                feature = "std",
                serde(with = "rust_decimal::serde::arbitrary_precision")
            )]
            #[cfg_attr(
                not(feature = "std"),
                serde(with = "crate::helpers::decimal_arbitrary_precision")
            )]
            value: Decimal,
        }

        let std_json = serde_json::to_string(&StdWrapper { value }).unwrap();
        let no_std_json = serde_json::to_string(&Wrapper { value }).unwrap();
        assert_eq!(std_json, no_std_json);
    }

    #[test]
    fn deserializes_quoted_string_form() {
        let json = r#"{"value":"1.1234127836128763"}"#;
        let wrapper: Wrapper = serde_json::from_str(json).unwrap();
        assert_eq!(
            wrapper.value,
            Decimal::from_str("1.1234127836128763").unwrap()
        );
    }
}
