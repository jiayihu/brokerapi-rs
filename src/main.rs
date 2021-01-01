mod binding;
mod catalog;
mod errors;
mod polling;
mod service;

use actix_web::middleware::Logger;
use actix_web::{guard, web, App, HttpResponse, HttpServer};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .data(catalog::CatalogProvider::new(catalog::build_catalog()))
            .service(
                web::scope("/v2")
                    .guard(guard::Header("X-Broker-API-Version", "2.16"))
                    .route("/catalog", web::get().to(catalog::get_catalog))
                    
                    .route(
                        "/service_instances/{instance_id}",
                        web::put().to(service::put_service_instance),
                    )
                    .route(
                        "/service_instances/{instance_id}",
                        web::get().to(service::get_service_instance),
                    )
                    .route(
                        "/service_instances/{instance_id}",
                        web::patch().to(service::patch_service_instance),
                    )
                    .route(
                        "/service_instances/{instance_id}",
                        web::delete().to(service::delete_service_instance),
                    )

                    .route(
                        "/service_instances/{instance_id}/service_bindings/{binding_id}",
                        web::put().to(binding::put_binding),
                    )
                    .route(
                        "/service_instances/{instance_id}/service_bindings/{binding_id}",
                        web::get().to(binding::get_binding),
                    )
                    .route(
                        "/service_instances/{instance_id}/service_bindings/{binding_id}",
                        web::delete().to(binding::delete_binding),
                    )

                    .route(
                        "/service_instances/{instance_id}/last_operation",
                        web::get().to(polling::get_service_instance_last_operation),
                    )
                    .route(
                        "/service_instances/{instance_id}/service_bindings/{binding_id}/last_operation",
                        web::get().to(polling::get_service_binding_state),
                    ),
            )
            .default_service(web::route().to(|| HttpResponse::NotFound()))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
