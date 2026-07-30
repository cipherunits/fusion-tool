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
├── fusiondev.json           # Development environment config
├── fusionprod.json          # Production environment config
├── fusionstage.json         # Staging environment config
└── .gitignore               # Git ignore rules (language-specific)
```

`main` and `core/settings` follow the extension of the selected language, so a
TypeScript project gets `main.ts` and `core/settings.ts` instead.

`core/settings.py` reads the `config` block of `fusion<env>.json` from the
project root, where `<env>` comes from the `FUSION_ENV` environment variable and
defaults to `dev`:

```bash
python main.py              # uses fusiondev.json
FUSION_ENV=prod python main.py   # uses fusionprod.json
```

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
# bump version = "1.0.3" in Cargo.toml first
git commit -am "release v1.0.3"
git tag v1.0.3
git push origin main && git push origin v1.0.3
```

This is the version of the tool only. The framework version that ends up in a
generated `fusion-framework.toml` is `FUSION_FRAMEWORK_VERSION` in
`src/setting/config.rs` and is bumped separately.

## License

MIT