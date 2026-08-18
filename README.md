# Fusion Tool

A CLI tool for creating and managing Fusion Framework projects. Supports both interactive and non-interactive (batch) modes.

## Installation

### Linux

```bash
curl -fsSL https://raw.githubusercontent.com/cipherunits/fusion-tool/main/scripts/install.sh | bash
```

Or install from source:

```bash
git clone https://github.com/cipherunits/fusion-tool.git
cd fusion-tool && cargo install --path .
```

### macOS

```bash
curl -fsSL https://raw.githubusercontent.com/cipherunits/fusion-tool/main/scripts/install.sh | bash
```

Or install from source:

```bash
git clone https://github.com/cipherunits/fusion-tool.git
cd fusion-tool && cargo install --path .
```

### Windows

In PowerShell:

```powershell
irm https://raw.githubusercontent.com/cipherunits/fusion-tool/main/scripts/install.ps1 | iex
```

This installs `fusion.exe` into `%LOCALAPPDATA%\Programs\fusion` and adds that
directory to your user `PATH`. **Open a new terminal window afterwards**, then:

```powershell
fusion --help
```

To install somewhere else, set `FUSION_INSTALL_DIR` (and optionally
`FUSION_VERSION`) before running the installer:

```powershell
$env:FUSION_INSTALL_DIR = "C:\tools\fusion"
irm https://raw.githubusercontent.com/cipherunits/fusion-tool/main/scripts/install.ps1 | iex
```

#### Manual install

