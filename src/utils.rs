use axum::Json;
use hyper::StatusCode;
use std::error::Error;
use std::num::ParseIntError;
use unicode_segmentation::UnicodeSegmentation;

use crate::structs::Cpf;
use crate::structs::CpfGenResponse;
use crate::structs::CpfUtils;

pub fn quantity_to_u32(
    quantity: &Option<String>,
) -> Result<u32, (StatusCode, Json<CpfGenResponse>)> {
    match quantity {
        Some(ref qtd) => match qtd.parse::<u32>() {
            Ok(qtd) => Ok(qtd),
            Err(error) => Err((
                StatusCode::BAD_REQUEST,
                Json(CpfGenResponse {
                    cpfs: None,
                    message: "Invalid state_code parameter.".to_string(),
                    quantity: None,
                    error: Some(error.to_string()),
                }),
            )),
        },
        None => Ok(1),
    }
}

pub fn get_state_code(
    state_code: &Option<String>,
) -> Result<Option<u8>, (StatusCode, Json<CpfGenResponse>)> {
    match &state_code {
        Some(code) => match parse_state_code(code) {
            Ok(code) => Ok(Some(code)),
            Err(_error) => Err((
                StatusCode::BAD_REQUEST,
                Json(CpfGenResponse {
                    cpfs: None,
                    message: "Invalid state_code parameter.".to_string(),
                    quantity: None,
                    error: Some("Invalid state_code parameter.".to_string()),
                }),
            )),
        },
        None => Ok(None),
    }
}

pub fn parse_state_code(state_code: &str) -> Result<u8, String> {
    match state_code.parse::<u8>() {
        Ok(state_code) => {
            if state_code > 9 {
                return Err("Invalid state code. Must be a number between 0 and 9.".to_string());
            }
            Ok(state_code)
        }
        Err(_) => Err("Invalid state code. Must be a number between 0 and 9.".to_string()),
    }
}

pub fn validate_cpf(cpf: &str) -> Result<String, Box<dyn Error>> {
    let cpf = cpf.replace(['.', '-'], "");
    let cpf_len = cpf.graphemes(true).count();
    if cpf_len != 11 {
        return Err(format!("Invalid CPF. Must have 11 digits. It has {cpf_len}").into());
    }
    // This line guarantees that the vector will have 9 elements
    let binding = cpf.graphemes(true).collect::<Vec<_>>()[..9].to_vec();
    let cpf_seed: Result<Vec<_>, ParseIntError> = binding
        .into_iter()
        .map(|x| x.to_string().parse::<u8>())
        .collect();
    let Ok(cpf_seed) = cpf_seed else {
        return Err("Invalid character in CPF. Must have 11 digits.".into());
    };
    // With that this unwrap is guaranteed to be valid
    let generated_cpf = generate_cpf(
        None,
        Some(
            cpf_seed
                .try_into()
                .expect("cpf_seed does not have the correct length"),
        ),
    );
    if generated_cpf.cpf == cpf {
        return Ok("Valid CPF.".to_string());
    }
    Err("Invalid CPF.".into())
}

pub fn generate_cpf(state_code: Option<u8>, validate_seed: Option<[u8; 9]>) -> Cpf {
    let mut cpf_seed = CpfUtils::new(validate_seed, state_code);
    cpf_seed.set_verifier_numbers();
    let mut cpf = cpf_seed.seed.into_iter().collect::<Vec<_>>();
    cpf.extend(cpf_seed.get_verifier_numbers());
    let str_cpf = cpf.iter().map(ToString::to_string).collect::<String>();
    let formatted_cpf = format!(
        "{}.{}.{}-{}",
        &str_cpf[0..3],
        &str_cpf[3..6],
        &str_cpf[6..9],
        &str_cpf[9..11]
    );
    Cpf {
        cpf: str_cpf.to_string(),
        cpf_formatted: formatted_cpf,
        cpf_state: cpf_state(&str_cpf)
            .iter()
            .map(|&state| state.to_string())
            .collect::<Vec<_>>(),
    }
}

fn cpf_state(cpf: &str) -> Vec<&str> {
    let state_code = cpf.graphemes(true).nth(8).unwrap();
    match state_code {
        "0" => vec!["RS"],
        "1" => vec!["DF", "GO", "MT", "MS", "TO"],
        "2" => vec!["AC", "AM", "AP", "PA", "RO", "RR"],
        "3" => vec!["CE", "MA", "PI"],
        "4" => vec!["AL", "PB", "PE", "RN"],
        "5" => vec!["BA", "SE"],
        "6" => vec!["MG"],
        "7" => vec!["ES", "RJ"],
        "8" => vec!["SP"],
        "9" => vec!["PR", "SC"],
        _ => panic!("Invalid state code."),
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_generate_cpf_with_starting_seed() {
        assert_eq!(
            generate_cpf(None, Some([2, 8, 0, 0, 1, 2, 3, 8, 9])),
            Cpf {
                cpf: "28001238938".to_string(),
                cpf_formatted: "280.012.389-38".to_string(),
                cpf_state: vec!["PR".to_string(), "SC".to_string()],
            }
        );
    }

    #[test]
    fn test_generate_cpf_with_state_code() {
        let cpf = generate_cpf(Some(9), None);
        assert_eq!(cpf.cpf.graphemes(true).nth(8).unwrap(), "9");
    }

    #[test]
    fn test_validate_generated_cpf() {
        let cpf = generate_cpf(None, None);
        let result = validate_cpf(&cpf.cpf);
        assert_eq!(result.ok(), Some("Valid CPF.".to_string()));
    }

    #[test]
    fn test_validate_cpf() {
        let result = validate_cpf("280.012.389-38");
        assert_eq!(result.ok(), Some("Valid CPF.".to_string()));
    }

    #[test]
    fn test_validate_cpf_invalid() {
        let result = validate_cpf("280.012.389-33");
        assert_eq!(result.ok(), None);
    }
}
