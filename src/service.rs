use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

use crate::catalog::MaintenanceInfo;

/// <https://github.com/openservicebrokerapi/servicebroker/blob/v2.16/spec.md#parameters-2>
#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct ProvisionParams {
    pub accepts_incomplete: Option<bool>,
}

/// <https://github.com/openservicebrokerapi/servicebroker/blob/v2.16/spec.md#body-3>
#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct ServiceInstanceRequestBody {
    pub service_id: String,
    pub plan_id: String,
    pub context: Option<HashMap<String, String>>,
    pub organization_guid: String,
    pub space_guid: String,
    pub parameters: Option<HashMap<String, String>>,
    pub maintenance_info: Option<MaintenanceInfo>,
}

/// <https://github.com/openservicebrokerapi/servicebroker/blob/v2.16/spec.md#body-4>
#[derive(Serialize, Default, Debug)]
#[allow(unused)]
pub struct CreatedServiceIstance {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ServiceInstanceMetadata>,
}

#[derive(Serialize, Default, Debug)]
#[allow(unused)]
pub struct ServiceInstanceMetadata {
    pub labels: HashMap<String, String>,
}

/// <https://github.com/openservicebrokerapi/servicebroker/blob/v2.16/spec.md#parameters-3>
#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct ServiceFetchParams {
    pub service: Option<String>,
    pub plan_id: Option<String>,
}

/// <https://github.com/openservicebrokerapi/servicebroker/blob/v2.16/spec.md#body-5>
#[derive(Serialize, Default, Debug)]
#[allow(unused)]
pub struct ServiceIstance {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ServiceInstanceMetadata>,
}

pub async fn get_service_instance(
    web::Path(_instance_id): web::Path<String>,
    web::Query(params): web::Query<ServiceFetchParams>,
) -> impl Responder {
    log::info!("params {:?}", params);

    HttpResponse::Ok().json(ServiceIstance::default())
}

/// <https://github.com/openservicebrokerapi/servicebroker/blob/v2.16/spec.md#body-7>
#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct ServiceUpdateRequestBody {
    pub service_id: String,
    pub plan_id: Option<String>,
    pub context: Option<HashMap<String, String>>,
    pub parameters: Option<HashMap<String, String>>,
    pub previous_values: Option<PreviousValues>,
    pub maintenance_info: Option<MaintenanceInfo>,
}

#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct PreviousValues {
    pub service_id: Option<String>,
    pub plan_id: Option<String>,
    pub organization_id: Option<String>,
    pub space_id: Option<String>,
    pub maintenance_info: Option<MaintenanceInfo>,
}

pub async fn patch_service_instance(
    web::Path(_instance_id): web::Path<String>,
    web::Query(params): web::Query<ProvisionParams>,
    web::Json(body): web::Json<ServiceUpdateRequestBody>,
) -> impl Responder {
    log::info!("params {:?}, body:\n{:#?}", params, body);

    HttpResponse::Ok().json(CreatedServiceIstance::default())
}

/// <https://github.com/openservicebrokerapi/servicebroker/blob/v2.16/spec.md#parameters-8>
#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct ServiceDeleteParams {
    pub service_id: String,
    pub plan_id: String,
    pub accepts_incomplete: Option<bool>,
}

pub async fn delete_service_instance(
    web::Path(_instance_id): web::Path<String>,
    web::Query(params): web::Query<ServiceDeleteParams>,
) -> impl Responder {
    log::info!("params {:?}", params);

    HttpResponse::Ok().json(json!({}))
}
