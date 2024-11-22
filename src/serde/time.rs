use crate::Time;
use serde::de;
use serde::ser;
use std::fmt;

/// Serialize a [`Time`] instance as `HH:mm:ss`.
impl ser::Serialize for Time {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str(&self.format("HH:mm:ss"))
    }
}

struct TimeVisitor;

impl de::Visitor<'_> for TimeVisitor {
    type Value = Time;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a formatted date string in the format `HH:mm:ss`")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        value.parse().map_err(E::custom)
    }
}

/// Deserialize a `HH:mm:ss` formatted string into a [`Time`] instance.
impl<'de> de::Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(TimeVisitor)
    }
}
