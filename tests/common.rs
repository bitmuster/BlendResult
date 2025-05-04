use anyhow::anyhow;
use anyhow::Context;
use std::process;

pub fn init_logger() {
    // Ignore result when logger is already initialised
    let _ = simple_logger::SimpleLogger::new().env().init();
}

pub fn run_rf_test(t: &str) -> anyhow::Result<()> {
    run_rf_test_with_options(t, true, "", "")
}

pub fn run_rf_test_with_options(
    t: &str,
    expect_fail: bool,
    appendix: &str,
    options: &str,
) -> anyhow::Result<()> {
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
            ". venv/bin/activate && robot {options} -d robot/results/ -o output_{t}{appendix}.xml -l log_{t}{appendix}.html -r report_{t}{appendix}.html robot/test_{t}.robot")
        )
        .output()
        .context("Failed to execute process: robotframework")?;
    if !output.status.success() {
        if expect_fail && output.status.code() == Some(1) {
        } else {
            return Err(anyhow!(
                "Call on shell failed: stdout:{:?} stderr:{:?} status: {:?}",
                String::from_utf8(output.stdout).context("Utf8 conversion failed")?,
                String::from_utf8(output.stderr).context("Utf8 conversion failed")?,
                output.status.code()
            ));
        }
    };

    println!(
        "Robot Outputs:\n{}",
        String::from_utf8(output.stdout).context("Utf8 conversion failed")?
    );
    Ok(())
}
