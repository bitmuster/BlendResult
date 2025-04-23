use anyhow::anyhow;
use anyhow::Context;
use std::process;

pub fn run_rf_test(t: &str) -> anyhow::Result<()> {
    let output = process::Command::new("sh")
        .arg("-c")
        .arg("rm -f output_{t}.xml log_{t}.html report_{t}.html")
        .output()
        .context("failed to execute process : rm")?;
    println!(
        "Process Outputs:\n{}",
        String::from_utf8(output.stdout).context("Utf8 conversion failed")?
    );
    let output = process::Command::new("sh")
        .arg("-c")
        .arg(
            format!(
            ". venv/bin/activate && robot -d robot/results/ -o output_{t}.xml -l log_{t}.html -r report_{t}.html robot/test_{t}.robot")
        )
        .output()
        .context("Failed to execute process: robotframework")?;
    if !output.status.success() {
        println!(
            "Robot Error:\n{:?}",
            String::from_utf8(output.stderr).context("Utf8 conversion failed")?
        );
        return Err(anyhow!(String::from("Something")));
    };

    println!(
        "Robot Outputs:\n{}",
        String::from_utf8(output.stdout).context("Utf8 conversion failed")?
    );
    Ok(())
}
