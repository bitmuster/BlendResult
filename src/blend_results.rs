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

use crate::element::{Element, ElementFlat, ElementType, ResultList, ResultType};
use crate::multi_result_list::MultiResultList;
use crate::rf_parser::{diff_tree, dump_csv_to_str, dump_flat, parse_inner, ParserStats};

/// Blend XML files into a multiresult list and write a CSV file
pub fn blend_and_save_to_csv(
    xml_files: &Vec<String>,
    csv_file: &str,
    max_depth: usize,
) -> anyhow::Result<()> {
    let mut xml_data: Vec<String> = vec![];
    // Parse input files
    for xml_file in xml_files {
        println!("Parsing {}", xml_file);
        xml_data
            .push(fs::read_to_string(xml_file).context(format!("File not found {}", xml_file))?);
    }

    let mrl = blend(&xml_data, &xml_files, max_depth)?;
    let _data = mrl.export_to_ods()?;
    let result = mrl.dump_to_csv_str()?;

    let mut buffer = File::create(csv_file)?;
    buffer.write(result.as_bytes())?;

    #[cfg(feature = "odson")]
    Ok(())
}

/// Blend XML data into a multiresult list and generate a CSV string
pub fn blend(
    xml_data: &Vec<String>,
    xml_files: &Vec<String>,
    max_depth: usize,
) -> anyhow::Result<MultiResultList> {
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
        let mut stat = ParserStats { max_depth: 1 };

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
    let header = xml_files
        .iter()
        .map(|f| {
            Some(ElementFlat {
                et: ElementType::File,
                result: ResultType::None,
                name: f.to_string(),
                depth: 0,
            })
        })
        .collect();
    mrl.push(header)?;
    diff_tree(&trees_to_diff, &mrl, 0, max_depth)?;
    //println!("{:?}",mrl);

    // println!("{}", mrl.dump_to_csv_str().unwrap());
    Ok(mrl)
}
