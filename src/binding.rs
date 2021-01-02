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

/// https://github.com/openservicebrokerapi/servicebroker/blob/v2.16/spec.md#parameters-5
#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct ProvisionParams {
    accepts_incomplete: Option<bool>,
}

/// https://github.com/openservicebrokerapi/servicebroker/blob/v2.16/spec.md#body-8
#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct BindingRequestBody {
    service_id: String,
    plan_id: String,
    context: Option<HashMap<String, String>>,
    bind_resource: Option<BindResource>,
    parameters: Option<HashMap<String, String>>,
}

/// https://github.com/openservicebrokerapi/servicebroker/blob/v2.16/spec.md#bind-resource-object
#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct BindResource {
    app_guid: Option<String>,
    route: Option<String>,
}

/// https://github.com/openservicebrokerapi/servicebroker/blob/v2.16/spec.md#body-9
#[derive(Serialize, Default, Debug)]
#[allow(unused)]
pub struct Binding {
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<BindingMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    credentials: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    syslog_drain_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    route_service_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_mounts: Option<VolumeMount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    endpoints: Option<Vec<Endpoint>>,
}

#[derive(Serialize, Default, Debug)]
#[allow(unused)]
pub struct BindingMetadata {
    expires_at: String,
}

#[derive(Serialize, Debug)]
#[allow(unused)]
pub struct VolumeMount {
    driver: String,
    container_dir: String,
    mode: VolumeMode,
    device_type: DeviceType,
    device: Device,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "lowercase")]
#[allow(unused)]
pub enum VolumeMode {
    R,
    RW,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case")]
#[allow(unused)]
pub enum DeviceType {
    Shared,
}

#[derive(Serialize, Default, Debug)]
#[allow(unused)]
pub struct Device {
    volume_id: String,
    mount_config: Option<HashMap<String, String>>,
}

#[derive(Serialize, Default, Debug)]
#[allow(unused)]
pub struct Endpoint {
    host: String,
    ports: Vec<String>,
    protocol: Option<Protocol>,
}

#[derive(Serialize, Debug)]
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

/// https://github.com/openservicebrokerapi/servicebroker/blob/v2.16/spec.md#parameters-6
#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct BindingFetchParams {
    service: Option<String>,
    plan_id: Option<String>,
}

pub async fn get_binding(
    web::Path((_instance_id, _binding_id)): web::Path<(String, String)>,
    web::Query(params): web::Query<BindingFetchParams>,
) -> impl Responder {
    log::info!("params {:?}", params);

    HttpResponse::Ok().json(Binding::default())
}

pub async fn delete_binding(
    web::Path((_instance_id, _binding_id)): web::Path<(String, String)>,
    web::Query(params): web::Query<BindingFetchParams>,
) -> impl Responder {
    log::info!("params {:?}", params);

    HttpResponse::Ok().json(json!({}))
}
