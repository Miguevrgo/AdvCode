use std::env;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use std::process::{Command, Stdio};
use std::time::Duration;

pub fn run(only: Option<u32>) -> Result<(), Box<dyn Error>> {
    let launcher = Path::new(env!("CARGO_MANIFEST_DIR"));
    let runner = launcher.join("benchmark.cpp");
    let compiler = env::var_os("CXX").unwrap_or_else(|| "c++".into());
    let build = tempfile::tempdir()?;
    let binary = build.path().canonicalize()?.join("solution");

    for day in (1..=14).filter(|day| only.is_none_or(|only| only == *day)) {
        let source_dir = launcher.join(format!("../2023/Day{day:02}"));
        let mut elapsed = Duration::ZERO;

        // Day 1 selects its first part at compile time.
        let variants: &[&[&str]] = if day == 1 {
            &[&["-DAOC_DAY_01", "-DSOLUCIONPART1"], &["-DAOC_DAY_01"]]
        } else {
            &[&[]]
        };

        File::open(source_dir.join("input.txt"))
            .map_err(|error| format!("Day {day:02}: input.txt: {error}"))?;
        for variant in variants {
            let status = Command::new(&compiler)
                .args(["-std=c++17", "-O3", "-march=native", "-flto"])
                .arg(format!("-DAOC_SOURCE=\"problem{day}.cpp\""))
                .args(*variant)
                .arg("-I")
                .arg(&source_dir)
                .arg(&runner)
                .arg("-o")
                .arg(&binary)
                .status()
                .map_err(|error| {
                    format!("Could not start {}: {error}", compiler.to_string_lossy())
                })?;
            if !status.success() {
                return Err(format!("Day {day:02}: compilation failed ({status})").into());
            }

            let output = Command::new(&binary)
                .current_dir(&source_dir)
                .stdout(Stdio::null())
                .output()
                .map_err(|error| format!("Day {day:02}: could not start solution: {error}"))?;
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(format!(
                    "Day {day:02}: solution failed ({}): {}",
                    output.status,
                    stderr.trim()
                )
                .into());
            }
            let nanos = String::from_utf8_lossy(&output.stderr)
                .trim()
                .parse()
                .map_err(|error| format!("Day {day:02}: invalid duration: {error}"))?;
            elapsed += Duration::from_nanos(nanos);
        }

        println!("   - Day {day:02}: \t{elapsed:.2?}");
    }

    Ok(())
}
