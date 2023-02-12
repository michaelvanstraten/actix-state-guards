use crate::Guard as GuardTrait;
use crate::StateGuard;

use actix_web::dev::{ServiceFactory, ServiceRequest};
use actix_web::{App, Error as ActixWebError, FromRequest, ResponseError, Scope};

macro_rules! impl_use_jwt_rest_for {
    ($type:ident, $trait_name:ident) => {
        pub trait $trait_name<Guard, Args, Error> {
            fn use_state_guard(self, guard: Guard, scope: Scope) -> Self;
        }

        impl<Guard, Args, Error, T> $trait_name<Guard, Args, Error> for $type<T>
        where
            T: ServiceFactory<ServiceRequest, Config = (), Error = ActixWebError, InitError = ()>,
            Guard: GuardTrait<Args, Error>,
            Error: ResponseError + 'static,
            Args: FromRequest + 'static,
        {
            fn use_state_guard(self, guard: Guard, scope: Scope) -> Self {
                self.service(scope.wrap(StateGuard::new(guard)))
            }
        }
    };
}

impl_use_jwt_rest_for!(App, UseStateGuardOnApp);
impl_use_jwt_rest_for!(Scope, UseStateGuardOnScope);
