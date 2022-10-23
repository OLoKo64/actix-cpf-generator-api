# CPF Generator and Validator

A simple CPF generator written in Rust using the [Actix framework](https://actix.rs/).

## Build

```bash
cargo build --release
```

## Endpoints

- `GET /gen-cpf` - Generates a valid CPF
- `GET /gen-cpf/8` - Generates a valid CPF from SP
- `GET /gen-cpf?qtd=10` - Generates 10 valid CPFs
- `GET /validate-cpf/{cpf}` - Validates a CPF
