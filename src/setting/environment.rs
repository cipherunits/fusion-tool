use console::style;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs;

use crate::setting::{config, get};

#[derive(Serialize, Deserialize, Debug)]
struct ProjectConfig {
    env: String,
    config: Config,
    command: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    port: u16,
}

pub fn dev() -> Result<(), Box<dyn std::error::Error>> {
    let project = ProjectConfig {
        env: "dev".to_string(),

        config: Config { port: 8080 },

        command: json!({}),
    };

    let json = serde_json::to_string_pretty(&project)?;

    fs::write("fusiondev.json", json)?;

    println!(
        "{}",
        style("✔ fusiondev.json created successfully!")
            .green()
            .bold()
    );

    Ok(())
}
pub fn prod() -> Result<(), Box<dyn std::error::Error>> {
    let project = ProjectConfig {
        env: "prod".to_string(),

        config: Config { port: 9090 },

        command: json!({}),
    };

    let json = serde_json::to_string_pretty(&project)?;

    fs::write("fusionprod.json", json)?;
    println!(
        "{}",
        style("✔ fusionprod.json created successfully!")
            .green()
            .bold()
    );

    Ok(())
}

pub fn stage() -> Result<(), Box<dyn std::error::Error>> {
    let project = ProjectConfig {
        env: "stage".to_string(),

        config: Config { port: 1010 },

        command: json!({}),
    };

    let json = serde_json::to_string_pretty(&project)?;

    fs::write("fusionstage.json", json)?;

    println!(
        "{}",
        style("✔ fusionstage.json created successfully!")
            .green()
            .bold()
    );

    Ok(())
}

pub fn git() -> Result<(), Box<dyn std::error::Error>> {
    let mut git_content = String::from("");
    if get::extension() == config::Language::Python.extension() {
        git_content = r#"
        
# Byte-compiled / optimized / DLL files
__pycache__/
*.py[codz]
*$py.class


# C extensions
*.so

# Distribution / packaging
.Python
build/
develop-eggs/
dist/
downloads/
eggs/
.eggs/
lib/
lib64/
parts/
sdist/
var/
wheels/
share/python-wheels/
*.egg-info/
.installed.cfg
*.egg
MANIFEST

# Environments
.env
.envrc
.venv
env/
venv/
ENV/
env.bak/
venv.bak/

# mypy
.mypy_cache/
.dmypy.json
dmypy.json"#
.to_owned();
    } else if get::extension() == config::Language::TypeScript.extension() {
        git_content = r#"
node_modules/
.node_modules/
built/*
tests/cases/rwc/*
tests/cases/perf/*
!tests/cases/webharness/compilerToString.js
test-args.txt
~*.docx
\#*\#
.\#*
tests/baselines/local/*
tests/baselines/local.old/*
tests/services/baselines/local/*
tests/baselines/prototyping/local/*
tests/baselines/rwc/*
tests/baselines/reference/projectOutput/*
tests/baselines/local/projectOutput/*
tests/baselines/reference/testresults.tap
tests/baselines/symlinks/*
tests/services/baselines/prototyping/local/*
tests/services/browser/typescriptServices.js
src/harness/*.js
src/compiler/diagnosticInformationMap.generated.ts
src/compiler/diagnosticMessages.generated.json
src/parser/diagnosticInformationMap.generated.ts
src/parser/diagnosticMessages.generated.json
rwc-report.html
*.swp
build.json
*.actual
tests/webTestServer.js
tests/webTestServer.js.map
tests/webhost/*.d.ts
tests/webhost/webtsc.js
tests/cases/**/*.js
tests/cases/**/*.js.map
*.config
scripts/eslint/built/
scripts/debug.bat
scripts/run.bat
scripts/**/*.js
scripts/**/*.js.map
coverage/
internal/
**/.DS_Store
.settings
**/.vs
**/.vscode/*
!**/.vscode/tasks.json
!**/.vscode/settings.template.json
!**/.vscode/launch.template.json
!**/.vscode/extensions.json
!tests/cases/projects/projectOption/**/node_modules
!tests/cases/projects/NodeModulesSearch/**/*
!tests/baselines/reference/project/nodeModules*/**/*
.idea
yarn.lock
yarn-error.log
.parallelperf.*
tests/baselines/reference/dt
.failed-tests
TEST-results.xml
package-lock.json
.eslintcache
*v8.log
/lib/"#
.to_owned();
    }

    fs::write(".gitignore", &git_content)?;
    println!(
        "{}",
        style("✔ .gitignore created successfully").green().bold()
    );
    Ok(())
}
