use std::mem;

use crate::{util::constants::BUG_MSG, Date, DateUtilities};
use sqlx::{
    encode::IsNull,
    error::BoxDynError,
    postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef},
    Decode, Encode, Postgres, Type,
};

impl Type<Postgres> for Date {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("DATE")
    }
}

impl Encode<'_, Postgres> for Date {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        let days = self.days_since(&postgres_epoch_date()) as i32;
        Encode::<Postgres>::encode(days, buf)
    }

    fn size_hint(&self) -> usize {
        mem::size_of::<i32>()
    }
}

impl Decode<'_, Postgres> for Date {
    fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
        let days: i64 = Decode::<Postgres>::decode(value)?;

        let date = if days >= 0 {
            postgres_epoch_date().add_days(days.unsigned_abs().try_into().expect(BUG_MSG))
        } else {
            postgres_epoch_date().sub_days(days.unsigned_abs().try_into().expect(BUG_MSG))
        };

        Ok(date)
    }
}

fn postgres_epoch_date() -> Date {
    Date::from_ymd(2000, 1, 1).expect(BUG_MSG)
}
