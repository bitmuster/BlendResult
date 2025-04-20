use quick_xml::events::attributes;
use quick_xml::events::attributes::AttrError;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::any;
use std::cell::RefCell;
use std::io;
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
        println!("{ident}    Attr: {:?} {:?}", key, value);
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
}

fn status_to_result(status: &str) -> ResultType {
    match status {
        "PASS" => ResultType::Pass,
        "FAIL" => ResultType::Fail,
        s => {
            //panic!("Panic:  {s}");
            println!("Panic:  {s}");
            ResultType::None
        }
    }
}

/// Slightly cursed parser for output.xml files
fn parse_inner(
    reader: &mut Reader<&[u8]>,
    element: &mut Element,
    depth: usize,
) -> anyhow::Result<()> {
    let mut buf = Vec::new();

    loop {
        let ident = " ".repeat(depth * 4 + 4);
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),

            Ok(Event::Eof) => break,

            Ok(Event::Start(e)) => {
                //println!("  Start {}", any::type_name_of_val(&e));
                println!(
                    "{ident}Start: {}",
                    str::from_utf8(e.local_name().as_ref()).unwrap()
                );
                print_attributes(&ident, e.attributes());
                let name = get_attr_name("name", e.attributes());
                let mut et : Option<ElementType> = None;
                match e.name().as_ref() {
                    b"robot" => (),
                    b"suite" => {
                        et = Some(ElementType::Suite)
                    },
                    b"test" => {
                        et = Some(ElementType::Test)
                    },
                    b"kw" => {
                        et = Some(ElementType::Keyword)
                    },
                    b"if" => {
                        et = Some(ElementType::If)
                    },
                    b"branch" => {
                        et = Some(ElementType::Branch)
                    },
                    b"try" => {
                        et = Some(ElementType::Try)
                    },
                    b"for" => {
                        et = Some(ElementType::For)
                    },
                    b"iter" => {
                        et = Some(ElementType::Iter)
                    },
                    b"while" => {
                        et = Some(ElementType::While)
                    },
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
                    s => {
                        println!("Unmatched {:?}", str::from_utf8(s).unwrap());
                        panic!()
                        }
                    }

                if let Some(e) = et {
                    let mut suite_element = Element {
                            et: e,
                            children: RefCell::new(Vec::new()),
                            parent: RefCell::new(Weak::new()),
                            result: ResultType::None,
                            name,
                        };
                    parse_inner(reader, &mut suite_element, depth + 1)?;
                    let mut parent = element.parent.borrow_mut();
                    let rc_suite_element = Rc::new(suite_element);
                    *parent = Rc::downgrade(&rc_suite_element);
                    element.children.borrow_mut().push(rc_suite_element);
                }
            }
            Ok(Event::Text(e)) => {
                //println!("{ident}Text {}", any::type_name_of_val(&e));
                let text : &str = &e.unescape().unwrap();
                let len = usize::min(text.len(),20);
                println!("{ident}    Text: {}", text.get(0..len).unwrap());
            }
            Ok(Event::End(e)) => {
                //println!("  End {}", any::type_name_of_val(&e));
                let ident = " ".repeat(depth * 4 + 4);
                println!(
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
                    _ => (),
                }
            }
            Ok(Event::Empty(e)) => {
                //println!("{ident}Empty {}", any::type_name_of_val(&e));
                println!(
                    "{ident}Empty: {}",
                    str::from_utf8(e.local_name().as_ref()).unwrap()
                );
                print_attributes(&ident, e.attributes());
                match element.et {
                    ElementType::Keyword | ElementType::Suite | ElementType::Test => {
                        let status = get_attr_name("status", e.attributes());
                        element.result = status_to_result(&status);
                    }
                    _ => (),
                }
            }
            Ok(Event::Decl(e)) => {
                println!("{ident}Decl {}", any::type_name_of_val(&e));
            }

            /*            Ok(x) => {
                println!("Ok {:?}", any::type_name_of_val(&x));
            }*/
            x => {
                println!("No Type {:?}", any::type_name_of_val(&x));
            }
        }
        buf.clear();
    }
    Ok(())
}

pub fn parse(xml_file: &str, csv_file: &str) -> anyhow::Result<ResultList> {
    let mut reader = Reader::from_str(xml_file);
    reader.config_mut().trim_text(true);

    let depth = 0;
    let mut root_element: Element = Element {
        et: ElementType::Robot,
        children: RefCell::new(Vec::new()),
        parent: RefCell::new(Weak::new()),
        result: ResultType::None,
        name: String::new(),
    };

    parse_inner(&mut reader, &mut root_element, depth)?;

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
    Ok(results)
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

fn dump_flat(element: &Element, results: &mut ResultList) {
    println!("Flat Dump:");
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
        println!("{:?}; {}; {:?}", child.et, child.name, child.result);
        results.list.borrow_mut().push(ElementFlat {
            et: child.et.clone(),
            name: child.name.clone(),
            result: child.result.clone(),
        });
        dump_flat_inner(child, results);
    }
}
