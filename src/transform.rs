use crate::middleware::StateGuardMiddleware;
use crate::Guard as GuardTrait;

use std::future::{ready, Ready};
use std::marker::PhantomData;
use std::rc::Rc;

use actix_web::body::MessageBody;
use actix_web::dev::{Service as ServiceTrait, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error as ActixWebError, FromRequest, ResponseError};

pub struct StateGuard<Guard, Args, Error> {
    guard: Rc<Guard>,
    args_marker: PhantomData<Args>,
    error_marker: PhantomData<Error>,
}

impl<Guard, Args, Error> StateGuard<Guard, Args, Error> {
    pub fn new(guard: Guard) -> Self {
        Self {
            guard: Rc::new(guard),
            args_marker: PhantomData,
            error_marker: PhantomData,
        }
    }
}

impl<Service, Body, Guard, Args, Error> Transform<Service, ServiceRequest>
    for StateGuard<Guard, Args, Error>
where
    Service: ServiceTrait<ServiceRequest, Response = ServiceResponse<Body>, Error = ActixWebError>
        + 'static,
    Body: MessageBody,
    Guard: GuardTrait<Args, Error>,
    Error: ResponseError + 'static,
    Args: FromRequest,
{
    type Response = <StateGuardMiddleware<Service, Guard, Args, Error> as ServiceTrait<
        ServiceRequest,
    >>::Response;

    type Error =
        <StateGuardMiddleware<Service, Guard, Args, Error> as ServiceTrait<ServiceRequest>>::Error;

    type Transform = StateGuardMiddleware<Service, Guard, Args, Error>;

    type InitError = ();

    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: Service) -> Self::Future {
        ready(Ok(StateGuardMiddleware::new(
            Rc::clone(&self.guard),
            service,
        )))
    }
}
