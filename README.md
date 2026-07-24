# Fusion Tool

A CLI tool for creating and managing Fusion Framework projects. Supports interactive and non-interactive (batch) modes.

## Installation

### macOS
```bash
brew install fusion-tool
```
Or via curl:
```bash
curl -fsSL https://raw.githubusercontent.com/cipherunits/fusion-tool/main/install.sh | bash
```

### Linux
```bash
wget -qO- https://raw.githubusercontent.com/cipherunits/fusion-tool/main/install.sh | bash
```
Or via package manager:
```bash
sudo apt-get install fusion-tool
```

### Windows
```powershell
winget install fusion-tool
```
Or via PowerShell:
```powershell
Invoke-WebRequest -Uri https://raw.githubusercontent.com/cipherunits/fusion-tool/main/install.ps1 -OutFile install.ps1; .\install.ps1
```

## Commands

### Interactive Mode

Create a new Fusion Framework project with interactive prompts:

```bash
fusion init
```

This will prompt you for:
1. Project directory (defaults to current directory)
2. Programming language (Python, TypeScript, or ASP.NET Core)
3. Project name
4. Project description

### Non-Interactive Mode

Create a new project with all options specified via CLI flags:

```bash
fusion init --lang python --name myproject --description "My awesome project"
```

#### Available Flags for `fusion init`

| Flag | Type | Description | Required |
|------|------|-------------|----------|
| `--lang` | string | Programming language: `python`, `typescript`, `asp-core` | No (interactive mode will prompt) |
| `--name` | string | Project name | No (defaults to directory name) |
| `--description` | string | Project description | No (defaults to "A Fusion Framework project") |
| `--directory` | string | Target directory for the project | No (defaults to current directory) |

#### Non-Interactive Examples

```bash
# Create a Python project named "my-app" in current directory
fusion init --lang python --name my-app

# Create a TypeScript project in a specific directory
fusion init --lang typescript --name my-app --description "A TypeScript app"

# Create a project in a custom directory
fusion init --lang python --name test --directory ./my-projects/test-app
```

### Version

Print the version of Fusion Tool:

```bash
fusion --version
```

### Help

Print help information:

```bash
fusion --help
fusion init --help
```

## Project Structure

When `fusion init` is run, the following files and directories are created:

```
<project-directory>/
├── fusion-framework.toml    # Project configuration
├── fusiondev.json           # Development environment config
├── fusionprod.json          # Production environment config
├── fusionstage.json         # Staging environment config
├── .gitignore               # Git ignore rules for the chosen language
└── src/                     # Source directory (if applicable)
```

### fusion-framework.toml

The main configuration file for Fusion Framework projects:

```toml
[project]
name = "my-project"
description = "A Fusion Framework project"

[framework]
language = "python"
extension = ".py"
version = "0.1.0"

[tool]
version = "0.1.0"
```

## Development

### Prerequisites

- Rust (1.70+)
- Cargo

### Building from Source

```bash
# Clone the repository
git clone https://github.com/cipherunits/fusion-tool.git
cd fusion-tool

# Build
cargo build --release

# Run
cargo run -- init
```

### Running Tests

```bash
cargo test
```

## License

MIT