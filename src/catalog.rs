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

#[derive(Serialize, Default, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct MaintenanceInfo {
    version: String,
    description: Option<String>,
}

pub async fn get_catalog(provider: web::Data<CatalogProvider>) -> impl Responder {
    HttpResponse::Ok().json(&provider.catalog)
}

pub struct CatalogProvider {
    plans: HashMap<String, ServicePlan>,
    pub catalog: Catalog,
}

impl CatalogProvider {
    pub fn new() -> Self {
        CatalogProvider {
            plans: HashMap::new(),
            catalog: Catalog::default(),
        }
    }

    pub fn add_plan(&mut self, plan: ServicePlan) -> &Self {
        self.plans.insert(plan.id.clone(), plan);

        self
    }

    pub fn get_plan(&self, plan_id: &str) -> Option<&ServicePlan> {
        self.plans.get(plan_id)
    }

    pub fn add_service(&mut self, service: Service) -> &Self {
        self.catalog.services.push(service);

        self
    }
}
