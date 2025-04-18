use quick_xml::events::attributes;
use quick_xml::events::attributes::AttrError;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::any;
use std::str;

use crate::element::{Element, ElementType, ResultType};

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

pub fn parse(xml_file: &str) {
    let mut reader = Reader::from_str(&xml_file);
    reader.config_mut().trim_text(true);

    let mut buf = Vec::new();
    let mut depth = 0;
    let mut root_element: Option<Element> = None;
    let mut current: Option<&mut Element> = None;
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

                match e.name().as_ref() {
                    b"robot" => (),
                    b"suite" => {
                        root_element = Some(Element {
                            et: ElementType::Suite,
                            children: Vec::new(),
                            result: ResultType::None,
                        });
                        current = Some(root_element.as_mut().unwrap());
                    }
                    b"test" => {
                        let mut new_element = Element {
                            et: ElementType::Test,
                            children: Vec::new(),
                            result: ResultType::None,
                        };
                        current.as_mut().unwrap().children.push(new_element);
                        //current = Some(&current.unwrap());
                    }
                    b"kw" => (),
                    b"doc" => (),
                    b"arg" => (),
                    b"statistics" => (),
                    b"total" => (),
                    b"errors" => (),
                    b"stat" => (),
                    b"tag" => (),
                    s => println!("Unmatched {:?}", str::from_utf8(s).unwrap()),
                }
                depth += 1;
            }
            Ok(Event::Text(e)) => {
                //println!("{ident}Text {}", any::type_name_of_val(&e));
                println!("{ident}Text: {}", e.unescape().unwrap());
            }
            Ok(Event::End(e)) => {
                //println!("  End {}", any::type_name_of_val(&e));
                depth -= 1;
                let ident = " ".repeat(depth * 4 + 4);
                println!(
                    "{ident}End: {}",
                    str::from_utf8(e.local_name().as_ref()).unwrap()
                );
            }
            Ok(Event::Empty(e)) => {
                //println!("{ident}Empty {}", any::type_name_of_val(&e));
                println!(
                    "{ident}Empty: {}",
                    str::from_utf8(e.local_name().as_ref()).unwrap()
                );
                print_attributes(&ident, e.attributes());
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

    println!("{:?}", root_element.unwrap());
}
