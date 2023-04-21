use crate::htmls::INDEX_HTML;
use crate::structs::{CpfGenResponse, GenCpfInfo, ValidateCpf, ValidateResponse};
use crate::utils::{self, get_state_code, quantity_to_u32};
use axum::{
    extract::{Json, Query},
    http::StatusCode,
    response::IntoResponse,
};

pub async fn index_page() -> impl IntoResponse {
    INDEX_HTML
}

// Extractors documentation: https://docs.rs/axum/latest/axum/#extractors
pub async fn new_cpf(
    Query(query_params): Query<GenCpfInfo>,
) -> Result<impl IntoResponse, (StatusCode, Json<CpfGenResponse>)> {
    let mut qtd = quantity_to_u32(&query_params.qtd)?;
    let state_code = get_state_code(&query_params.state_code)?;
    if qtd > 1000 {
        qtd = 1000;
    }
    let mut cpfs = Vec::new();
    (0..qtd).for_each(|_| cpfs.push(utils::generate_cpf(state_code, None)));
    Ok((
        StatusCode::OK,
        Json(CpfGenResponse {
            cpfs: Some(cpfs),
            message: format!("{qtd} CPFs generated."),
            quantity: Some(qtd),
            error: None,
        }),
    ))
}

pub async fn validate_cpf(Query(query): Query<ValidateCpf>) -> impl IntoResponse {
    let Some(cpf) = &query.cpf else {
        return Json(ValidateResponse { is_valid: false, error: Some("CPF not provided. Inform the cpf in the query params: '/validate-cpf?cpf=123456789012'".to_string()) });
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
