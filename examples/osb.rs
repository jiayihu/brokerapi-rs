use actix_web::middleware::Logger;
use actix_web::{guard, web, App, HttpResponse, HttpServer};
use brokerapi::{binding, catalog, polling, service};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    simple_logging::log_to_file("brokerapi.log", log::LevelFilter::Info)?;

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("%a %t %r %s %Ts"))
            .data(catalog::CatalogProvider::new(catalog::build_catalog()))
            .service(
                web::scope("/v2")
                    .guard(guard::Header("X-Broker-API-Version", "2.16"))
                    .route("/catalog", web::get().to(catalog::get_catalog))
                    .service(
                web::resource("/service_instances/{instance_id}")
                            .route(web::put().to(service::put_service_instance))
                            .route(web::get().to(service::get_service_instance))
                            .route(web::patch().to(service::patch_service_instance))
                            .route(web::delete().to(service::delete_service_instance))
                    )
                    .service(
                web::resource("/service_instances/{instance_id}/service_bindings/{binding_id}")
                            .route(web::put().to(binding::put_binding))
                            .route(web::get().to(binding::get_binding))
                            .route(web::delete().to(binding::delete_binding))
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
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
