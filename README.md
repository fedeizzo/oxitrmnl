# Oxitrmnl

## Development

```
direnv allow
```

### OpenAPI spec codegen

```sh
openapi-generator-cli generate -g rust-axum -i openapi.json -o src/lib/openapi
```
