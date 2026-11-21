use std::env;
use std::error::Error;
use std::path::Path;
use std::process::Command;
use std::time::Duration;

pub fn run(only: Option<u32>) -> Result<(), Box<dyn Error>> {
    let source_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../2021/src");
    let zeru = env::var_os("ZERU").unwrap_or_else(|| "zeru".into());
    let output = Command::new(&zeru)
        .args(["run", "--release-fast", "main.zr", "--"])
        .arg(only.unwrap_or(0).to_string())
        .current_dir(&source_dir)
        .output()
        .map_err(|error| format!("Could not start {}: {error}", zeru.to_string_lossy()))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr)
            .trim()
            .to_string()
            .into());
    }
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        let (day, nanos) = line.split_once(' ').ok_or("main.zr printed no time")?;
        let elapsed = Duration::from_nanos(nanos.parse()?);
        println!("   - Day {day:0>2}: \t{elapsed:.2?}");
    }
    Ok(())
}
