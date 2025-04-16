// combined from https://github.com/tafia/quick-xml

use quick_xml::events::attributes::AttrError;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use std::any;
use std::env;
use std::fs;
use std::io;
use std::io::Read;
use std::str;

#[allow(dead_code)]
#[derive(Debug)]
enum AppError {
    /// XML parsing error
    Xml(quick_xml::Error),
    /// The `Translation/Text` node is missed
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
        
        match reader.read_event_into(&mut buf) {
            Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),

            Ok(Event::Eof) => break,

            Ok(Event::Start(e)) => {
                //println!("  Start {}", any::type_name_of_val(&e));
                println!("  Start: {}", str::from_utf8( e.local_name().as_ref()).unwrap());
                /*match e.name().as_ref() {
                    b"tag1" => {
                        println!("Tag1 {e:?}");
                        /*println!(
                            "attributes values: {:?}",
                            e.attributes().map(|a| a.unwrap().value).collect::<Vec<_>>()
                        );*/
                        let wtf = e.attributes().next().unwrap().unwrap();
                        let s = wtf.decode_and_unescape_value(reader.decoder()).unwrap();
                        println!("Type {}", any::type_name_of_val(&s));
                        //println!("attributes values: {:?}", s)
                    }
                    b"tag2" => count += 1,
                    _ => (),
                }*/
                depth +=1;
            }
            Ok(Event::Text(e)) => {
                println!("  Text {}", any::type_name_of_val(&e));
                txt.push(e.unescape().unwrap().into_owned())
            }
            Ok(Event::End(e)) => {
                //println!("  End {}", any::type_name_of_val(&e));
                println!("  End: {}", str::from_utf8( e.local_name().as_ref()).unwrap());
                depth -= 1;
            }
            Ok(Event::Empty(e)) => {
                println!("  Empty {}", any::type_name_of_val(&e));
            }
            Ok(Event::Decl(e)) => {
                println!("  Decl {}", any::type_name_of_val(&e));
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
