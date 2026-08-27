---
name: fusion-cli-templates
description: >-
  Locates and updates embedded Fusion CLI templates (Rust string constants).
  Use when changing starter code, gitignore, swagger JSON, or module scaffolds.
---

# Templates

Templates are **embedded strings** in Rust — not files under `templates/`.

| Template set | File | Contents |
|--------------|------|----------|
| App | `src/setting/structure.rs` | `PYTHON_*`, `TYPESCRIPT_*`, `CSHARP_*` |
| Env | `src/setting/environment.rs` | swagger blob, ports, gitignore |
| Module packages | `src/setting/module/scaffold.rs` | `fusion.module.toml` + hello packages |

## Editing rules

1. Change the string constant
2. Update / add `#[cfg(test)]` assertions on the generated text
3. Keep cross-language demos equivalent (same routes/behavior)
4. Run `cargo test`

## Placeholder

`__PROJECT_NAME__` is replaced by `render()` in `structure.rs` when needed.
