use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

/// https://github.com/openservicebrokerapi/servicebroker/blob/v2.16/spec.md#request-1
#[derive(Deserialize, Debug)]
#[allow(unused)]
pub struct LastOperationParams {
    service_id: Option<String>,
    plan_id: Option<String>,
    operation: Option<String>,
}

/// https://github.com/openservicebrokerapi/servicebroker/blob/v2.16/spec.md#response-1
#[derive(Serialize, Default, Debug)]
pub struct ServiceInstanceLastOp {
    state: LastOperationState,
    description: Option<String>,
    instance_usable: Option<bool>,
    update_repeatable: Option<bool>,
}

#[derive(Serialize, Debug)]
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

/// https://github.com/openservicebrokerapi/servicebroker/blob/v2.16/spec.md#response-2
#[derive(Serialize, Default, Debug)]
struct ServiceBindingLastOp {
    state: LastOperationState,
    description: Option<String>,
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
