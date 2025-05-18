use std::fs;
mod common;
use anyhow::{self, Context};
use blend_result::blend;
use blend_result::element::*;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test_parser_a() -> anyhow::Result<()> {
    common::init_logger();
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
    common::init_logger();
    common::run_rf_test("b")?;
    let filename = "robot/results/output_b.xml";
    let csv_file = "robot/results/output_b.csv";
    let xml = fs::read_to_string(filename).context(format!("File not found {}", filename))?;
    let _results = blend_result::parse(&xml, csv_file).context("Parsing failed")?;
    Ok(())
}
#[test]
fn test_parse_from_str_to_str() -> anyhow::Result<()> {
    common::init_logger();
    common::run_rf_test("b")?;
    let filename = "robot/results/output_b.xml";
    let xml = fs::read_to_string(filename).context(format!("File not found {}", filename))?;
    let results = blend_result::parse_from_str_to_str(&xml).context("Parsing failed")?;

    let expect = "Type,Name,Result\n\
        Robot,,None\n\
        Suite,Test B,Pass\n\
        Test,Demo Test A,Pass\n\
        Keyword,Keyword A,Pass\n\
        Keyword,No Operation,Pass\n\
        Test,Demo Test B,Pass\n\
        Keyword,No Operation,Pass\n\
        Keyword,Keyword B,Pass\n\
        Keyword,Keyword A,Pass\n\
        Keyword,No Operation,Pass\n\
        Keyword,No Operation,Pass\n\
        Test,Demo Test C,Pass\n\
        Keyword,No Operation,Pass\n\
        Keyword,Keyword C,Pass\n\
        Keyword,Keyword B,Pass\n\
        Keyword,Keyword A,Pass\n\
        Keyword,No Operation,Pass\n\
        Keyword,No Operation,Pass\n\
        Keyword,No Operation,Pass\n\
        Test,Demo Test D,Pass\n\
        Keyword,Log To Console,Pass\n";

    assert_eq!(results, expect);
    Ok(())
}
#[test]
fn test_parse_from_str_to_str_c() -> anyhow::Result<()> {
    common::init_logger();
    common::run_rf_test_with_options("c", true, "_fail", "--variable failhere:True")?;
    let filename = "robot/results/output_c_fail.xml";
    let xml = fs::read_to_string(filename).context(format!("File not found {}", filename))?;
    let results = blend_result::parse_from_str_to_str(&xml).context("Parsing failed")?;

    let expect = "Type,Name,Result\n\
        Robot,,None\n\
        Suite,Test C,Fail\n\
        Test,Demo Test A,Pass\n\
        Keyword,Keyword A,Pass\n\
        Keyword,No Operation,Pass\n\
        Test,Demo Test B,Pass\n\
        Keyword,No Operation,Pass\n\
        Keyword,Keyword B,Pass\n\
        Keyword,Keyword A,Pass\n\
        Keyword,No Operation,Pass\n\
        Keyword,No Operation,Pass\n\
        Test,Demo Test C,Fail\n\
        Keyword,No Operation,Pass\n\
        If,,Fail\n\
        Branch,,Fail\n\
        Keyword,Keyword C,Pass\n\
        Keyword,Keyword B,Pass\n\
        Keyword,Keyword A,Pass\n\
        Keyword,No Operation,Pass\n\
        Keyword,No Operation,Pass\n\
        Keyword,No Operation,Pass\n\
        Keyword,Log,Pass\n\
        Keyword,Fail,Fail\n\
        Branch,,None\n\
        Keyword,Log,NotRun\n\
        Test,Demo Test D,Pass\n\
        Keyword,Log To Console,Pass\n";

    assert_eq!(results, expect);
    Ok(())
}
#[test]
fn test_parse_from_str_to_str_d() -> anyhow::Result<()> {
    common::init_logger();
    common::run_rf_test_with_options("d", true, "_fail", "--variable failhere:True")?;
    let filename = "robot/results/output_d_fail.xml";
    let xml = fs::read_to_string(filename).context(format!("File not found {}", filename))?;
    let results = blend_result::parse_from_str_to_str(&xml).context("Parsing failed")?;

    let expect = "Type,Name,Result\n\
        Robot,,None\n\
        Suite,Test D,Fail\n\
        Test,Demo Test D,Fail\n\
        Keyword,No Operation,Pass\n\
        Keyword,Fail,Fail\n\
        Keyword,No Operation,NotRun\n";

    assert_eq!(results, expect);
    Ok(())
}
#[test]
fn test_parser_c() -> anyhow::Result<()> {
    common::init_logger();
    common::run_rf_test_with_options("c", false, "_pass", "--variable failhere:False")?;
    common::run_rf_test_with_options("c", true, "_fail", "--variable failhere:True")?;

    let filename1 = "robot/results/output_c_pass.xml";
    let filename2 = "robot/results/output_c_fail.xml";
    let csv_file_blend = "robot/results/output_c_bledned.csv";
    let files = vec![filename1, filename2];
    let _ = blend(&files, csv_file_blend);
    Ok(())
}
