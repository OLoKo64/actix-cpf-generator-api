use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, PartialEq, Eq)]
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

#[derive(Debug)]
pub struct CpfUtils {
    pub seed: [u8; 9],
    cpf_seed_with_first_verifier: Option<[u8; 9]>,
    first_verifier_number: Option<u8>,
    second_verifier_number: Option<u8>,
}

impl CpfUtils {
    pub fn new(validate_seed: Option<[u8; 9]>, state_code: Option<u8>) -> Self {
        Self {
            seed: match validate_seed {
                Some(seed) => seed,
                None => match state_code {
                    Some(state_code) => {
                        let mut seed = CpfUtils::random_seed();
                        seed[8] = state_code;
                        seed
                    }
                    None => CpfUtils::random_seed(),
                },
            },
            cpf_seed_with_first_verifier: None,
            first_verifier_number: None,
            second_verifier_number: None,
        }
    }

    pub fn get_verifier_numbers(&self) -> [u8; 2] {
        [
            self.first_verifier_number
                .expect("first_verifier_number is not set"),
            self.second_verifier_number
                .expect("second_verifier_number is not set"),
        ]
    }

    pub fn set_verifier_numbers(&mut self) {
        let seed = match self.cpf_seed_with_first_verifier {
            Some(seed) => seed,
            None => self.seed,
        };
        let sum_elements: u64 = seed
            .iter()
            .enumerate()
            .map(|(index, &number)| u64::from(number) * (10 - index as u64))
            .sum::<u64>();

        let n2 = sum_elements % 11;
        let verifier_number = if n2 < 2 {
            0
        } else {
            (11 - n2)
                .try_into()
                .expect("verifier_number could not be parsed to u8")
        };
        if self.cpf_seed_with_first_verifier.is_some() {
            self.second_verifier_number = Some(verifier_number);
        } else {
            self.first_verifier_number = Some(verifier_number);
            self.cpf_seed_with_first_verifier = Some(self.set_cpf_seed_with_first_verifier());
            self.set_verifier_numbers();
        }
    }

    fn set_cpf_seed_with_first_verifier(&self) -> [u8; 9] {
        let mut seed_vec = self.seed.to_vec();
        seed_vec.push(
            self.first_verifier_number
                .expect("first_verifier_number is not set"),
        );
        seed_vec.remove(0);
        seed_vec
            .try_into()
            .expect("Invalid seed with first verifier")
    }

    fn random_seed() -> [u8; 9] {
        let mut seed = [0; 9];
        for i in &mut seed {
            *i = rand::thread_rng().gen_range(0..10);
        }
        seed
    }
}
