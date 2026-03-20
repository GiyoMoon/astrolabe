use std::borrow::Cow;

use crate::DateTime;
use sqlx::{
    encode::IsNull,
    error::BoxDynError,
    sqlite::{SqliteArgumentValue, SqliteTypeInfo, SqliteValueRef},
    Decode, Encode, Sqlite, Type,
};

impl Type<Sqlite> for DateTime {
    fn type_info() -> SqliteTypeInfo {
        <str as Type<Sqlite>>::type_info()
    }
}

impl Encode<'_, Sqlite> for DateTime {
    fn encode_by_ref(&self, buf: &mut Vec<SqliteArgumentValue<'_>>) -> Result<IsNull, BoxDynError> {
        buf.push(SqliteArgumentValue::Text(Cow::Owned(
            self.format("yyyy-MM-dd HH:mm:ss"),
        )));
        Ok(IsNull::No)
    }
}

impl<'r> Decode<'r, Sqlite> for DateTime {
    fn decode(value: SqliteValueRef<'r>) -> Result<Self, BoxDynError> {
        let text: &str = Decode::<Sqlite>::decode(value)?;
        Ok(DateTime::parse(text, "yyyy-MM-dd HH:mm:ss")?)
    }
}
