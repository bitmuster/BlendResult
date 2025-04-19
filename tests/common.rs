use std::process;

pub fn run_rf_test_a() {
    let output = process::Command::new("sh")
        .arg("-c")
        .arg(
            ". venv/bin/activate;
            robot -d robot/results/ \
            -o output_a.xml -l log_a.html -r report_a.html\
            robot/test_a.robot
            ",
        )
        .output()
        .expect("failed to execute process");
    println!(
        "Robot Outputs:\n{}",
        String::from_utf8(output.stdout).unwrap()
    );
}
