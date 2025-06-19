use std::any;
use std::cell::Ref;
use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;
use std::slice::Iter;
use std::str;

// use anyhow::Context;
use colored::Colorize;
use csv::Writer;

use log::{debug, info, trace, warn};
use quick_xml::encoding::Decoder;
use quick_xml::events::attributes;
use quick_xml::events::attributes::{AttrError, Attribute};
use quick_xml::events::Event;
use quick_xml::reader::Reader;

use crate::element::{Element, ElementFlat, ElementType, ResultList, ResultType};
use crate::multi_result_list::MultiResultList;

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

/// Convert Attribute to key and value
/// TODO: Can we change the return type to e.g. back to Cow and &str?
fn get_attribute_kv(decoder: Decoder, a: Result<Attribute, AttrError>) -> (String, String) {
    let key = str::from_utf8(a.clone().unwrap().key.local_name().into_inner()).unwrap();
    let value;
    #[cfg(feature = "odson")]
    {
        value = a
            .unwrap()
            .decode_and_unescape_value(decoder)
            .expect("decode_and_unescape_value");
    }
    #[cfg(not(feature = "odson"))]
    {
        value = a.unwrap().unescape_value().unwrap();
    }
    (key.to_string(), value.to_string())
}

/// Print all XML attributes
fn print_attributes(decoder: Decoder, ident: &str, attr: attributes::Attributes) {
    for a in attr {
        let (key, value) = get_attribute_kv(decoder, a);
        debug!("{ident}    Attr: {:?} {:?}", key, value);
    }
}

/// Return the value of an XML attribute by the attribute name.
/// Otherwise return an empty string.
fn get_attr_name<'a>(decoder: Decoder, name: &'a str, attr: attributes::Attributes<'a>) -> String {
    for a in attr {
        let (key, value) = get_attribute_kv(decoder, a);
        if name == key {
            return value.to_string();
        }
    }
    "".to_string()
    //panic!("Cannot get attr {name}");
}

/// Convert a string status to a ResultType
/// TODO This could belong to ResultType
fn status_to_result(status: &str) -> ResultType {
    match status {
        "PASS" => ResultType::Pass,
        "FAIL" => ResultType::Fail,
        "NOT RUN" => ResultType::NotRun,
        "SKIP" => ResultType::Skip,
        s => {
            info!("Panic:  \"{s}\"");
            panic!("Panic:  \"{s}\"");
            // ResultType::None
        }
    }
}

pub struct ParserStats {
    pub max_depth: usize,
}

