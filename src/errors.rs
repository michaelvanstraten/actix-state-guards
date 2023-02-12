use std::fmt::Display;

use actix_web::{Error as ActixWebError, ResponseError};

pub type GuardResult<GuardCallErr> = Result<(), GuardError<GuardCallErr>>;

#[derive(Debug)]
pub enum GuardError<GuardCallErr> {
    FromRequest(ActixWebError),
    GuardCall(GuardCallErr),
}

impl<GuardCallErr> Display for GuardError<GuardCallErr>
where
    GuardCallErr: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GuardError::FromRequest(from_request_error) => from_request_error.fmt(f),
            GuardError::GuardCall(guard_call_error) => guard_call_error.fmt(f),
        }
    }
}

impl<GuardCallErr> ResponseError for GuardError<GuardCallErr>
where
    GuardCallErr: ResponseError,
{
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            GuardError::FromRequest(from_request_error) => from_request_error.as_response_error().status_code(),
            GuardError::GuardCall(guard_call_error) => guard_call_error.status_code(),
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        match self {
            GuardError::FromRequest(from_request_error) => from_request_error.error_response(),
            GuardError::GuardCall(guard_call_error) => guard_call_error.error_response(),
        }
    }
}
