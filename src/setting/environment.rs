use std::fs;
use std::path::Path;
use console::style;
use crate::setting::{config, get};

pub fn environment_file(env: &str, config: &str, target_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let content = format!(
        r#"{{
    "env": "{}",
    "config": {},
    "commands": {{}}
}}"#,
        env, config
    );
    let filename = target_dir.join(format!("fusion{}.json", env));
    fs::write(&filename, content)?;
    println!(
        "{}",
        style(format!("✔ {} created successfully!", filename.display()))
            .green()
            .bold()
    );
    Ok(())
}

pub fn dev(target_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    environment_file("dev", r#"{ "port": 8080 }"#, target_dir)?;
    Ok(())
}

pub fn prod(target_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    environment_file("prod", r#"{ "port": 9090 }"#, target_dir)?;
    Ok(())
}

pub fn stage(target_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    environment_file("stage", r#"{ "port": 1010 }"#, target_dir)?;
    Ok(())
}

pub fn git(target_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let ext = get::extension();
    let git_content = if ext == config::Language::Python.extension() {
        r#"
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
    } else if ext == config::Language::TypeScript.extension() {
        r#"
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
tests/baselines/local.old/*
tests/baselines/reference/*
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
    } else {
        ""
    };

    if !git_content.is_empty() {
        let gitignore_path = target_dir.join(".gitignore");
        fs::write(&gitignore_path, git_content)?;
        println!(
            "{}",
            style(format!("✔ {} created successfully", gitignore_path.display()))
                .green()
                .bold()
        );
    }
    Ok(())
}