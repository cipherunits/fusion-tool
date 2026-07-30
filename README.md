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
1. Project directory (defaults to current directory)
2. Programming language (Python, TypeScript, or ASP.NET Core)
3. Project name
4. Project description

### Non-Interactive Mode

```bash
fusion init --lang python --name myproject --description "My awesome project"
```

#### Available Flags for `fusion init`

| Flag | Type | Description | Required |
|------|------|-------------|----------|
| `--lang` | string | `python`, `typescript`, `asp-core` | No |
| `--name` | string | Project name | No |
| `--description` | string | Project description | No |
| `--directory` | string | Target directory | No |

#### Examples

```bash
fusion init --lang python --name my-app
fusion init --lang typescript --name my-app --description "A TypeScript app"
fusion init --lang python --name test --directory ./my-projects/test-app
```

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
├── fusion-framework.toml    # Project configuration
├── fusiondev.json           # Development environment config
├── fusionprod.json          # Production environment config
├── fusionstage.json         # Staging environment config
├── .gitignore               # Git ignore rules (language-specific)
└── src/                     # Source directory
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

## License

MIT