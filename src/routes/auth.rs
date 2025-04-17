use actix_web::{web, HttpResponse, Scope};
use crate::handlers::auth_handler::login_check;
use crate::handlers::auth_handler::logout;


pub fn init_routes(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/auth")
            .route("/login-check", web::post().to(login_check))
            .route("/logout", web::post().to(logout))
    );
}