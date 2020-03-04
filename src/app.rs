use actix_web::web;

pub fn load_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").service(
            web::scope("/auth").service(
                web::resource("/login").route(web::post().to(crate::controller::auth::login))
            ).service(
                web::resource("/register").route(web::post().to(crate::controller::auth::register))
            )
        )
    );
}