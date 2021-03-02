use std::collections::HashMap;

use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[allow(unused)]
pub enum BindingType {
    Credentials,
    LogDrain,
    RouteServices,
    VolumeServices,
}

/// <https://github.com/openservicebrokerapi/servicebroker/blob/v2.16/spec.md#parameters-5>
#[derive(Deserialize, Debug, Clone)]
#[allow(unused)]
pub struct ProvisionParams {
    pub accepts_incomplete: Option<bool>,
}

/// <https://github.com/openservicebrokerapi/servicebroker/blob/v2.16/spec.md#body-8>
#[derive(Deserialize, Debug, Clone)]
#[allow(unused)]
pub struct BindingRequestBody {
    pub service_id: String,
    pub plan_id: String,
    pub context: Option<HashMap<String, String>>,
    pub bind_resource: Option<BindResource>,
    pub parameters: Option<HashMap<String, String>>,
}

/// <https://github.com/openservicebrokerapi/servicebroker/blob/v2.16/spec.md#bind-resource-object>
#[derive(Deserialize, Debug, Clone)]
#[allow(unused)]
pub struct BindResource {
    pub app_guid: Option<String>,
    pub route: Option<String>,
}

/// <https://github.com/openservicebrokerapi/servicebroker/blob/v2.16/spec.md#body-9>
#[derive(Serialize, Default, Debug, Clone)]
#[allow(unused)]
pub struct Binding {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<BindingMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syslog_drain_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_service_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_mounts: Option<VolumeMount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub endpoints: Option<Vec<Endpoint>>,
}

#[derive(Serialize, Default, Debug, Clone)]
#[allow(unused)]
pub struct BindingMetadata {
    pub expires_at: String,
}

#[derive(Serialize, Debug, Clone)]
#[allow(unused)]
pub struct VolumeMount {
    pub driver: String,
    pub container_dir: String,
    pub mode: VolumeMode,
    pub device_type: DeviceType,
    pub device: Device,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
#[allow(unused)]
pub enum VolumeMode {
    R,
    RW,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
#[allow(unused)]
pub enum DeviceType {
    Shared,
}

#[derive(Serialize, Default, Debug, Clone)]
#[allow(unused)]
pub struct Device {
    pub volume_id: String,
    pub mount_config: Option<HashMap<String, String>>,
}

#[derive(Serialize, Default, Debug, Clone)]
#[allow(unused)]
pub struct Endpoint {
    pub host: String,
    pub ports: Vec<String>,
    pub protocol: Option<Protocol>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
#[allow(unused)]
pub enum Protocol {
    TCP,
    UDP,
    All,
}

impl Default for Protocol {
    fn default() -> Self {
        Protocol::TCP
    }
}

pub async fn put_binding(
    web::Path((_instance_id, _binding_id)): web::Path<(String, String)>,
    web::Query(params): web::Query<ProvisionParams>,
    web::Json(body): web::Json<BindingRequestBody>,
) -> impl Responder {
    log::info!("params {:?}, body:\n{:#?}", params, body);

    HttpResponse::Created().json(Binding::default())
}

/// <https://github.com/openservicebrokerapi/servicebroker/blob/v2.16/spec.md#parameters-6>
#[derive(Deserialize, Debug, Clone)]
#[allow(unused)]
pub struct BindingFetchParams {
    pub service: Option<String>,
    pub plan_id: Option<String>,
}

pub async fn get_binding(
    web::Path((_instance_id, _binding_id)): web::Path<(String, String)>,
    web::Query(params): web::Query<BindingFetchParams>,
) -> impl Responder {
    log::info!("params {:?}", params);

    HttpResponse::Ok().json(Binding::default())
}

/// <https://github.com/openservicebrokerapi/servicebroker/blob/v2.16/spec.md#parameters-7>
#[derive(Deserialize, Debug, Clone)]
#[allow(unused)]
pub struct BindingDeleteParams {
    pub service_id: String,
    pub plan_id: String,
    pub accepts_incomplete: Option<bool>,
}

pub async fn delete_binding(
    web::Path((_instance_id, _binding_id)): web::Path<(String, String)>,
    web::Query(params): web::Query<BindingDeleteParams>,
) -> impl Responder {
    log::info!("params {:?}", params);

    HttpResponse::Ok().json(json!({}))
}
