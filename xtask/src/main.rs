use std::{
    fs,
    path::Path,
    io::Write,
    process::{Command, exit}
};

use anyhow::{Context, Result};
use clap::{arg, Parser, Subcommand};

fn main() -> Result<()> {
    let Cli { cmd } = Cli::parse();
    match cmd {
        Cmd::Scaffold { n } => scaffold(n)?,
        Cmd::Solve { n } => cargo_run(n)?,
        Cmd::All => run_all()?,
        Cmd::Time { n, iters } => time_it(n, iters)?
    }
    Ok(())
}

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd
}

#[derive(Subcommand)]
enum Cmd {
    /// Create src/bin/problem_<nnn>.rs with a skeletal function
    Scaffold { n: u32 },

    /// Compile and run one binary problem
    Solve { n: u32 },

    /// Run every solved problem binary and report timings
    All,

    /// Time a single problem with multiple iterations
    Time { 
        n: u32,
        #[arg(short, long, default_value_t=100)]
        iters: u32
    }
}

fn scaffold(n: u32) -> Result<()> {
    let dir = Path::new("project_euler").join("src/bin");
    let file = dir.join(format!("problem_{n:03}.rs"));

    if file.exists() {
        eprintln!("✗ {}", file.display());
        anyhow::bail!("file already exists")
    }

    fs::create_dir_all(&dir)?;

    let mut f = fs::File::create(&file)?;
    writeln!(f, "// Solution to problem {n:03}")?;
    writeln!(f, "// https://projecteuler.net/problem={n}")?;
    writeln!(f, "")?;
    writeln!(f, "project_euler::solution!({n});")?;
    writeln!(f, "")?;
    writeln!(f, "pub fn solve_problem_{n}() -> Option<u64> {{")?;
    writeln!(f, "    None")?;
    writeln!(f, "}}")?;
    writeln!(f, "")?;
    writeln!(f, "")?;
    writeln!(f, "#[cfg(test)]")?;
    writeln!(f, "mod tests {{")?;
    writeln!(f, "    use super::*;")?;
    writeln!(f, "")?;
    writeln!(f, "    #[test]")?;
    writeln!(f, "    fn solves_official_answer() {{")?;
    writeln!(f, "        assert_eq!(solve_problem_{n}(), None);")?;
    writeln!(f, "    }}")?;
    writeln!(f, "}}")?;

    println!("✓ {}", file.display());
    Ok(())
}

fn cargo_run(n: u32) -> Result<()> {
    let bin = format!("problem_{n:03}");
    status(Command::new("cargo").args(["run", "--quiet", "--bin", &bin]))?;
    Ok(())
}

fn run_all() -> Result<()> {
    let bin_dir = Path::new("euler").join("src/bin");
    for entry in fs::read_dir(&bin_dir).with_context(|| "no bin dir (scaffold something first)")? {
        let path = entry?.path();
        if path.extension() == Some("rs".as_ref()) {
            let name = path.file_stem().unwrap().to_string_lossy();
            println!("▶ {name}");
            status(Command::new("cargo").args(["run", "--quiet", "--bin", &name]))?;
        }
    }
    Ok(())
}

fn time_it(n: u32, iters: u32) -> Result<()> {
    let bin = format!("problem_{n:03}");
    let mut times = Vec::with_capacity(iters as usize);

    for _ in 0..iters {
        let t0 = std::time::Instant::now();
        status(Command::new("cargo").args(["run", "--quiet", "--bin", &bin]))?;
        times.push(t0.elapsed());
    }
    let mean = times.iter().sum::<std::time::Duration>() / iters;
    println!("mean over {iters} runs: {mean:?}");
    Ok(())
}

fn status(cmd: &mut Command) -> Result<()> {
    let ok = cmd.status().with_context(|| format!("failed to run {cmd:?}"))?;
    if !ok.success() {
        exit(ok.code().unwrap_or(1));
    }
    Ok(())
}