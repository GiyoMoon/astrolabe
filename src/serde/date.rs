use crate::Date;
use serde::de;
use serde::ser;
use std::fmt;

/// Serialize a [`Date`] instance as `yyyy-MM-dd`.
impl ser::Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str(&self.format("yyyy-MM-dd"))
    }
}

struct DateVisitor;

impl de::Visitor<'_> for DateVisitor {
    type Value = Date;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a formatted date string in the format `yyyy-MM-dd`")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        value.parse().map_err(E::custom)
    }
}

/// Deserialize a `yyyy-MM-dd` formatted string into a [`Date`] instance.
impl<'de> de::Deserialize<'de> for Date {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(DateVisitor)
    }
}
