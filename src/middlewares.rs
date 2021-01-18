use std::task::{Context, Poll};
use actix_identity::{RequestIdentity};
use actix_service::{Service, Transform};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{Error, HttpResponse};
use futures::future::{ok, Either, Ready};

pub struct CheckLogin;

impl<S, B> Transform<S> for CheckLogin
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = CheckLoginMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(CheckLoginMiddleware { service })
    }
}

fn not_auth(path: &str) -> bool {
    let paths = ["/static", "/api-open"];
    paths.iter().any(|x| path.starts_with(x))
}

pub struct CheckLoginMiddleware<S> {
    service: S,
}


impl<S, B> Service for CheckLoginMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        if not_auth(req.path()) {
            return Either::Left(self.service.call(req));
        };
        let token = req.get_identity();
        if token.is_some() {
            Either::Left(self.service.call(req))
        } else {
            Either::Right(ok(req.into_response(
                HttpResponse::Unauthorized()
                    .finish()
                    .into_body(),
            )))
        }
    }
}