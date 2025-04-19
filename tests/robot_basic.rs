use std::fs;

#[test]
fn test_stuff() {
    let filename = "output.xml";
    let xml = fs::read_to_string(filename).unwrap();
    blend_result::parse(&xml);
}
