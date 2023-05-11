use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
	EntityNotFound { entity: &'static str, id: i64 },
	UserAlreadyExists { username: String },

	DbFailToCreatePool(String),

	// -- Sub-Modules
	Crypt(String),
	Store(String),

	// -- Dev
	DevFailInitDb(String),

	// -- External
	Io(String),

	Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
}

// region:    --- Error Boilerplate
impl std::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut std::fmt::Formatter,
	) -> core::result::Result<(), std::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate

// region:    --- Froms
impl From<crate::crypt::Error> for Error {
	fn from(val: crate::crypt::Error) -> Self {
		Error::Crypt(val.to_string())
	}
}
impl From<sqlx::Error> for Error {
	fn from(val: sqlx::Error) -> Self {
		Error::Sqlx(val)
	}
}
impl From<std::io::Error> for Error {
	fn from(val: std::io::Error) -> Self {
		Error::Io(val.to_string())
	}
}
// endregion: --- Froms