/// Slightly cursed recursive parser for output.xml files
pub fn parse_inner(
    reader: &mut Reader<&[u8]>,
    element: &mut Element,
    depth: usize,
    stats: &mut ParserStats,
) -> anyhow::Result<()> {
    let mut buf = Vec::new();
    let decoder = reader.decoder();
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
                print_attributes(decoder, &ident, e.attributes());
                let name = get_attr_name(decoder, "name", e.attributes());
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
                    let status = get_attr_name(decoder, "status", e.attributes());
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

                print_attributes(decoder, &ident, e.attributes());
                match element.et {
                    ElementType::Keyword | ElementType::Suite | ElementType::Test => {
                        let status = get_attr_name(decoder, "status", e.attributes());
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
///
/// TODO reduce complexity
pub fn diff_tree(
    elements: &[Option<&Element>],
    mrl: &MultiResultList,
    depth: usize,
    max_depth: usize,
) -> anyhow::Result<()> {
    if depth > max_depth && max_depth != 0 {
        return Ok(());
    }

    let len = elements.len();

    // temporary values to store our borrowed children Vec
    let mut borrowed_children: Vec<Option<Ref<'_, Vec<Rc<Element>>>>> = Vec::new();

    for element in elements.iter() {
        let children = match element {
            Some(s) => Some(s.children.borrow()),
            None => None,
        };
        borrowed_children.push(children);
    }

    let mut iterators: Vec<Option<Iter<Rc<Element>>>> = Vec::new();

    for child in borrowed_children.iter() {
        let iterator = match child {
            Some(s) => Some(s.iter()),
            None => None,
        };
        iterators.push(iterator);
    }

    loop {
        let mut elf: Vec<Option<ElementFlat>> = Vec::new();

        let mut count = 0;
        let mut velem: Vec<Option<&Element>> = Vec::new();
        let mut state: String = String::new();
        for iterator in iterators.iter_mut() {
            let next: Option<&Rc<Element>> = match iterator {
                Some(ref mut s) => s.next(),
                None => None,
            };

            match next {
                Some(s) => {
                    trace!(
                        "name: {}-{} {:?} {:?} {:?}",
                        count,
                        depth,
                        s.name,
                        s.et,
                        s.result
                    );
                    elf.push(Some(ElementFlat {
                        et: s.et.clone(),
                        result: s.result.clone(),
                        name: s.name.clone(),
                        depth,
                    }));
                    state.push_str(&format!(
                        "{:<16} {:<16?} {:<16} ",
                        s.name.blue(),
                        s.et,
                        s.result.to_string().yellow()
                    ));
                    velem.push(Some(s));
                }
                None => {
                    trace!("name: {}-{} None", count, depth);
                    elf.push(None);
                    state.push_str(&format!("{:<16} {:<16?} {:<16}", "-", "-", "-"));
                    velem.push(None);
                }
            }
            count += 1;
        }

        if velem.iter().filter(|s| s.is_none()).count() == len {
            break;
        }

        {
            let mut mrlb = mrl.list.borrow_mut();
            mrlb.push(elf);
        };

        println!("{}", state);
        diff_tree(&velem, &mrl, depth + 1, max_depth)?;
    }
    Ok(())
}

/// Parse a XML str and dump it into a CSV file
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
    let mut stats = ParserStats { max_depth: 3 };

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
    dump_csv_file(csv_file, &results)?;
    println!("Parsed {} elements", results.list.borrow().len());
    println!("Maximum tree depth {}", stats.max_depth);
    Ok(results)
}

/// Parse a XML str and dump it into a CSV str
/// TODO combine with parse above
#[allow(dead_code)]
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

    dump_csv_to_str(&results)
}

/// Dump a ResultList into a single CSV file
pub fn dump_csv_file(csv_file: &str, results: &ResultList) -> anyhow::Result<()> {
    //let mut wtr = csv::Writer::from_writer(io::stdout());
    let mut wtr = csv::Writer::from_path(csv_file)?;

    wtr.write_record(["Type", "Name", "Result"])?;
    for child in results.list.borrow().iter() {
        wtr.write_record(&[
            format!("{:?}", child.et),
            child.name.to_string(),
            format!("{:?}", child.result),
        ])?;
    }

    wtr.flush()?;
    Ok(())
}

/// Dump a ResultList into a single CSV String
/// TODO combine with above
pub fn dump_csv_to_str(results: &ResultList) -> anyhow::Result<String> {
    let mut wtr = Writer::from_writer(vec![]);
    wtr.write_record(["Type", "Name", "Result"])?;
    for child in results.list.borrow().iter() {
        wtr.write_record(&[
            format!("{:?}", child.et),
            child.name.to_string(),
            format!("{:?}", child.result),
        ])?;
    }

    wtr.flush()?;
    Ok(String::from_utf8(wtr.into_inner()?)?)
}

/// Dump an Element tree into a flat ResultList
pub fn dump_flat(element: &Element, results: &mut ResultList) {
    debug!("Flat Dump:");
    //println!("{:?}; {}", element.et, element.name);
    results.list.borrow_mut().push(ElementFlat {
        et: element.et.clone(),
        name: element.name.clone(),
        result: element.result.clone(),
        depth: 0,
    });
    dump_flat_inner(element, results, 1);
}

/// Internas of dumping an Element tree into a flat ResultList
fn dump_flat_inner(element: &Element, results: &mut ResultList, depth: usize) {
    for child in element.children.borrow().iter() {
        debug!("{:?}; {}; {:?}", child.et, child.name, child.result);
        results.list.borrow_mut().push(ElementFlat {
            et: child.et.clone(),
            name: child.name.clone(),
            result: child.result.clone(),
            depth,
        });
        dump_flat_inner(child, results, depth + 1);
    }
}
