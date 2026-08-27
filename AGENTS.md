# Fusion Tool — Agent Guide

This repository is the **Fusion CLI** (`fusion`). It scaffolds and manages applications for [Fusion Framework](https://github.com/cipherunits/fusion-framework).

## Source of truth

When CLI templates conflict with the framework:

1. **fusion-framework source** (APIs, defaults, versions)
2. Framework tests / examples
3. Official docs ([fusion.cipherunit.xyz](https://fusion.cipherunit.xyz/) / fusion-docs)
4. This CLI’s generators

**Never invent Fusion APIs.** Inspect the framework before changing a template.

## Supported languages

| Init `--lang` | Generated stack |
| --- | --- |
| `python` | `main.py`, `requirements.txt`, `pyproject.toml` |
| `typescript` / `node` | `main.ts`, `package.json`, `tsconfig.json` |
| `csharp` / `asp-core` | `main.cs`, `*.csproj` (`net10.0`) |

## Key constants

| Constant | File | Meaning |
| --- | --- | --- |
| `FUSION_FRAMEWORK_VERSION` | `src/setting/config.rs` | Pin written into generated deps / `fusion-framework.toml` |
| `FUSION_TOOL_VERSION` | Cargo package version | CLI self version |

Bump `FUSION_FRAMEWORK_VERSION` whenever framework packages release a new version.

## Where templates live

There are **no** external template directories. Scaffolds are Rust string constants:

| Concern | Module |
| --- | --- |
| App layout / entry / route module | `src/setting/structure.rs` |
| `fusion.*.json`, `.gitignore`, swagger | `src/setting/environment.rs` |
| Library packages (`fusion module init`) | `src/setting/module/scaffold.rs` |
| `fusion init` command | `src/command/init.rs` |

## FMA rules for generated apps

- One entrypoint owns `FusionApp.listen()` / `Listen()`
- Route modules under `src/modules/...` as `FusionBaseApi` + `@route` / `route()` / `[Route]`
- Import/register modules explicitly (no filesystem discovery)
- Nested `fusion.<env>.json` with `config` + `commands`
- Do **not** scaffold ORM, DI containers, or job queues
- `FusionApp` already registers framework identity headers — do not claim “no default middleware”

## Verification workflow

After changing generators:

```bash
cargo test
cargo build --release
# Then generate a throwaway project and inspect pins / routes / env JSON
cargo run -- init /tmp/fusion-py-check --lang python --name check --description "check"
```

Prefer asserting generated **file contents** in `#[cfg(test)]` (see `structure.rs`, `environment.rs`).

## Skills

Project skills live under `.agents/skills/`. Read the matching skill before editing generators or templates.

## Forbidden

- Inventing decorators / attributes not in fusion-framework
- Leaving Python apps without a dependency pin
- Targeting `net8.0` while the NuGet package is `net10.0`
- Diverging Python / Node / C# starters without verifying each language independently
