use std::fs;
mod common;
use anyhow::{self, Context};
use blend_result::element::*;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test_parser_a() -> anyhow::Result<()> {
    common::run_rf_test("a")?;
    let filename = "robot/results/output_a.xml";
    let csv_file = "robot/results/output_a.csv";
    let xml = fs::read_to_string(filename).context(format!("File not found {}", filename))?;
    let results = blend_result::parse(&xml, csv_file).context("Parsing failed")?;

    let expect = ResultList {
        list: Rc::new(RefCell::new(vec![
            ElementFlat {
                et: ElementType::Robot,
                result: ResultType::None,
                name: String::from(""),
            },
            ElementFlat {
                et: ElementType::Suite,
                result: ResultType::Pass,
                name: String::from("Test A"),
            },
            ElementFlat {
                et: ElementType::Test,
                result: ResultType::Pass,
                name: String::from("Demo Test A"),
            },
            ElementFlat {
                et: ElementType::Keyword,
                result: ResultType::Pass,
                name: String::from("No Operation"),
            },
            ElementFlat {
                et: ElementType::Test,
                result: ResultType::Pass,
                name: String::from("Demo Test B"),
            },
            ElementFlat {
                et: ElementType::Keyword,
                result: ResultType::Pass,
                name: String::from("Keyword B"),
            },
            ElementFlat {
                et: ElementType::Keyword,
                result: ResultType::Pass,
                name: String::from("No Operation"),
            },
        ])),
    };
    assert_eq!(results, expect);
    Ok(())
}
#[test]
fn test_parser_b() -> anyhow::Result<()> {
    common::run_rf_test("b")?;
    let filename = "robot/results/output_b.xml";
    let csv_file = "robot/results/output_b.csv";
    let xml = fs::read_to_string(filename).context(format!("File not found {}", filename))?;
    let results = blend_result::parse(&xml, csv_file).context("Parsing failed")?;
    Ok(())
}
