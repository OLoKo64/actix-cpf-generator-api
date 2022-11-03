# CPF Generator and Validator

A simple CPF generator written in Rust using both [Actix](https://actix.rs/) and [Axum](https://crates.io/crates/axum) frameworks.

## Build

```bash
cargo build --release
```

## Endpoints

- `GET /gen-cpf` - Generates a valid CPF
- `GET /gen-cpf?qtd=10&state_code=8` - Generates 10 valid CPFs with state code 8 (SP)
- `GET /validate-cpf?cpf=123456789012` - Validates a CPF
