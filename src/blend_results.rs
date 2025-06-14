use std::cell::RefCell;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::rc::Rc;
use std::rc::Weak;

// use log::{debug, info, trace, warn};
use log::{debug, trace};

use anyhow::Context;
use quick_xml::reader::Reader;

use crate::element::{Element, ElementType, ResultList, ResultType};
use crate::multi_result_list::MultiResultList;
use crate::rf_parser::{diff_tree, dump_csv_to_str, dump_flat, parse_inner, ParserStats};

/// Blend XML files into a multiresult list and write a CSV file
pub fn blend_and_save_to_csv(xml_files: &Vec<String>, csv_file: &str) -> anyhow::Result<()> {
    let mut xml_data: Vec<String> = vec![];
    // Parse input files
    for xml_file in xml_files {
        println!("Parsing {}", xml_file);
        xml_data
            .push(fs::read_to_string(xml_file).context(format!("File not found {}", xml_file))?);
    }

    let result = blend(&xml_data)?;

    let mut buffer = File::create(csv_file)?;
    buffer.write(result.as_bytes())?;

    #[cfg(feature = "odson")]
    export_to_ods();

    Ok(())
}

/// Blend XML data into a multiresult list and generate a CSV string
pub fn blend(xml_data: &Vec<String>) -> anyhow::Result<String> {
    let mut trees: Vec<Element> = Vec::new();
    let mut results: Vec<ResultList> = Vec::new();
    let mut stats: Vec<ParserStats> = Vec::new();

    // Parse input files
    for xml in xml_data {
        let mut reader = Reader::from_str(&xml);
        reader.config_mut().trim_text(true);

        let depth = 0;
        let mut root_element: Element = Element {
            et: ElementType::Robot,
            children: RefCell::new(Vec::new()),
            parent: RefCell::new(Weak::new()),
            result: ResultType::None,
            name: String::new(),
        };
        let mut stat = ParserStats { max_depth: 0 };

        parse_inner(&mut reader, &mut root_element, depth, &mut stat)?;
        trees.push(root_element);
        debug!("Maximum tree depth {}", stat.max_depth);
        stats.push(stat);
    }

    // Dump flat contents just as reference to compare
    for tree in trees.iter() {
        let mut result = ResultList {
            list: Rc::new(RefCell::new(Vec::new())),
        };
        dump_flat(tree, &mut result);
        debug!("Parsed {} flat elements", result.list.borrow().len());
        results.push(result)
    }

    // Dump unblended csv
    for result in results {
        for robot_result in result.list.borrow().iter() {
            trace!("Result contents: {robot_result:?}")
        }
        let csv_str = dump_csv_to_str(&result)?;
        debug!("{csv_str}");
    }

    let trees_to_diff: Vec<Option<&Element>> = trees.iter().map(|t| Some(t)).collect();

    let mrl = MultiResultList::new(trees.len());
    diff_tree(&trees_to_diff, &mrl, 0)?;
    //println!("{:?}",mrl);

    // println!("{}", mrl.dump_to_csv_str().unwrap());

    Ok(mrl.dump_to_csv_str()?)
}

#[cfg(feature = "odson")]
use icu_locid::locale;
#[cfg(feature = "odson")]
use spreadsheet_ods::color::Rgb;
#[cfg(feature = "odson")]
use spreadsheet_ods::format;
#[cfg(feature = "odson")]
use spreadsheet_ods::formula;
#[cfg(feature = "odson")]
use spreadsheet_ods::mm;
#[cfg(feature = "odson")]
use spreadsheet_ods::style::units::{Border, TextRelief};
#[cfg(feature = "odson")]
use spreadsheet_ods::style::CellStyle;
#[cfg(feature = "odson")]
use spreadsheet_ods::{Sheet, Value, WorkBook};

#[cfg(feature = "odson")]
pub fn export_to_ods() {
    fs::create_dir_all("test_out").expect("create_dir");

    let path = std::path::Path::new("test_out/lib_example.ods");
    let mut wb = if path.exists() {
        spreadsheet_ods::read_ods(path).unwrap()
    } else {
        WorkBook::new(locale!("en_US"))
    };

    if wb.num_sheets() == 0 {
        let mut sheet = Sheet::new("one");
        sheet.set_value(0, 0, true);
        sheet.set_value(1, 0, "so zeug da");
        sheet.set_value(2, 0, "so zeug da");
        sheet.set_value(3, 0, "so zeug da");
        wb.push_sheet(sheet);
    }

    spreadsheet_ods::write_ods(&mut wb, "test_out/lib_example.ods").expect("write_ods");
}
