use actix_web::{error, get, web, Responder};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

use crate::lib;

#[derive(Debug, Display, Error)]
#[display(fmt = "error: {}", message)]
pub struct ResponseError {
    message: &'static str,
}

// Use default implementation for `error_response()` method
impl error::ResponseError for ResponseError {}

#[derive(Deserialize)]
pub struct Info {
    qtd: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ValidCpfResponse {
    cpf: Vec<lib::Cpf>,
    message: String,
    quantity: u32,
}

#[derive(Debug, Serialize)]
struct ValidateResponse {
    is_valid: bool,
    error: Option<String>,
}

#[get("/gen-cpf")]
pub async fn new_cpf(qtd: web::Query<Info>) -> impl Responder {
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
        cpfs.push(lib::generate_cpf(None, None));
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
    qtd: web::Query<Info>,
    state_code: web::Path<String>,
) -> Result<impl Responder, ResponseError> {
    let state_code_parsed: u8 = match state_code.parse::<u8>() {
        Ok(state_code) => {
            if state_code > 9 {
                return Err(ResponseError {
                    message: "Invalid state code. Must be a number between 0 and 9.",
                });
            }
            state_code
        }
        Err(_) => {
            return Err(ResponseError {
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
        cpfs.push(lib::generate_cpf(Some(state_code_parsed), None));
    }
    Ok(web::Json({
        ValidCpfResponse {
            cpf: cpfs,
            message: format!("Generated {} CPFs with state code {}", qtd, state_code_parsed),
            quantity: qtd,
        }
    }))
}

#[get("/validate-cpf/{cpf}")]
pub async fn validate_cpf(cpf: web::Path<String>) -> impl Responder {
    let cpf = cpf.to_string();
    let is_valid = lib::validate_cpf(&cpf);
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
