use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
	pub(crate) code: u16,
	pub(crate) message: String,
}


#[derive(Error, Debug)]
pub enum UploadError {
	#[error("Invalid form data model, `{0}`")]
	InvalidFormData(String),
}

impl ResponseError for UploadError {
	fn status_code(&self) -> StatusCode {
		StatusCode::BAD_REQUEST
	}

	fn error_response(&self) -> HttpResponse {
		let status_code = self.status_code();
		let error_response = ErrorResponse {
			code: status_code.as_u16(),
			message: self.to_string(),
		};
		HttpResponse::build(status_code).json(error_response)
	}
}
