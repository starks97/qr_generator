use actix_cors::Cors;
use actix_web::{http::header, web};

use crate::config_secrets;

use super::{health_route, qr};

pub fn config_handler(config: &mut web::ServiceConfig, config_data: &config_secrets::Config) {
    let cors = Cors::default()
        .allowed_origin(&config_data.client_origin)
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH"])
        .allowed_headers(vec![
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::ACCEPT,
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
        ])
        .supports_credentials()
        .max_age(3600);

    let scope = web::scope("/api")
        .service(health_route::health_checker_handler)
        .service(qr::create_qr::create_qr_data)
        .service(qr::create_qr::get_all_qr_data)
        .service(qr::create_qr::get_qr_data)
        .wrap(cors);

    config.service(scope);
}
