---
name: fusion-cli-parity
description: >-
  Keeps Python, TypeScript, and C# fusion init scaffolds architecturally aligned.
  Use when a Fusion Framework feature must land in all language starters.
---

# Cross-language parity

When framework adds a feature that starters should demo:

```text
Framework change
  → structure.rs (Python + TypeScript + C#)
  → environment.rs if config needed
  → module/scaffold.rs only if library packages need it
  → cargo tests for each language
  → README / AGENTS.md
```

## Shared concepts (must match)

- FMA layout: entry + `core/settings` + `src/modules/...`
- Nested `fusion.<env>.json`
- Framework version pin
- Sample products module with convention + custom HTTP route
- Correct identity-header middleware narrative

## Language-specific (must stay idiomatic)

| Language | Registration | Custom HTTP | Package pin |
|----------|--------------|-------------|-------------|
| Python | import + `@route` | `@http_get` | `requirements.txt` / `pyproject.toml` |
| TypeScript | side-effect import + `route()(Class)` | `httpGet(...)(proto.method)` | `package.json` |
| C# | `Route.RegisterAll` / attributes | `[HttpGet]` | `.csproj` PackageReference |

Verify each language independently — never assume identical syntax.
