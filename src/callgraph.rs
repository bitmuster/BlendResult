use crate::element::Element;
/// Example copied from : https://docs.rs/graphviz-rust/0.9.6/graphviz_rust/#examples
use graphviz_rust;
use graphviz_rust::dot_generator::*;
use graphviz_rust::dot_structures::*;
use graphviz_rust::{
    attributes::*,
    cmd::{CommandArg, Format},
    exec, exec_dot, parse,
    printer::{DotPrinter, PrinterContext},
};
use std::fs;
use std::io::prelude::*;

fn demo() {
    let g = graph!(strict di id!("t");
          node!("aa";attr!("color","green")),
          subgraph!("v";
            node!("aa"; attr!("shape","square")),
            subgraph!("vv"; edge!(node_id!("a2") => node_id!("b2"))),
            node!("aaa";attr!("color","red")),
            edge!(node_id!("aaa") => node_id!("bbb"))
            ),
          edge!(node_id!("aa") => node_id!("be") => subgraph!("v"; edge!(node_id!("d") => node_id!("aaa")))),
          edge!(node_id!("aa") => node_id!("aaa") => node_id!("v"))
    );

    let dot = g.print(&mut PrinterContext::default());
    println!("{}", dot);
    let format = Format::Svg;

    let graph_svg = exec_dot(dot, vec![format.clone().into()]).unwrap();
    let _ = fs::File::create("graph.svg").unwrap().write(&graph_svg);
}

pub fn dot_callgraph(xml_data: &str) -> anyhow::Result<()> {
    demo();
    let mut reader = fs::Reader::from_str(xml_data);
    reader.config_mut().trim_text(true);

    let depth = 0;
    let mut root_element: Element = Element::new();
    let mut stats = ParserStats { max_depth: 0 };

    parse_inner(&mut reader, &mut root_element, depth, &mut stats)?;
    Ok(())
}
