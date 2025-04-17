// combined from https://github.com/tafia/quick-xml

use quick_xml::events::attributes::AttrError;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use quick_xml::events::attributes;
use std::any;
use std::env;
use std::fs;
use std::str;

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

fn print_attributes(ident:&str, attr: attributes::Attributes ) {
    for a in attr {
        let key = str::from_utf8(a.clone().unwrap().key.local_name().into_inner()).unwrap();
        let value = a.unwrap().unescape_value().unwrap();
        println!( "{ident}    Attr: {:?} {:?}",key, value);
    }
}

fn main() -> Result<(), AppError> {
    let filename = env::args().skip(1).next().unwrap();
    println!("Analyzing {}", filename);

    let xml = fs::read_to_string(filename).unwrap();

    let mut reader = Reader::from_str(&xml);
    reader.config_mut().trim_text(true);

    let mut txt = Vec::new();
    let mut buf = Vec::new();
    let mut depth = 0;
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
                depth += 1;
            }
            Ok(Event::Text(e)) => {
                //println!("{ident}Text {}", any::type_name_of_val(&e));
                println!("{ident}Text: {}", e.unescape().unwrap());
                txt.push(e.unescape().unwrap().into_owned())
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

    Ok(())
}
