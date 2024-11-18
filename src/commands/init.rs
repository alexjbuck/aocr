// src/commands/init.rs
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub async fn execute(path: PathBuf) -> Result<()> {
    // Create the workspace directory
    fs::create_dir_all(&path)?;

    // Create workspace Cargo.toml with all day crates as members
    let workspace_toml = r#"[workspace]
members = [
    "runner",".tmp*",
    "day01", "day02", "day03", "day04", "day05",
    "day06", "day07", "day08", "day09", "day10",
    "day11", "day12", "day13", "day14", "day15",
    "day16", "day17", "day18", "day19", "day20",
    "day21", "day22", "day23", "day24", "day25"
]
resolver = "2"

[workspace.dependencies]
anyhow = "1.0.75"
"#;
    fs::write(path.join("Cargo.toml"), workspace_toml)?;

    // Create runner crate
    create_runner_crate(&path)?;

    // Create all day crates
    for day in 1..=25 {
        create_day_crate(&path, day)?;
    }

    // Create .gitignore
    let gitignore = r#"# Generated by Cargo
/target/
Cargo.lock

# Editor specific files
.idea/
.vscode/
*.swp
*.swo

# macOS specific files
.DS_Store

# Project specific
/inputs/
.tmp*
"#;
    fs::write(path.join(".gitignore"), gitignore)?;

    // Initialize git repository
    init_git_repo(&path)?;

    println!(
        "Successfully initialized Advent of Code workspace at {:?}",
        path
    );
    Ok(())
}

fn init_git_repo(path: &Path) -> Result<()> {
    // Initialize git repository
    Command::new("git")
        .arg("init")
        .current_dir(path)
        .output()
        .context("Failed to initialize git repository")?;

    // Add all files
    Command::new("git")
        .args(["add", "."])
        .current_dir(path)
        .output()
        .context("Failed to stage files")?;

    // Initial commit
    Command::new("git")
        .args(["commit", "-m", "Initial commit: Advent of Code workspace"])
        .current_dir(path)
        .output()
        .context("Failed to create initial commit")?;

    Ok(())
}

fn create_runner_crate(workspace_path: &Path) -> Result<()> {
    let runner_path = workspace_path.join("runner");
    fs::create_dir_all(runner_path.join("src"))?;

    let runner_toml = r#"[package]
name = "runner"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow.workspace = true
"#;
    fs::write(runner_path.join("Cargo.toml"), runner_toml)?;

    let runner_main = r#"fn main() {
    let input = include_str!("INPUT_PATH");
    let result = TARGET_CRATE::partN(input);
    println!("Day {} Part {}: {}", DAY, PART, result);
}
"#;
    fs::write(runner_path.join("src").join("main.rs"), runner_main)?;

    Ok(())
}

fn create_day_crate(workspace_path: &Path, day: u8) -> Result<()> {
    let day_str = format!("day{:02}", day);
    let day_path = workspace_path.join(&day_str);
    fs::create_dir_all(day_path.join("src"))?;

    let day_toml = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow.workspace = true

[lib]
name = "{}"
path = "src/lib.rs"
"#,
        day_str, day_str
    );
    fs::write(day_path.join("Cargo.toml"), day_toml)?;

    let day_lib = format!(
        r#"//! Solution for Advent of Code 2024, Day {}

pub fn part1(input: &str) -> usize {{
    // TODO: Implement part 1 solution
    0
}}

pub fn part2(input: &str) -> usize {{
    // TODO: Implement part 2 solution
    0
}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn test_part1() {{
        let input = "";
        assert_eq!(part1(input), 0);
    }}

    #[test]
    fn test_part2() {{
        let input = "";
        assert_eq!(part2(input), 0);
    }}
}}
"#,
        day
    );
    fs::write(day_path.join("src").join("lib.rs"), day_lib)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Output;
    use tempfile::TempDir;

    fn git_command(dir: &Path, args: &[&str]) -> Result<Output> {
        Command::new("git")
            .args(args)
            .current_dir(dir)
            .output()
            .context("Failed to run git command")
    }

    #[tokio::test]
    async fn test_init_command() -> Result<()> {
        let temp_dir = TempDir::new()?;

        // Run init command
        execute(temp_dir.path().to_path_buf()).await?;

        // Verify workspace structure
        assert!(temp_dir.path().join("Cargo.toml").exists());
        assert!(temp_dir.path().join(".gitignore").exists());
        assert!(temp_dir.path().join("runner").exists());
        assert!(temp_dir.path().join("runner/src").exists());
        assert!(temp_dir.path().join("runner/src/main.rs").exists());
        assert!(temp_dir.path().join("runner/Cargo.toml").exists());

        // Verify all day crates are created
        for day in 1..=25 {
            let day_str = format!("day{:02}", day);
            let day_path = temp_dir.path().join(&day_str);

            assert!(day_path.exists(), "Day crate {} not created", day_str);
            assert!(
                day_path.join("src").exists(),
                "src directory missing for {}",
                day_str
            );
            assert!(
                day_path.join("src/lib.rs").exists(),
                "lib.rs missing for {}",
                day_str
            );
            assert!(
                day_path.join("Cargo.toml").exists(),
                "Cargo.toml missing for {}",
                day_str
            );

            // Verify Cargo.toml contents
            let cargo_contents = fs::read_to_string(day_path.join("Cargo.toml"))?;
            assert!(cargo_contents.contains(&format!("name = \"{}\"", day_str)));

            // Verify lib.rs contents
            let lib_contents = fs::read_to_string(day_path.join("src/lib.rs"))?;
            assert!(lib_contents.contains("pub fn part1"));
            assert!(lib_contents.contains("pub fn part2"));
            assert!(lib_contents.contains(&format!("Day {}", day)));
        }

        // Verify workspace Cargo.toml contains all crates
        let workspace_contents = fs::read_to_string(temp_dir.path().join("Cargo.toml"))?;
        for day in 1..=25 {
            assert!(workspace_contents.contains(&format!("\"day{:02}\"", day)));
        }

        // Verify git repository
        assert!(
            temp_dir.path().join(".git").exists(),
            "Git repository not initialized"
        );

        // Check git status
        let status = git_command(temp_dir.path(), &["status", "--porcelain"])?;
        assert!(
            String::from_utf8_lossy(&status.stdout).is_empty(),
            "Git repository has uncommitted changes"
        );

        // Check git log
        let log = git_command(temp_dir.path(), &["log", "--oneline"])?;
        let log_output = String::from_utf8_lossy(&log.stdout);
        assert!(
            log_output.contains("Initial commit"),
            "Missing initial commit"
        );

        Ok(())
    }
}
