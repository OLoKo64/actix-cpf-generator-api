use axum::response::Html;

pub const INDEX_HTML: Html<&str> = Html(
    r#"
<h1>CPF Generator and Validator API</h1>
<h4>For more information see the <a href="https://github.com/OLoKo64/rust-cpf-generator-api">Github Repository</a>.</h4>
<table>
    <tr>
        <th>Method</th>
        <th>Endpoint</th>
        <th>Description</th>
    </tr>
    <tr>
        <th>GET</th>
        <th><a href="https://cpf-generator-api.onrender.com/gen-cpf">/gen-cpf</a></th>
        <th>Generates a valid CPF</th>
    </tr>
    <tr>
        <th>GET</th>
        <th><a href="https://cpf-generator-api.onrender.com/gen-cpf?qtd=10&state_code=8">/gen-cpf?qtd=10&state_code=8</a></th>
        <th>Generates 10 valid CPFs with state code 8 (SP)</th>
    </tr>
    <tr>
        <th>GET</th>
        <th><a href="https://cpf-generator-api.onrender.com/validate-cpf?cpf=280.012.389-38">/validate-cpf?cpf=280.012.389-38</a></th>
        <th>Validates the given CPF</th>
    </tr>
</table>
<style>
    body {
        font-family: arial, sans-serif;
    }

    table {
        border-collapse: collapse;
        width: 100%;
    }

    td, th {
        border: 1px solid #dddddd;
        text-align: left;
        padding: 8px;
    }

    tr:nth-child(even) {
        background-color: #dddddd;
    }
</style>
"#,
);
