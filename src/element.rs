pub enum ElementType {
    Suite,
    Test,
    Keyword,
}

pub enum ResultType {
    Pass,
    Fail,
    None,
}

pub struct Element {
    pub et: ElementType,
    pub children: Vec<Element>,
    pub result: ResultType,
}

#[cfg(test)]
mod test {
    use super::*;

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
        let mut et: &mut Element = suite.children.get_mut(1).unwrap();
        et.children.push(new_kw);
    }
}
