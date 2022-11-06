# CPF Generator and Validator

[![Rust API Workflow](https://github.com/OLoKo64/rust-cpf-generator-api/actions/workflows/rust-workflow.yml/badge.svg?branch=main)](https://github.com/OLoKo64/rust-cpf-generator-api/actions/workflows/rust-workflow.yml)

A simple CPF generator written in Rust using the [Axum](https://crates.io/crates/axum) framework.

## Build

```bash
cargo build --release
```

## Endpoints

- `GET /gen-cpf` - Generates a valid CPF
- `GET /gen-cpf?qtd=10&state_code=8` - Generates 10 valid CPFs with state code 8 (SP)
- `GET /validate-cpf?cpf=123456789012` - Validates a CPF

## Dependencies

> ```libssl-1.1```

Ubuntu:

```bash
sudo apt install libssl-dev
```
Arch:

```bash
sudo pacman -S openssl-1.1
```
