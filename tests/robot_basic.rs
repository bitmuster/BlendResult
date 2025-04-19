use std::fs;
mod common;

#[test]
fn test_stuff() {
    common::run_rf_test_a();
    let filename = "robot/results/output_a.xml";
    let xml = fs::read_to_string(filename).unwrap();
    blend_result::parse(&xml);
}
