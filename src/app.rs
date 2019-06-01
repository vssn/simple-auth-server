use crate::models::DbExecutor;
use crate::routes::auth::{get_me, login, logout};
use crate::routes::invitation::register_email;
use crate::routes::register::register_user;
use actix::prelude::*;
use actix_web::middleware::identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{http::Method, middleware, App};
use chrono::Duration;

pub struct AppState {
    pub db: Addr<DbExecutor>,
}

pub fn create_app(db: Addr<DbExecutor>) -> App<AppState> {
    let secret: String = std::env::var("SECRET_KEY").unwrap_or_else(|_| "0".repeat(32));
    let domain: String = std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());

    App::with_state(AppState { db })
        .middleware(middleware::Logger::default())
        .middleware(IdentityService::new(
            CookieIdentityPolicy::new(secret.as_bytes())
                .name("auth")
                .path("/")
                .domain(domain.as_str())
                .max_age(Duration::days(1))
                .secure(false),
        ))
        .resource("/auth", |r| {
            r.method(Method::GET).with(get_me);
            r.method(Method::POST).with(login);
            r.method(Method::DELETE).with(logout);
        })
        .resource("/invitation", |r| {
            r.method(Method::POST).with(register_email);
        })
        .resource("/register/{invitation_id}", |r| {
            r.method(Method::POST).with(register_user);
        })
}
