use crate::GuardError;

use std::future::Future;
use std::pin::Pin;

use actix_web::dev::ServiceRequest;
use actix_web::{FromRequest, Handler, ResponseError};

pub trait Guard<Args: FromRequest, Error: ResponseError>:
    Handler<Args, Output = Result<(), Error>>
{
    fn call_guard_with_service_request<'a, 'b: 'a>(
        &'a self,
        req: &'b mut ServiceRequest,
    ) -> Pin<Box<dyn Future<Output = Result<(), GuardError<Error>>> + 'a>> {
        let (mut_req, mut payload) = req.parts_mut();

        // waiting for async functions in traits <https://github.com/rust-lang/rust/issues/91611>
        Box::pin(async move {
            match Args::from_request(&mut_req, &mut payload).await {
                Ok(args) => self
                    .call(args)
                    .await
                    .map_err(|err| GuardError::GuardCall(err)),
                Err(err) => Err(GuardError::FromRequest(err.into())),
            }
        })
    }
}

impl<T, Args, Error> Guard<Args, Error> for T
where
    T: Handler<Args, Output = Result<(), Error>>,
    Args: FromRequest,
    Error: ResponseError,
{
}
