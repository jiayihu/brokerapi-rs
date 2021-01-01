use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

use crate::catalog::MaintenanceInfo;

/// https://github.com/openservicebrokerapi/servicebroker/blob/v2.16/spec.md#parameters-2
#[derive(Deserialize)]
#[allow(unused)]
pub struct ProvisionParams {
    accepts_incomplete: Option<bool>,
}

/// https://github.com/openservicebrokerapi/servicebroker/blob/v2.16/spec.md#body-3
#[derive(Deserialize)]
#[allow(unused)]
pub struct ServiceInstanceRequestBody {
    service_id: String,
    plan_id: String,
    context: Option<HashMap<String, String>>,
    organization_guid: String,
    space_guid: String,
    parameters: Option<HashMap<String, String>>,
    maintenance_info: Option<MaintenanceInfo>,
}

/// https://github.com/openservicebrokerapi/servicebroker/blob/v2.16/spec.md#body-4
#[derive(Serialize, Default)]
#[allow(unused)]
pub struct CreatedServiceIstance {
    dashboard_url: Option<String>,
    operation: Option<String>,
    metadata: Option<ServiceInstanceMetadata>,
}

#[derive(Serialize, Default)]
#[allow(unused)]
pub struct ServiceInstanceMetadata {
    labels: HashMap<String, String>,
}

pub async fn put_service_instance(
    _instance_id: web::Path<String>,
    web::Query(_query): web::Query<ProvisionParams>,
    web::Json(_info): web::Json<ServiceInstanceRequestBody>,
) -> impl Responder {
    HttpResponse::Created().json(CreatedServiceIstance::default())
}

/// https://github.com/openservicebrokerapi/servicebroker/blob/v2.16/spec.md#parameters-3
#[derive(Deserialize)]
#[allow(unused)]
pub struct ServiceFetchParams {
    service: Option<String>,
    plan_id: Option<String>,
}

/// https://github.com/openservicebrokerapi/servicebroker/blob/v2.16/spec.md#body-5
#[derive(Serialize, Default)]
#[allow(unused)]
pub struct ServiceIstance {
    service_id: Option<String>,
    dashboard_url: Option<String>,
    operation: Option<String>,
    metadata: Option<ServiceInstanceMetadata>,
}

pub async fn get_service_instance(
    _instance_id: web::Path<String>,
    web::Query(_query): web::Query<ServiceFetchParams>,
) -> impl Responder {
    HttpResponse::Ok().json(ServiceIstance::default())
}

/// https://github.com/openservicebrokerapi/servicebroker/blob/v2.16/spec.md#body-7
#[derive(Deserialize)]
#[allow(unused)]
pub struct ServiceUpdateRequestBody {
    service_id: String,
    plan_id: Option<String>,
    context: Option<HashMap<String, String>>,
    parameters: Option<HashMap<String, String>>,
    previous_values: Option<PreviousValues>,
    maintenance_info: Option<MaintenanceInfo>,
}

#[derive(Deserialize)]
#[allow(unused)]
pub struct PreviousValues {
    service_id: Option<String>,
    plan_id: Option<String>,
    organization_id: Option<String>,
    space_id: Option<String>,
    maintenance_info: Option<MaintenanceInfo>,
}

pub async fn patch_service_instance(
    _instance_id: web::Path<String>,
    web::Query(_query): web::Query<ProvisionParams>,
    web::Json(_info): web::Json<ServiceUpdateRequestBody>,
) -> impl Responder {
    HttpResponse::Ok().json(CreatedServiceIstance::default())
}

/// https://github.com/openservicebrokerapi/servicebroker/blob/v2.16/spec.md#parameters-8
#[derive(Deserialize)]
#[allow(unused)]
pub struct ServiceDeleteParams {
    service_id: String,
    plan_id: String,
    accepts_incomplete: Option<bool>,
}

pub async fn delete_service_instance(
    _instance_id: web::Path<String>,
    web::Query(_query): web::Query<ServiceDeleteParams>,
) -> impl Responder {
    HttpResponse::Ok().json(json!({}))
}
