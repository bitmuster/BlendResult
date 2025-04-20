use std::cell::RefCell;
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

#[derive(Debug)]
pub struct Element {
    pub et: ElementType,
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
