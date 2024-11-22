use crate::DateTime;
use crate::Precision;
use serde::de;
use serde::ser;
use std::fmt;

/// Serialize a [`DateTime`] instance as an RFC 3339 string.
impl ser::Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str(&self.format_rfc3339(Precision::Seconds))
    }
}

struct DateTimeVisitor;

impl de::Visitor<'_> for DateTimeVisitor {
    type Value = DateTime;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an RFC 3339 formatted date string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        value.parse().map_err(E::custom)
    }
}

/// Deserialize an RFC 3339 string into a [`DateTime`] instance.
impl<'de> de::Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(DateTimeVisitor)
    }
}
