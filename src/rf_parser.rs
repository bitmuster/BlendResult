use anyhow::Context;
use csv::Writer;
use log::{debug, info, trace, warn};
use quick_xml::events::attributes;
use quick_xml::events::attributes::AttrError;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::any;
use std::cell::RefCell;
use std::fs;
use std::io;
use std::iter::zip;
use std::rc::Rc;
use std::rc::Weak;
use std::str;

use crate::element::{Element, ElementFlat, ElementType, ResultList, ResultType};

#[allow(dead_code)]
#[derive(Debug)]
enum AppError {
    Xml(quick_xml::Error),
    NoText(String),
}

impl From<quick_xml::Error> for AppError {
    fn from(error: quick_xml::Error) -> Self {
        Self::Xml(error)
    }
}

impl From<AttrError> for AppError {
    fn from(error: AttrError) -> Self {
        Self::Xml(quick_xml::Error::InvalidAttr(error))
    }
}

fn print_attributes(ident: &str, attr: attributes::Attributes) {
    for a in attr {
        let key = str::from_utf8(a.clone().unwrap().key.local_name().into_inner()).unwrap();
        let value = a.unwrap().unescape_value().unwrap();
        debug!("{ident}    Attr: {:?} {:?}", key, value);
    }
}

fn get_attr_name<'a>(name: &'a str, attr: attributes::Attributes<'a>) -> String {
    for a in attr {
        let key = str::from_utf8(a.clone().unwrap().key.local_name().into_inner()).unwrap();
        let value = a.unwrap().unescape_value().unwrap();
        if name == key {
            return value.to_string();
        }
    }
    "".to_string()
    //panic!("Cannot get attr {name}");
}

fn status_to_result(status: &str) -> ResultType {
    match status {
        "PASS" => ResultType::Pass,
        "FAIL" => ResultType::Fail,
        "NOT RUN" => ResultType::NotRun,
        s => {
            info!("Panic:  \"{s}\"");
            panic!("Panic:  \"{s}\"");
            // ResultType::None
        }
    }
}

struct ParserStats {
    max_depth: usize,
}

/// Slightly cursed parser for output.xml files
fn parse_inner(
    reader: &mut Reader<&[u8]>,
    element: &mut Element,
    depth: usize,
    stats: &mut ParserStats,
) -> anyhow::Result<()> {
    let mut buf = Vec::new();

    if depth > stats.max_depth {
        stats.max_depth = depth;
    }
    loop {
        let ident = " ".repeat(depth * 4 + 4);
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),

            Ok(Event::Eof) => {
                // println!("EOF");
                break;
            }

            Ok(Event::Start(e)) => {
                // println!("  Start {}", any::type_name_of_val(&e));
                debug!(
                    "{ident}Start: {}",
                    str::from_utf8(e.local_name().as_ref()).unwrap()
                );
                print_attributes(&ident, e.attributes());
                let name = get_attr_name("name", e.attributes());
                let mut et: Option<ElementType> = None;
                match e.name().as_ref() {
                    b"robot" => (),
                    b"suite" => et = Some(ElementType::Suite),
                    b"test" => et = Some(ElementType::Test),
                    b"kw" => et = Some(ElementType::Keyword),
                    b"if" => et = Some(ElementType::If),
                    b"branch" => et = Some(ElementType::Branch),
                    b"try" => et = Some(ElementType::Try),
                    b"for" => et = Some(ElementType::For),
                    b"iter" => et = Some(ElementType::Iter),
                    b"while" => et = Some(ElementType::While),
                    b"continue" => et = Some(ElementType::Continue),
                    b"doc" => (),
                    b"arg" => (),
                    b"statistics" => break,
                    b"total" => (),
                    b"errors" => (),
                    b"stat" => (),
                    b"tag" => (),
                    b"msg" => (),
                    b"var" => (),
                    b"return" => (),
                    b"value" => (),
                    b"break" => (),
                    b"status" => (),
                    // At least in one example a "pattern" appeared here instead of at End
                    b"pattern" => break,
                    s => {
                        warn!("Unmatched {:?}", str::from_utf8(s).unwrap());
                        panic!()
                    }
                }
                if e.name().as_ref() == b"status" {
                    let status = get_attr_name("status", e.attributes());
                    debug!("{ident}Got status from Start Element {:?}", status);
                    element.result = status_to_result(&status);
                }

                if let Some(e) = et {
                    let mut suite_element = Element {
                        et: e,
                        children: RefCell::new(Vec::new()),
                        parent: RefCell::new(Weak::new()),
                        result: ResultType::None,
                        name,
                    };
                    parse_inner(reader, &mut suite_element, depth + 1, stats)?;
                    let mut parent = element.parent.borrow_mut();
                    let rc_suite_element = Rc::new(suite_element);
                    *parent = Rc::downgrade(&rc_suite_element);
                    element.children.borrow_mut().push(rc_suite_element);
                }
            }
            Ok(Event::Text(e)) => {
                //println!("{ident}Text {}", any::type_name_of_val(&e));
                let text: &str = &e.unescape().unwrap();
                let len = usize::min(text.len(), 30);
                debug!("{ident}    Text: {} ...", text.get(0..len).unwrap());
            }
            Ok(Event::End(e)) => {
                // End means elements that end without having sub elements

                // println!("  End {}", any::type_name_of_val(&e));
                let ident = " ".repeat(depth * 4 + 4);
                debug!(
                    "{ident}End: {}",
                    str::from_utf8(e.local_name().as_ref()).unwrap()
                );

                match e.name().as_ref() {
                    b"robot" => break,
                    b"suite" => break,
                    b"test" => break,
                    b"kw" => break,
                    b"branch" => break,
                    b"if" => break,
                    b"try" => break,
                    b"for" => break,
                    b"iter" => break,
                    b"while" => break,
                    b"pattern" => break,
                    _ => (),
                }
            }
            // Empty means that the element has no subelements, only attributes
            Ok(Event::Empty(e)) => {
                // println!("{ident}Empty {}", any::type_name_of_val(&e));
                debug!(
                    "{ident}Empty: {}",
                    str::from_utf8(e.local_name().as_ref()).unwrap()
                );

                match e.name().as_ref() {
                    b"timeout" => continue,
                    b"status" => (),
                    b"var" => (),
                    s => panic!("Cannot parse {}", str::from_utf8(s).unwrap()),
                }

                print_attributes(&ident, e.attributes());
                match element.et {
                    ElementType::Keyword | ElementType::Suite | ElementType::Test => {
                        let status = get_attr_name("status", e.attributes());
                        debug!("{ident}Got status from Empty element {:?}", status);
                        element.result = status_to_result(&status);
                    }
                    _ => (),
                }
            }
            Ok(Event::Decl(e)) => {
                debug!("{ident}Decl {}", any::type_name_of_val(&e));
            }

            /*            Ok(x) => {
                println!("Ok {:?}", any::type_name_of_val(&x));
            }*/
            x => {
                warn!("No Type {:?}", any::type_name_of_val(&x));
            }
        }
        //println!("{:?}", str::from_utf8(&buf));
        buf.clear();
    }
    Ok(())
}

