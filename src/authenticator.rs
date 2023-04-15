use std::future::{Ready, ready};

use actix_web::{FromRequest, HttpRequest, dev::Payload, Error as ActixError, http::header::AUTHORIZATION, error::ErrorUnauthorized};

pub struct AuthMiddleware {

}

impl AuthMiddleware {
    fn is_valid(token: &str) -> bool {
        if (token == "access") {
            return true;
        }
        return false; 
    }
}
impl FromRequest for AuthMiddleware {
    type Error = ActixError;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        // TODO: implement proper auth 
        let token = match req.headers().get(AUTHORIZATION) {
            Some(t) => { t.to_str().unwrap() },
            None => { "" }
        };    

        if !AuthMiddleware::is_valid(token) {
            return ready(Err(ErrorUnauthorized("err")))
        }

        ready(Ok(AuthMiddleware{}))
    }
} 