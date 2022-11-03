use crate::types::{CpfGenResponse, GenCpfInfo, ValidateCpf, ValidateResponse};
use crate::utils;
use axum::{
    extract::{Json, Query},
    http::StatusCode,
    response::IntoResponse,
};

// Extractors documentation: https://docs.rs/axum/latest/axum/#extractors
pub async fn new_cpf(query_params: Query<GenCpfInfo>) -> (StatusCode, impl IntoResponse) {
    let mut qtd = match query_params.qtd {
        Some(ref qtd) => match qtd.parse::<u32>() {
            Ok(qtd) => qtd,
            Err(error) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(CpfGenResponse {
                        cpfs: None,
                        message: "Invalid qtd parameter.".to_string(),
                        quantity: None,
                        error: Some(error.to_string()),
                    }),
                )
            }
        },
        None => 1,
    };
    let state_code: Option<u8> = match &query_params.state_code {
        Some(state_code) => match utils::parse_state_code(state_code) {
            Ok(state_code) => Some(state_code),
            Err(error) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(CpfGenResponse {
                        cpfs: None,
                        message: "Invalid state_code parameter.".to_string(),
                        quantity: None,
                        error: Some(error),
                    }),
                )
            }
        },
        None => None,
    };
    if qtd > 1000 {
        qtd = 1000
    }
    let mut cpfs = Vec::new();
    (0..qtd)
        .into_iter()
        .for_each(|_| cpfs.push(utils::generate_cpf(state_code, None)));
    (
        StatusCode::OK,
        Json(CpfGenResponse {
            cpfs: Some(cpfs),
            message: format!("{} CPFs generated.", qtd),
            quantity: Some(qtd),
            error: None,
        }),
    )
}

pub async fn validate_cpf(query: Query<ValidateCpf>) -> impl IntoResponse {
    let cpf = match &query.cpf {
        Some(cpf) => cpf,
        None => return Json(ValidateResponse { is_valid: false, error: Some("CPF not provided. Inform the cpf in the query params: '/validate-cpf?cpf=123456789012'".to_string()) }),
    };
    let is_valid = utils::validate_cpf(cpf);
    if let Err(error_message) = is_valid {
        Json(ValidateResponse {
            is_valid: false,
            error: Some(error_message.to_string()),
        })
    } else {
        Json(ValidateResponse {
            is_valid: true,
            error: None,
        })
    }
}