/// Should iterate over multiple trees of Elements to compare
/// We are getting N trees and we want to compare each of the child elements
/// This is similar to a generic N-times-zip function
pub fn diff_tree(elements: &[&Element]) -> anyhow::Result<()> {
    // The results we accumulate
    // let result_list = Vec::new();

    // The list of elements we want to compare

    //let element_list : Vec<&Element>= Vec::new();

    for element in elements {
/*        debug!(
            "Element {:?} {:?} {:?}",
            element.et,
            element.name,
            element.children.borrow().len()
        );*/
        //let wtf = element.children.borrow_mut().get(0).unwrap();
        //element_list.push(&wtf);
    }

    let u = &elements[0];
    let v = &elements[1];

    /*
        // if name == name
        for child in element.children.borrow().iter() {
            debug!("Element {:?} {:?}", element.et, element.name);
            diff_tree_inner(child)?;
        }
    */

    for (x, y) in u.children.borrow().iter().zip(v.children.borrow().iter()) {
        debug!("name: x, y {:?} {:?}", x.name, y.name);
        debug!("    type : x, y {:?} {:?}", x.et, y.et);
        debug!("    result:  x, y {:?} {:?}", x.result, y.result);
        let m: &Element = &x;
        let n: &Element = &y;
        diff_tree(&vec![m, n]);
    }

    Ok(())
}
/*
/// Should iterate over multiple trees of Elements to compare
pub fn diff_tree_weg(elements: &[Element]) -> anyhow::Result<()> {
    for element in elements {
        debug!("*** Root-tree Element from list {:?} ***", element.name);
        diff_tree_inner(element)?;
    }

    Ok(())
}
*/
pub fn blend(xml_files: &[&str], csv_file: &str) -> anyhow::Result<ResultList> {
    let mut trees: Vec<Element> = Vec::new();
    let mut results: Vec<ResultList> = Vec::new();
    let mut stats: Vec<ParserStats> = Vec::new();

    for xml_file in xml_files {
        println!("Parsing {}", xml_file);
        let xml_data =
            fs::read_to_string(xml_file).context(format!("File not found {}", xml_file))?;

        let mut reader = Reader::from_str(&xml_data);
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
        println!("Maximum tree depth {}", stat.max_depth);
        stats.push(stat);
    }

    // println!("Root {:#?}", root_element);
    // println!("{:?}", current);

    for tree in trees.iter() {
        let mut result = ResultList {
            list: Rc::new(RefCell::new(Vec::new())),
        };
        dump_flat(&tree, &mut result);
        println!("Parsed {} flat elements", result.list.borrow().len());
        results.push(result)
    }

    for result in results {
        for robot_result in result.list.borrow().iter() {
            trace!("Result contents: {robot_result:?}")
        }
        let csv_str = dump_csv_to_str(&result)?;
        debug!("{csv_str}");
    }
    let whatever = vec![&trees[0], &trees[1]];
    diff_tree(&whatever)?;

    let result = ResultList {
        list: Rc::new(RefCell::new(Vec::new())),
    };
    Ok(result)
}

