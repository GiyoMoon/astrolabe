#[cfg(feature = "sqlx-postgres")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlx-postgres")))]
mod postgres;

#[cfg(feature = "sqlx-sqlite")]
#[cfg_attr(docsrs, doc(cfg(feature = "sqlx-sqlite")))]
mod sqlite;
