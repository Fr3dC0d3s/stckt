use crate::api::{node_controller, ping_controller};
use actix_web::web;

pub fn config_services(cfg: &mut web::ServiceConfig) {
    info!("Configuration routes on the node...");
    cfg.service(
        web::scope("/api").service(ping_controller::ping).service(
            web::scope("/nodes")
                .service(web::resource("")
                    .route(web::post().to(node_controller::insert))
                    .route(web::get().to(node_controller::find_all))),
        ),
    );
}
