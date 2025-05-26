use std::cell::RefCell;
use std::fmt;
use std::rc::{Rc, Weak};

#[derive(Debug, PartialEq, Clone)]
pub enum ElementType {
    Robot,
    Suite,
    Test,
    Keyword,
    If,
    Branch,
    Try,
    For,
    Iter,
    While,
    Continue,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ResultType {
    Pass,
    Fail,
    NotRun,
    None,
}

impl fmt::Display for ResultType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct Element {
    pub et: ElementType,
    // TODO Change this into a better pattern
    // pub children: Rc<RefCell<Vec<Element>>>,
    pub children: RefCell<Vec<Rc<Element>>>,
    pub parent: RefCell<Weak<Element>>,
    pub result: ResultType,
    pub name: String,
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        self.et == other.et && self.children == other.children && self.result == other.result
    }
}

#[derive(Debug, PartialEq)]
pub struct ElementFlat {
    pub et: ElementType,
    pub result: ResultType,
    pub name: String,
}

#[derive(Debug)]
pub struct ResultList {
    pub list: Rc<RefCell<Vec<ElementFlat>>>,
}

impl PartialEq for ResultList {
    fn eq(&self, other: &Self) -> bool {
        let mut equal = true;
        for (a, b) in self.list.borrow().iter().zip(other.list.borrow().iter()) {
            if a == b {
            } else {
                equal = false;
            }
        }
        equal
    }
}

#[derive(Debug)]
pub struct MultiResultList {
    pub list: Rc<RefCell<Vec<Vec<Option<ElementFlat>>>>>,
}

impl MultiResultList {
    fn new() -> Self {
        MultiResultList {
            list: Rc::new(RefCell::new(Vec::new())),
        }
    }
    fn push(&self, value: Vec<Option<ElementFlat>>) {
        self.list.borrow_mut().push(value)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_elements() {
        let _elem = Element {
            et: ElementType::Suite,
            children: RefCell::new(Vec::new()),
            parent: RefCell::new(Weak::new()),
            result: ResultType::None,
            name: String::new(),
        };
        //assert
    }

    #[test]
    fn test_new_tree() {
        let suite = Element {
            et: ElementType::Suite,
            children: RefCell::new(Vec::new()),
            parent: RefCell::new(Weak::new()),
            result: ResultType::None,
            name: String::new(),
        };
        let new_test = Element {
            et: ElementType::Test,
            children: RefCell::new(Vec::new()),
            parent: RefCell::new(Weak::new()),
            result: ResultType::Pass,
            name: String::new(),
        };
        suite.children.borrow_mut().push(Rc::new(new_test));
        let new_test2 = Element {
            et: ElementType::Test,
            children: RefCell::new(Vec::new()),
            parent: RefCell::new(Weak::new()),
            result: ResultType::Fail,
            name: String::new(),
        };
        suite.children.borrow_mut().push(Rc::new(new_test2));
        let new_kw = Element {
            et: ElementType::Keyword,
            children: RefCell::new(Vec::new()),
            parent: RefCell::new(Weak::new()),
            result: ResultType::None,
            name: String::new(),
        };
        {
            // Now we add the kw to the second test
            let binding = suite.children.borrow_mut();
            let ref_test2 = binding.get(1);
            ref_test2
                .unwrap()
                .children
                .borrow_mut()
                .push(Rc::new(new_kw));
        }

        assert_eq!(
            suite.children.borrow_mut().get(0).unwrap().et,
            ElementType::Test
        );
        assert_eq!(
            suite.children.borrow_mut().get(0).unwrap().result,
            ResultType::Pass
        );
        assert_eq!(
            suite.children.borrow_mut().get(1).unwrap().et,
            ElementType::Test
        );
        assert_eq!(
            suite.children.borrow_mut().get(1).unwrap().result,
            ResultType::Fail
        );
        assert_eq!(
            suite
                .children
                .borrow()
                .get(1)
                .unwrap()
                .children
                .borrow()
                .get(0)
                .unwrap()
                .et,
            ElementType::Keyword
        );
        assert_eq!(
            suite
                .children
                .borrow_mut()
                .get(1)
                .unwrap()
                .children
                .borrow_mut()
                .get(0)
                .unwrap()
                .result,
            ResultType::None
        );
    }
    #[test]
    fn test_parent() {
        let kw = Rc::new(Element {
            et: ElementType::Keyword,
            children: RefCell::new(Vec::new()),
            parent: RefCell::new(Weak::new()),
            result: ResultType::None,
            name: String::new(),
        });
        let test = Rc::new(Element {
            et: ElementType::Test,
            children: RefCell::new(Vec::new()),
            parent: RefCell::new(Weak::new()),
            result: ResultType::None,
            name: String::new(),
        });

        let mut parent = kw.parent.borrow_mut();
        *parent = Rc::downgrade(&test);
        test.children.borrow_mut().push(kw.clone());
    }
}

#[cfg(test)]
mod TestMultiResultList {
    use super::*;

    #[test]
    fn create_empty() {
        let mrl = MultiResultList::new();
        let el: Vec<Option<ElementFlat>> = vec![];
        mrl.push(el);
        let result = mrl.list.borrow();
        assert_eq!(result[0], vec![])
    }
    #[test]
    fn create_none() {
        let mrl = MultiResultList::new();
        let el: Vec<Option<ElementFlat>> = vec![None];
        mrl.push(el);
        let result = mrl.list.borrow();
        assert_eq!(result[0], vec![None])
    }
    #[test]
    fn create_element() {
        let mrl = MultiResultList::new();
        println!("{:?}", mrl);

        mrl.push(vec![Some(ElementFlat {
            et: ElementType::Suite,
            result: ResultType::Pass,
            name: "a suite".to_string(),
        })]);
        /*mrl.push(vec![
            Some(ElementFlat {
                et: ElementType::Suite,
                result: ResultType::Pass,
                name: "a suite".to_string(),
            }),
            Some(ElementFlat {
                et: ElementType::Keyword,
                result: ResultType::Fail,
                name: "another suite".to_string(),
            }),
        ]);*/
        println!("{:?}", mrl);
        let result = mrl.list.borrow();

        if let Some(_) = result[0][0] {
        } else {
            panic!("Ohn no")
        }
    }
}
