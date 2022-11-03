use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct Cpf {
    pub cpf: String,

    // This renames the key during serialization to the desired name in the JSON.
    #[serde(rename = "cpf-formatted")]
    pub cpf_formatted: String,

    #[serde(rename = "cpf-state")]
    pub cpf_state: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct GenCpfInfo {
    pub qtd: Option<String>,

    #[serde(rename = "state-code")]
    pub state_code: Option<String>,
}

#[derive(Deserialize)]
pub struct ValidateCpf {
    pub cpf: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CpfGenResponse {
    // Only serialize if Some value is present, if not the key will not be present in the JSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpfs: Option<Vec<Cpf>>,

    pub message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u32>,

    pub error: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ValidateResponse {
    #[serde(rename = "is-valid")]
    pub is_valid: bool,

    pub error: Option<String>,
}
