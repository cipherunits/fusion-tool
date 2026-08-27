---
name: fusion-cli-testing
description: >-
  Tests Fusion CLI generators and generated project contents. Use after template
  or init changes, before releases.
---

# Testing the CLI

## Unit (required)

```bash
cargo test
```

Key modules:

- `setting::structure::tests` — Python / TypeScript / C# file contents
- `setting::environment::tests` — ports, fingerprint, gitignore
- `setting::module::scaffold` tests — library packages

## Manual smoke (recommended)

```bash
cargo run -- init /tmp/ff-py --lang python --name ff-py --description "smoke"
cargo run -- init /tmp/ff-ts --lang typescript --name ff-ts --description "smoke"
cargo run -- init /tmp/ff-cs --lang csharp --name ff-cs --description "smoke"
```

Inspect:

- `fusion-framework.toml` version pin
- `fusion.dev.json` port / fingerprint / swagger
- Route module custom HTTP helpers
- Dependency manifests

Full install/run of generated apps requires network access to PyPI/npm/NuGet and matching runtimes.

## Never

Mark a template change complete without `cargo test` and content assertions.
