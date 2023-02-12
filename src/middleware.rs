use crate::Guard as GuardTrait;

use std::marker::PhantomData;
use std::rc::Rc;
use std::{future::Future, pin::Pin};

use actix_web::body::MessageBody;
use actix_web::dev::{forward_ready, Service as ServiceTrait, ServiceRequest, ServiceResponse};
use actix_web::{Error as ActixWebError, FromRequest, ResponseError};

pub struct StateGuardMiddleware<Service, Guard, Args, Error> {
    service: Rc<Service>,
    guard: Rc<Guard>,
    args_marker: PhantomData<Args>,
    error_marker: PhantomData<Error>,
}

impl<Service, Guard, Args, Error> StateGuardMiddleware<Service, Guard, Args, Error> {
    pub(crate) fn new(
        guard: Rc<Guard>,
        service: Service,
    ) -> StateGuardMiddleware<Service, Guard, Args, Error> {
        Self {
            guard,
            service: Rc::new(service),
            args_marker: PhantomData,
            error_marker: PhantomData,
        }
    }
}

impl<Service, Body, Guard, Args, Error> ServiceTrait<ServiceRequest>
    for StateGuardMiddleware<Service, Guard, Args, Error>
where
    Service: ServiceTrait<ServiceRequest, Response = ServiceResponse<Body>, Error = ActixWebError>
        + 'static,
    Body: MessageBody,
    Guard: GuardTrait<Args, Error>,
    Error: ResponseError + 'static,
    Args: FromRequest,
{
    type Response = Service::Response;

    type Error = Service::Error;

    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        let guard = Rc::clone(&self.guard);

        Box::pin(async move {
            match guard.call_guard_with_service_request(&mut req).await {
                Ok(()) => service.call(req).await,
                Err(err) => Err(err.into()),
            }
        })
    }
}
