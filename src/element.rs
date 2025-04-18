
#[derive(Debug, PartialEq)]
pub enum ElementType {
    Suite,
    Test,
    Keyword,
}

#[derive(Debug, PartialEq)]
pub enum ResultType {
    Pass,
    Fail,
    None,
}

#[derive(Debug, PartialEq)]
pub struct Element {
    pub et: ElementType,
    pub children: Vec<Element>,
    pub result: ResultType,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_elements(){
        let _elem = Element {
            et: ElementType::Suite,
            children: Vec::new(),
            result: ResultType::None,
        };
        //assert
    }
    
    #[test]
    fn test_new_tree() {
        let mut suite = Element {
            et: ElementType::Suite,
            children: Vec::new(),
            result: ResultType::None,
        };
        let new_test = Element {
            et: ElementType::Test,
            children: Vec::new(),
            result: ResultType::Pass,
        };
        suite.children.push(new_test);
        let new_test = Element {
            et: ElementType::Test,
            children: Vec::new(),
            result: ResultType::Fail,
        };
        suite.children.push(new_test);
        let new_kw = Element {
            et: ElementType::Keyword,
            children: Vec::new(),
            result: ResultType::None,
        };
        //let mut et: &mut Element = suite.children.get_mut(1).unwrap();
        let et: &mut Element = suite.children.get_mut(1).unwrap();
        et.children.push(new_kw);
        assert_eq!( suite.children.get(0).unwrap().et, ElementType::Test);
        assert_eq!( suite.children.get(0).unwrap().result, ResultType::Pass);
        assert_eq!( suite.children.get(1).unwrap().et, ElementType::Test);
        assert_eq!( suite.children.get(1).unwrap().result, ResultType::Fail);
        assert_eq!( suite.children.get(1).unwrap().children.get(0).unwrap().et, ElementType::Keyword);
        assert_eq!( suite.children.get(1).unwrap().children.get(0).unwrap().result, ResultType::None);
    }
}
