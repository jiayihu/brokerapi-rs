use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Default)]
pub struct Catalog {
    pub services: Vec<Service>,
}

/// Service Offering
///
/// <https://github.com/openservicebrokerapi/servicebroker/blob/v2.16/spec.md#service-offering-object>
#[derive(Serialize, Default)]
pub struct Service {
    pub name: String,
    pub id: String,
    pub description: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub requires: Vec<Permission>,
    pub bindable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instances_retrievable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bindings_retrievable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_context_updates: Option<bool>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_updateable: Option<bool>,
    pub plans: Vec<ServicePlan>,
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
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub metadata: HashMap<String, String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bindable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_updateable: Option<bool>,
    // schemas: Option<Schemas>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_polling_duration: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintenance_info: Option<MaintenanceInfo>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct MaintenanceInfo {
    version: String,
    description: Option<String>,
}

pub async fn get_catalog(provider: web::Data<CatalogProvider>) -> impl Responder {
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
    pub catalog: Catalog,
}

impl CatalogProvider {
    pub fn new(catalog: Catalog) -> Self {
        CatalogProvider { catalog }
    }

    pub fn get_catalog(&self) -> &Catalog {
        &self.catalog
    }
}
