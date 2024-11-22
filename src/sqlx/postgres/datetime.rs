use std::{mem, time::Duration};

use crate::{util::constants::BUG_MSG, DateTime, TimeUtilities};
use sqlx::{
    encode::IsNull,
    error::BoxDynError,
    postgres::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueRef},
    Decode, Encode, Postgres, Type,
};

impl Type<Postgres> for DateTime {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("timestamp")
    }
}

impl PgHasArrayType for DateTime {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("timestamp[]")
    }
}

impl Encode<'_, Postgres> for DateTime {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        let us: i64 = self
            .clear_until_nano()
            .micros_since(&postgres_epoch_datetime())
            .try_into()
            .expect(BUG_MSG);

        Encode::<Postgres>::encode(us, buf)
    }

    fn size_hint(&self) -> usize {
        mem::size_of::<i64>()
    }
}

impl Decode<'_, Postgres> for DateTime {
    fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
        let us: i64 = Decode::<Postgres>::decode(value)?;

        let datetime = if us >= 0 {
            postgres_epoch_datetime() + Duration::from_micros(us.unsigned_abs())
        } else {
            postgres_epoch_datetime() - Duration::from_micros(us.unsigned_abs())
        };

        Ok(datetime)
    }
}

fn postgres_epoch_datetime() -> DateTime {
    DateTime::from_ymd(2000, 1, 1).expect(BUG_MSG)
}
