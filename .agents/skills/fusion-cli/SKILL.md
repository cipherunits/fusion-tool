---
name: fusion-cli
description: >-
  Maintains Fusion Tool (fusion CLI) synchronization with fusion-framework.
  Use when editing generators, templates, init, module scaffolds, version pins,
  or CLI tests in the fusion-tool repository.
---

# Fusion CLI

You are modifying the **Fusion CLI**, not the framework itself.

## Hard rules

1. Before changing a generator or template, inspect the matching Fusion Framework implementation.
2. Never invent Fusion APIs, decorators, attributes, or config keys.
3. Keep `FUSION_FRAMEWORK_VERSION` aligned with published framework packages.
4. After template changes: update unit tests that assert **file contents**, then run `cargo test`.
5. Python, TypeScript, and C# starters must stay architecturally aligned but idiomatic.

## Source layout

| Path | Role |
|------|------|
| `src/command/` | CLI subcommands (`init`, `command`, `module`, `add`, `update`) |
| `src/setting/structure.rs` | App scaffolds |
| `src/setting/environment.rs` | Env JSON + gitignore |
| `src/setting/module/` | Library package scaffolds |
| `src/setting/config.rs` | Version pins + language enum |

## Relationship

```text
fusion-framework APIs
        ↓
CLI generators (structure / environment / module)
        ↓
fusion init / fusion module init output
        ↓
Developer project
```

## Do not confuse

| Concept | Meaning |
|---------|---------|
| Route module | `FusionBaseApi` class in an app (`src/modules/...`) |
| Library package | `fusion module init` output (`fusion.module.toml`) |

See also: `AGENTS.md`, `fusion-cli-init`, `fusion-cli-templates`, `fusion-cli-parity`, `fusion-cli-testing`.
