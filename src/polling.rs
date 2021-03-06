use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

/// <https://github.com/openservicebrokerapi/servicebroker/blob/v2.16/spec.md#request-1>
#[derive(Deserialize, Debug, Clone)]
#[allow(unused)]
pub struct LastOperationParams {
    pub service_id: Option<String>,
    pub plan_id: Option<String>,
    pub operation: Option<String>,
}

/// <https://github.com/openservicebrokerapi/servicebroker/blob/v2.16/spec.md#response-1>
#[derive(Serialize, Default, Debug, Clone)]
pub struct ServiceInstanceLastOp {
    pub state: LastOperationState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_usable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_repeatable: Option<bool>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
#[allow(unused)]
pub enum LastOperationState {
    #[serde(rename(serialize = "in progress"))]
    InProgress,
    Succeeded,
    Failed,
}

impl Default for LastOperationState {
    fn default() -> Self {
        LastOperationState::InProgress
    }
}

pub async fn get_service_instance_last_operation(
    _instance_id: web::Path<String>,
    web::Query(params): web::Query<LastOperationParams>,
) -> impl Responder {
    log::info!("params {:?}", params);

    let mut response = ServiceInstanceLastOp::default();
    response.state = LastOperationState::Succeeded;

    HttpResponse::Ok().json(response)
}

/// <https://github.com/openservicebrokerapi/servicebroker/blob/v2.16/spec.md#response-2>
#[derive(Serialize, Default, Debug, Clone)]
struct ServiceBindingLastOp {
    pub state: LastOperationState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

pub async fn get_service_binding_state(
    web::Path((_instance_id, _binding_id)): web::Path<(String, String)>,
    web::Query(params): web::Query<LastOperationParams>,
) -> impl Responder {
    log::info!("params {:?}", params);

    let mut response = ServiceBindingLastOp::default();
    response.state = LastOperationState::Succeeded;

    HttpResponse::Ok().json(response)
}
