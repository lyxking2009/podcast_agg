use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

pub struct CommitOptions<'a> {
    pub repo: &'a Path,
    pub message: &'a str,
    pub paths: &'a [&'a Path],
    pub push: bool,
}

pub fn commit_and_push(opts: CommitOptions<'_>) -> Result<bool> {
    if !is_git_repo(opts.repo) {
        tracing::warn!("not a git repo, skipping commit");
        return Ok(false);
    }

    for p in opts.paths {
        run_git(opts.repo, &["add", &p.to_string_lossy()])?;
    }

    let dirty = run_git_capture(opts.repo, &["status", "--porcelain"])?;
    if dirty.trim().is_empty() {
        tracing::info!("nothing to commit");
        return Ok(false);
    }

    run_git(opts.repo, &["commit", "-m", opts.message]).context("git commit")?;

    if opts.push {
        if let Err(e) = run_git(opts.repo, &["push"]) {
            tracing::warn!(error = %e, "git push failed (continuing)");
        }
    }
    Ok(true)
}

fn is_git_repo(p: &Path) -> bool {
    p.join(".git").exists()
}

fn run_git(repo: &Path, args: &[&str]) -> Result<()> {
    let status = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .status()
        .with_context(|| format!("running git {args:?}"))?;
    if !status.success() {
        anyhow::bail!("git {args:?} exited with {status}");
    }
    Ok(())
}

fn run_git_capture(repo: &Path, args: &[&str]) -> Result<String> {
    let out = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .output()
        .with_context(|| format!("running git {args:?}"))?;
    if !out.status.success() {
        anyhow::bail!(
            "git {args:?} exited with {}: {}",
            out.status,
            String::from_utf8_lossy(&out.stderr)
        );
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}
