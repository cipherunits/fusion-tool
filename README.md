# Fusion Tool

A CLI tool for creating and managing Fusion Framework projects. Supports both interactive and non-interactive (batch) modes.

## Installation

### Linux

```bash
wget -qO- https://raw.githubusercontent.com/cipherunits/fusion-tool/main/scripts/install.sh | bash
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

Download the latest release from [GitHub Releases](https://github.com/cipherunits/fusion-tool/releases), extract `fusion.exe`, and run it.

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