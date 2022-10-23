use actix_web::{get, web, Responder};

use crate::types::{
    QuantityCpf, ResponseErrorCustom, ValidCpfResponse, ValidateCpf, ValidateResponse,
};
use crate::utils;

#[get("/gen-cpf")]
pub async fn new_cpf(qtd: web::Query<QuantityCpf>) -> impl Responder {
    let mut qtd = qtd
        .qtd
        .clone()
        .unwrap_or_else(|| "1".to_string())
        .parse::<u32>()
        .unwrap_or(1);
    if qtd > 1000 {
        qtd = 1000;
    }
    let mut cpfs = Vec::new();
    for _ in 0..qtd {
        cpfs.push(utils::generate_cpf(None, None));
    }
    web::Json({
        ValidCpfResponse {
            cpf: cpfs,
            message: format!("{} CPFs generated.", qtd),
            quantity: qtd,
        }
    })
}

#[get("/gen-cpf/{state_code}")]
pub async fn new_cpf_state_code(
    qtd: web::Query<QuantityCpf>,
    state_code: web::Path<String>,
) -> Result<impl Responder, ResponseErrorCustom> {
    let state_code_parsed: u8 = match state_code.parse::<u8>() {
        Ok(state_code) => {
            if state_code > 9 {
                return Err(ResponseErrorCustom {
                    message: "Invalid state code. Must be a number between 0 and 9.",
                });
            }
            state_code
        }
        Err(_) => {
            return Err(ResponseErrorCustom {
                message: "Invalid state code. Must be a number between 0 and 9.",
            })
        }
    };
    let mut qtd = qtd
        .qtd
        .clone()
        .unwrap_or_else(|| "1".to_string())
        .parse::<u32>()
        .unwrap_or(1);
    if qtd > 1000 {
        qtd = 1000;
    }
    let mut cpfs = Vec::new();
    for _ in 0..qtd {
        cpfs.push(utils::generate_cpf(Some(state_code_parsed), None));
    }
    Ok(web::Json({
        ValidCpfResponse {
            cpf: cpfs,
            message: format!(
                "Generated {} CPFs with state code {}",
                qtd, state_code_parsed
            ),
            quantity: qtd,
        }
    }))
}

#[get("/validate-cpf")]
pub async fn validate_cpf(query: web::Query<ValidateCpf>) -> impl Responder {
    let cpf = match &query.cpf {
        Some(cpf) => cpf,
        None => return web::Json(ValidateResponse { is_valid: false, error: Some("CPF not provided. Inform the cpf in the query params: '/validate-cpf?cpf=123456789012'".to_string()) }),
    };
    let is_valid = utils::validate_cpf(&cpf);
    if let Err(error_message) = is_valid {
        web::Json(ValidateResponse {
            is_valid: false,
            error: Some(error_message.to_string()),
        })
    } else {
        web::Json(ValidateResponse {
            is_valid: true,
            error: None,
        })
    }
}
