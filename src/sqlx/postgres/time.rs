use std::mem;

use crate::{util::constants::BUG_MSG, Time};
use sqlx::{
    encode::IsNull,
    error::BoxDynError,
    postgres::{PgArgumentBuffer, PgHasArrayType, PgTypeInfo, PgValueRef},
    Decode, Encode, Postgres, Type,
};

impl Type<Postgres> for Time {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("time")
    }
}

impl PgHasArrayType for Time {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("time[]")
    }
}

impl Encode<'_, Postgres> for Time {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        let micros = (self.as_nanos() / 1_000) as i64;
        Encode::<Postgres>::encode(micros, buf)
    }

    fn size_hint(&self) -> usize {
        mem::size_of::<u64>()
    }
}

impl Decode<'_, Postgres> for Time {
    fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
        let micros: i64 = Decode::<Postgres>::decode(value)?;
        Ok(Time::from_nanos(micros as u64 * 1_000).expect(BUG_MSG))
    }
}
