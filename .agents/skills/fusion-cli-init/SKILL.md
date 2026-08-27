---
name: fusion-cli-init
description: >-
  Guides safe changes to fusion init project generation for Python, TypeScript,
  and C#. Use when editing init command, structure.rs, or environment.rs.
---

# fusion init

## Flow

1. Resolve language (`python` / `typescript` / `asp-core`)
2. Write `fusion-framework.toml` with framework + tool versions
3. Write `fusion.{dev,stage,prod}.json` via `environment.rs`
4. Write `.gitignore`
5. Create dirs + files via `structure::create`

## Generated app must include

- Entrypoint that imports/registers one sample route module and calls `listen`
- `core/settings` overlay
- `src/modules/products` route module with `[module]`, `version="v1"`, convention verbs, and one custom HTTP route (`http_get` / `httpGet` / `[HttpGet]`)
- Nested env JSON: `config` (host/port/debug/fingerprint/swagger) + `commands.run`
- Dependency pin to `FUSION_FRAMEWORK_VERSION`
- Accurate comments about `framework_headers` defaults (already on `FusionApp`)

## Ports

| Env | Port |
|-----|------|
| dev | 8080 |
| stage | 8081 |
| prod | 9090 |

## Checklist before merge

- [ ] `cargo test` passes
- [ ] Generated Python has `requirements.txt` + `pyproject.toml`
- [ ] Generated Node pins `fusion-framework` in `package.json`
- [ ] Generated C# uses `net10.0` and NuGet pin
- [ ] No “ships with none by default” middleware wording
