use actix_web::middleware::Logger;
use actix_web::{guard, web, App, HttpResponse, HttpServer};
use brokerapi::{binding, catalog, polling, service};

const LOG_DEFAULT: &str = "%a %t %r %s %Ts";
#[allow(unused)]
const LOG_HEADERS: &str = "%a %t %r %s %Ts
            Headers:
                X-Broker-API-Version: %{X-Broker-API-Version}i
                X-Broker-API-Originating-Identity: %{X-Broker-API-Originating-Identity}i
                X-Broker-API-Request-Identity: %{X-Broker-API-Request-Identity}i
";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    simple_logging::log_to_file("brokerapi.log", log::LevelFilter::Info)?;

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new(LOG_DEFAULT))
            .data(catalog::CatalogProvider::new(catalog::build_catalog()))
            .service(
                web::scope("/v2")
                    .guard(guard::fn_guard(|req| {
                        match req.headers().get("X-Broker-API-Version") {
                            Some(value) => value
                                .to_str().ok()
                                .and_then(|version| version.parse::<f32>().ok())
                                .map_or(false, |version| {
                                    version >= 2.14
                                }),
                            None => false
                        }
                    }))
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
