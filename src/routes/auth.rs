use crate::app::AppState;
use crate::handler::auth::{AuthData, LoggedUser};
use crate::utils::create_token;
use actix_web::middleware::identity::RequestIdentity;
use actix_web::{AsyncResponder, FutureResponse, HttpRequest, HttpResponse, Json, ResponseError};
use futures::future::Future;

pub fn login(
    (auth_data, req): (Json<AuthData>, HttpRequest<AppState>),
) -> FutureResponse<HttpResponse> {
    req.state()
        .db
        .send(auth_data.into_inner())
        .from_err()
        .and_then(move |res| match res {
            Ok(slim_user) => {
                let token = create_token(&slim_user)?;
                req.remember(token);
                Ok(HttpResponse::Ok().into())
            }
            Err(err) => Ok(err.error_response()),
        })
        .responder()
}

pub fn logout(req: HttpRequest<AppState>) -> HttpResponse {
    req.forget();
    HttpResponse::Ok().into()
}

pub fn get_me(logged_user: LoggedUser) -> HttpResponse {
    HttpResponse::Ok().json(logged_user)
}