pub fn parse(xml_data: &str, csv_file: &str) -> anyhow::Result<ResultList> {
    let mut reader = Reader::from_str(xml_data);
    reader.config_mut().trim_text(true);

    let depth = 0;
    let mut root_element: Element = Element {
        et: ElementType::Robot,
        children: RefCell::new(Vec::new()),
        parent: RefCell::new(Weak::new()),
        result: ResultType::None,
        name: String::new(),
    };
    let mut stats = ParserStats { max_depth: 0 };

    parse_inner(&mut reader, &mut root_element, depth, &mut stats)?;

    // println!("Root {:#?}", root_element);
    // println!("{:?}", current);

    let mut results = ResultList {
        list: Rc::new(RefCell::new(Vec::new())),
    };
    dump_flat(&root_element, &mut results);
    /*
    for result in results.list.borrow().iter() {
        println!("{result:?}")
    }*/
    dump_csv(csv_file, &results)?;
    println!("Parsed {} elements", results.list.borrow().len());
    println!("Maximum tree depth {}", stats.max_depth);
    Ok(results)
}

pub fn parse_from_str_to_str(xml_data: &str) -> anyhow::Result<String> {
    let mut reader = Reader::from_str(xml_data);
    reader.config_mut().trim_text(true);

    let depth = 0;
    let mut root_element: Element = Element {
        et: ElementType::Robot,
        children: RefCell::new(Vec::new()),
        parent: RefCell::new(Weak::new()),
        result: ResultType::None,
        name: String::new(),
    };
    let mut stats = ParserStats { max_depth: 0 };

    parse_inner(&mut reader, &mut root_element, depth, &mut stats)?;

    // println!("Root {:#?}", root_element);
    // println!("{:?}", current);

    let mut results = ResultList {
        list: Rc::new(RefCell::new(Vec::new())),
    };
    dump_flat(&root_element, &mut results);
    /*
    for result in results.list.borrow().iter() {
        println!("{result:?}")
    }*/

    Ok(dump_csv_to_str(&results)?)
}

fn dump_csv(csv_file: &str, results: &ResultList) -> anyhow::Result<()> {
    //let mut wtr = csv::Writer::from_writer(io::stdout());
    let mut wtr = csv::Writer::from_path(csv_file)?;

    wtr.write_record(&["Type", "Name", "Result"])?;
    for child in results.list.borrow().iter() {
        wtr.write_record(&[
            format!("{:?}", child.et),
            format!("{}", child.name),
            format!("{:?}", child.result),
        ])?;
    }

    wtr.flush()?;
    Ok(())
}

fn dump_csv_to_str(results: &ResultList) -> anyhow::Result<String> {
    let mut wtr = Writer::from_writer(vec![]);
    wtr.write_record(&["Type", "Name", "Result"])?;
    for child in results.list.borrow().iter() {
        wtr.write_record(&[
            format!("{:?}", child.et),
            format!("{}", child.name),
            format!("{:?}", child.result),
        ])?;
    }

    wtr.flush()?;
    Ok(String::from_utf8(wtr.into_inner()?)?)
}

fn dump_flat(element: &Element, results: &mut ResultList) {
    debug!("Flat Dump:");
    //println!("{:?}; {}", element.et, element.name);
    results.list.borrow_mut().push(ElementFlat {
        et: element.et.clone(),
        name: element.name.clone(),
        result: element.result.clone(),
    });
    dump_flat_inner(element, results);
}

fn dump_flat_inner(element: &Element, results: &mut ResultList) {
    for child in element.children.borrow().iter() {
        debug!("{:?}; {}; {:?}", child.et, child.name, child.result);
        results.list.borrow_mut().push(ElementFlat {
            et: child.et.clone(),
            name: child.name.clone(),
            result: child.result.clone(),
        });
        dump_flat_inner(child, results);
    }
}
