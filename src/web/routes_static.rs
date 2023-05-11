use crate::conf;
use axum::handler::HandlerWithoutStateExt;
use axum::http::StatusCode;
use axum::routing::{any_service, MethodRouter};
use tower_http::services::ServeDir;

// See: https://github.com/tokio-rs/axum/issues/1931#issuecomment-1506067949
pub fn serve_dir() -> MethodRouter {
	async fn handle_404() -> (StatusCode, &'static str) {
		(StatusCode::NOT_FOUND, "Not found")
	}

	any_service(
		ServeDir::new(&conf().WEB_FOLDER)
			.not_found_service(handle_404.into_service()),
	)
}
