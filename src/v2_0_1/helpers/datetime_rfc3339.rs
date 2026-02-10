use chrono::{DateTime, SecondsFormat, Utc};
use serde::{Deserialize, Deserializer, Serializer};
pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&date.to_rfc3339_opts(SecondsFormat::Millis, true))
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    DateTime::parse_from_rfc3339(&s)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(serde::de::Error::custom)
}

pub mod option {
    use chrono::{DateTime, SecondsFormat, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(dt) => serializer.serialize_str(&dt.to_rfc3339_opts(SecondsFormat::Millis, true)),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt = Option::<String>::deserialize(deserializer)?;
        match opt {
            Some(s) => DateTime::parse_from_rfc3339(&s)
                .map(|dt| Some(dt.with_timezone(&Utc)))
                .map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};
    use serde_json;

    #[test]
    fn test_serialize_deserialize_datetime() {
        let naive = Utc
            .with_ymd_and_hms(2023, 9, 3, 12, 34, 56)
            .unwrap()
            .naive_utc();
        let naive = naive
            .checked_add_signed(chrono::Duration::milliseconds(789))
            .unwrap();
        let dt: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive, Utc);
        let json = serde_json::to_string(&dt).unwrap();
        let deserialized: DateTime<Utc> = serde_json::from_str(&json).unwrap();
        assert_eq!(dt, deserialized);
    }

    #[test]
    fn test_serialize_deserialize_option_some() {
        let naive = Utc
            .with_ymd_and_hms(2023, 9, 3, 12, 34, 56)
            .unwrap()
            .naive_utc();
        let naive = naive
            .checked_add_signed(chrono::Duration::milliseconds(789))
            .unwrap();
        let dt: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive, Utc);
        let opt_dt = Some(dt);
        let json = serde_json::to_string(&opt_dt).unwrap();
        let deserialized: Option<DateTime<Utc>> =
            option::deserialize(&mut serde_json::Deserializer::from_str(&json)).unwrap();
        assert_eq!(opt_dt, deserialized);
    }

    #[test]
    fn test_serialize_deserialize_option_none() {
        let opt_dt: Option<DateTime<Utc>> = None;
        let json = serde_json::to_string(&opt_dt).unwrap();
        let deserialized: Option<DateTime<Utc>> =
            option::deserialize(&mut serde_json::Deserializer::from_str(&json)).unwrap();
        assert_eq!(opt_dt, deserialized);
    }
}
