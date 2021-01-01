use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Default)]
pub struct Catalog {
    services: Vec<Service>,
}

/// Service Offering
///
/// https://github.com/openservicebrokerapi/servicebroker/blob/v2.16/spec.md#service-offering-object
#[derive(Serialize, Default)]
pub struct Service {
    name: String,
    id: String,
    description: String,
    tags: Vec<String>,
    requires: Vec<Permission>,
    bindable: bool,
    instances_retrievable: Option<bool>,
    bindings_retrievable: Option<bool>,
    allow_context_updates: Option<bool>,
    metadata: Option<HashMap<String, String>>,
    // dashboard_client: Option<DashboardClient>
    plan_updateable: Option<bool>,
    plans: Vec<ServicePlan>,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
#[allow(unused)]
pub enum Permission {
    SyslogDrain,
    RouteForwarding,
    VolumeMount,
}

#[derive(Serialize, Default)]
pub struct ServicePlan {
    id: String,
    name: String,
    description: String,
    metadata: Option<HashMap<String, String>>,
    free: Option<bool>,
    bindable: Option<bool>,
    plan_updateable: Option<bool>,
    // schemas: Option<Schemas>,
    maximum_polling_duration: Option<u64>,
    maintenance_info: Option<MaintenanceInfo>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct MaintenanceInfo {
    version: String,
    description: Option<String>,
}

pub async fn get_catalog(
    _req: HttpRequest,
    provider: web::Data<CatalogProvider>,
) -> impl Responder {
    HttpResponse::Ok().json(provider.get_catalog())
}

pub fn build_catalog() -> Catalog {
    let mut catalog = Catalog::default();

    let mut free_plan = ServicePlan::default();
    free_plan.id = "free".to_string();
    free_plan.name = "free".to_string();
    free_plan.description = "Free plan".to_string();
    free_plan.maximum_polling_duration = Some(60);

    let mut hello_wasi = Service::default();
    hello_wasi.id = "hellowasi".to_string();
    hello_wasi.name = "hellowasi".to_string();
    hello_wasi.description = "Hello world in WASI".to_string();
    hello_wasi.tags.push("wasi".to_string());
    hello_wasi.requires.push(Permission::VolumeMount);
    hello_wasi.bindable = true;
    hello_wasi.plans.push(free_plan);

    catalog.services.push(hello_wasi);

    catalog
}

pub struct CatalogProvider {
    catalog: Catalog,
}

impl CatalogProvider {
    pub fn new(catalog: Catalog) -> Self {
        CatalogProvider { catalog }
    }

    pub fn get_catalog(&self) -> &Catalog {
        &self.catalog
    }
}
