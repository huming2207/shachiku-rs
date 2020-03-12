use actix_web::web;
use crate::controller::auth;

pub fn load_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").service(
            web::scope("/auth").service(
                web::resource("/login").route(web::post().to(auth::login))
            ).service(
                web::resource("/register").route(web::post().to(auth::register))
            )
        )
    );
}