Download the latest `.zip` from
[GitHub Releases](https://github.com/cipherunits/fusion-tool/releases) and
extract `fusion.exe` into a folder of your choice. Running it from another
directory (for example `fusion` in `cmd`) only works once that folder is on your
`PATH`:

```powershell
[Environment]::SetEnvironmentVariable(
  "Path",
  [Environment]::GetEnvironmentVariable("Path", "User") + ";C:\tools\fusion",
  "User"
)
```

Or install from source:

```powershell
git clone https://github.com/cipherunits/fusion-tool.git
cd fusion-tool
cargo install --path .
```

## Commands

### Interactive Mode

```bash
fusion init
```

You will be prompted for:
1. Programming language (Python, TypeScript, or ASP.NET Core)
2. Project name
3. Project description

The project is created in the current directory unless you pass a directory:

```bash
fusion init my-app
```

### Non-Interactive Mode

```bash
fusion init --lang python --name myproject --description "My awesome project"
```

#### Available Arguments for `fusion init`

| Argument | Type | Description | Required |
|------|------|-------------|----------|
| `[DIRECTORY]` | string | Target directory, created if missing (defaults to the current directory) | No |
| `--lang` | string | `python`, `typescript`, `asp-core` | No |
| `--name` | string | Project name | No |
| `--description` | string | Project description | No |

Any option you leave out is asked interactively.

#### Examples

```bash
fusion init --lang python --name my-app
fusion init --lang typescript --name my-app --description "A TypeScript app"
fusion init ./my-projects/test-app --lang python --name test
```

### Project Commands

Commands are declared per environment in the `commands` block of
`fusion.<env>.json`, so each project defines its own. Run one by name:

```bash
fusion command run               # dev, the default environment
fusion command run --stage
fusion command run:stage         # same thing, shorter
fusion command run --prod
fusion command run --env test    # any environment you created
```

Leave the name out to see what an environment declares:

```bash
fusion command --stage
```

The command runs in the project root through your shell (`sh -c` on Linux and
macOS, `cmd /C` on Windows), with `FUSION_ENV` set to the chosen environment so
that `core/settings` loads the matching config. Its exit code becomes the exit
code of `fusion`, which keeps it usable in scripts and CI.

Without a flag or a `:env` suffix, the environment comes from `FUSION_ENV` if it
is set, and falls back to `dev`.

### Modules

Fusion modules are **publishable library packages** (separate repos). Authors
scaffold with `fusion module init`, push to GitHub, and apps install with
`fusion add --github`. After install, import the package in your Fusion app
like any other dependency — modules are not route/API plugins.

#### Create a module package

```bash
fusion module init
```

You will be prompted for:

1. Implementation language — Python, TypeScript, C#, or Rust (Rust can target all hosts via PyO3 / N-API / a C# class library)
2. Module name (id)
3. Description
4. Output directory (defaults to `fusion-<id>-mod`)

Non-interactive:

```bash
fusion module init --lang python --name example --description "My first module"
fusion module init --lang csharp --name security --description "Security helpers"
fusion module init --lang rust --name auth --description "Auth helpers" ./fusion-auth-mod
```

**Recommended package naming** (not required):

| Host | Pattern | Example (`--name jwt`) |
|------|---------|-------------------------|
| Python | `fusion_<name>_mod` | `fusion_jwt_mod` → `from fusion_jwt_mod import ...` |
| npm/TS | `fusion-<name>-mod` | `fusion-jwt-mod` → `import { ... } from "fusion-jwt-mod"` |
| C# | `Fusion<Name>Mod` | `FusionJwtMod` → `using FusionJwtMod;` |

Every package gets a `fusion.module.toml` manifest and a small example export
(e.g. `hello()` / `Module.Hello()`). Replace that with your own library code.

#### Add a module from GitHub

From inside a Fusion project:

```bash
fusion add --github OWNER/MODULE_NAME
fusion add --github OWNER/MODULE_NAME@v1.0.0
fusion add --github https://github.com/OWNER/MODULE_NAME
```

This downloads the repo, validates `fusion.module.toml`, vendors it under
`.fusion/modules/<id>/`, runs the declared build/install steps (`pip install -e .`,
`maturin`, `npm`, or `dotnet build`), and records the module in `fusion-framework.toml` as
`[[modules]]`. TypeScript hosts get a `package.json` `file:` link; C# (`asp-core`)
hosts get `dotnet add reference` to the vendored `.csproj`.

Then import it:

```python
from fusion_example_mod import hello

print(hello("world"))
```

```csharp
using FusionExampleMod;

Console.WriteLine(Module.Hello("world"));
```

### Update

```bash
fusion update
```

Checks the latest GitHub release and, if it is newer, downloads the build for
your platform and replaces the running `fusion` binary in place. It works the
same on Linux, macOS and Windows, keeps the current install location, and needs
no reinstall or `PATH` change. Nothing happens if you are already up to date.

If `fusion` was installed to a system-wide directory, run the update with the
permissions needed to write there (for example `sudo fusion update`).

### Version

```bash
fusion --version
```

### Help

```bash
fusion --help
fusion init --help
fusion command --help
fusion module init --help
fusion add --help
```

## Project Structure

Running `fusion init` creates:

```
<project-directory>/
├── core/
│   └── settings.py          # Project settings
├── src/
│   └── modules/             # Application modules
├── main.py                  # Entry point
├── fusion-framework.toml    # Project configuration
├── fusion.dev.json          # Development environment
├── fusion.prod.json         # Production environment
├── fusion.stage.json        # Staging environment
└── .gitignore               # Git ignore rules (language-specific)
```

`main` and `core/settings` follow the extension of the selected language, so a
TypeScript project gets `main.ts` and `core/settings.ts` instead.

`core/settings.py` reads the `config` block of `fusion.<env>.json` from the
project root, where `<env>` comes from the `FUSION_ENV` environment variable and
defaults to `dev`:

```bash
python main.py                   # uses fusion.dev.json
FUSION_ENV=prod python main.py   # uses fusion.prod.json
```

## Environment Files

Each environment is one `fusion.<env>.json` in the project root. Add as many as
you like: a `fusion.test.json` becomes the `test` environment, no configuration
needed anywhere else.

```json
{
  "env": "stage",
  "config": { "port": 1010 },
  "commands": {
    "run": "docker compose up",
    "stop": "docker compose down"
  }
}
```

`config` is yours to shape and is what `core/settings` reads. `commands` holds
project commands that `fusion command` runs.

New projects also get a `swagger` block under `config` (enabled in `dev`, off in
`prod`/`stage`). Edit it to control the docs UI, OpenAPI info, auth, and navbar:

```json
{
  "env": "dev",
  "config": {
    "host": "127.0.0.1",
    "port": 8080,
    "swagger": {
      "enabled": true,
      "path": "/swagger",
      "title": "Fusion API Docs",
      "info": {
        "title": "Fusion API",
        "version": "1.0.0",
        "description": "API documentation generated by fusion-framework"
      },
      "servers": [{ "url": "/", "description": "Current host" }],
      "auth": {
        "persistAuthorization": true,
        "schemes": {
          "BearerAuth": { "type": "http", "scheme": "bearer", "bearerFormat": "JWT" },
          "ApiKeyAuth": { "type": "apiKey", "in": "header", "name": "X-API-Key" }
        },
        "global": [],
        "oauth": {
          "clientId": "",
          "appName": "Fusion API",
          "usePkceWithAuthorizationCodeGrant": true
        }
      },
      "navbar": {
        "enabled": true,
        "showUrlInput": true
      },
      "ui": {
        "deepLinking": true,
        "docExpansion": "list",
        "filter": true,
        "tryItOutEnabled": true
      }
    }
  },
  "commands": { "run": "python main.py" }
}
```

- `auth.schemes` / `auth.global` become OpenAPI security definitions
- `auth.oauth` is passed to Swagger UI `initOAuth`
- `navbar.enabled` turns on the Topbar (`StandaloneLayout`)
- `swagger.ui` maps to [Swagger UI configuration](https://swagger.io/docs/open-source-tools/swagger-ui/usage/configuration/)


## Development

### Prerequisites

- Rust (1.70+)
- Cargo

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run -- init
```

### Tests

```bash
cargo test
```

### Release

`fusion --version` reports the `Cargo.toml` version, so bump it first and then
push a matching tag. CI refuses to build a tag that disagrees with `Cargo.toml`:

```bash
# bump version = "1.0.4" in Cargo.toml first
git commit -am "release v1.0.4"
git tag v1.0.4
git push origin main && git push origin v1.0.4
```

This is the version of the tool only. The framework version that ends up in a
generated `fusion-framework.toml` is `FUSION_FRAMEWORK_VERSION` in
`src/setting/config.rs` and is bumped separately.

## License

MIT