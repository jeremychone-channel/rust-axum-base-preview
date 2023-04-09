use crate::{model, web};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
	// -- Conf
	ConfMissingEnv(&'static str),
	ConfWrongFormat(&'static str),

	// -- Sub-Modules
	Web(web::Error),
	Crypt(String),
	Ctx(crate::ctx::Error),

	// -- Model errors.
	Model(model::Error),

	// -- Utils
	FailToB64UDecode,
	DateFailParse(String),

	// -- Conf
	FailToLoadConf(&'static str),
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

// region:    --- Error Froms
impl From<crate::crypt::Error> for Error {
	fn from(val: crate::crypt::Error) -> Self {
		Error::Crypt(val.to_string())
	}
}

impl From<crate::model::Error> for Error {
	fn from(val: crate::model::Error) -> Self {
		Error::Model(val)
	}
}
// endregion: --- Error Froms

// region:    --- Axum IntoResponse
impl IntoResponse for Error {
	fn into_response(self) -> Response {
		println!("->> {:<12} - {self:?}", "INTO_RES");

		// Create a placeholder Axum reponse.
		let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

		// Insert the Error into the reponse.
		response.extensions_mut().insert(self);

		response
	}
}
// endregion: --- Axum IntoResponse

impl Error {
	pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
		println!("->> client_status_and_error {self:?}");
		match self {
			// -- Web
			Self::Web(web::Error::LoginFail) => {
				(StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL)
			}
			Self::Web(web::Error::CtxAuth(_)) => {
				(StatusCode::FORBIDDEN, ClientError::NO_AUTH)
			}
			Self::Web(web::Error::Model(model::Error::EntityNotFound {
				typ,
				id,
			})) => (
				StatusCode::BAD_REQUEST,
				ClientError::EntityNotFound { typ, id: *id },
			),

			// -- Fallback.
			_ => (
				StatusCode::INTERNAL_SERVER_ERROR,
				ClientError::SERVICE_ERROR,
			),
		}
	}
}

#[derive(Debug, strum_macros::AsRefStr)]
#[allow(non_camel_case_types)]
pub enum ClientError {
	LOGIN_FAIL,
	NO_AUTH,
	EntityNotFound { typ: &'static str, id: i64 },
	INVALID_PARAMS,
	SERVICE_ERROR,
}
