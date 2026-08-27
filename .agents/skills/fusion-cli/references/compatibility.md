# Framework ↔ CLI compatibility

| Item | Framework (source of truth) | CLI must emit |
|------|-----------------------------|---------------|
| Package version | `1.2.3` | `FUSION_FRAMEWORK_VERSION` |
| C# TFM | `net10.0` | csproj `TargetFramework` |
| Default middleware | `framework_headers` on `FusionApp` | Document as already registered |
| Custom HTTP | `http_get` / `httpGet` / `[HttpGet]` | Demo on sample products module |
| Settings JSON | Nested `config` + top-level `commands` OK | Nested scaffold |
| Fingerprint | `fingerprint.enabled` | Present in env JSON |
| Route modules | Explicit import / `Route.Register` | Import in entry / RegisterAll |
| Library packages | Plain imports after `fusion add` | Separate from app route modules |

When framework releases, update `src/setting/config.rs` first, then re-run `cargo test` and regenerate sample projects.
