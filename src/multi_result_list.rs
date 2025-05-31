use csv::Writer;
use std::cell::RefCell;
use std::rc::Rc;

use anyhow::anyhow;

use crate::element::ElementFlat;

#[derive(Debug)]
pub struct MultiResultList {
    pub list: Rc<RefCell<Vec<Vec<Option<ElementFlat>>>>>,
    pub width: usize,
}

impl MultiResultList {
    pub fn new(width: usize) -> Self {
        MultiResultList {
            list: Rc::new(RefCell::new(Vec::new())),
            width,
        }
    }
    pub fn push(&self, value: Vec<Option<ElementFlat>>) -> anyhow::Result<()> {
        if value.len() == self.width {
            self.list.borrow_mut().push(value);
        } else {
            return Err(anyhow!(
                "Width {} does not match {}",
                value.len(),
                self.width
            ));
        }
        Ok(())
    }

    pub fn dump_to_csv_str(&self) -> anyhow::Result<String> {
        let mut wtr = Writer::from_writer(vec![]);
        let mut record: Vec<String> = Vec::new();
        for result in 0..self.width {
            record.push(format!("Type {result}"));
            record.push(format!("Name {result}"));
            record.push(format!("Result {result}"));
        }
        //println!("{record:?}");
        wtr.write_record(&record)?;

        for child in self.list.borrow().iter() {
            let mut record: Vec<String> = Vec::new();
            for result in 0..self.width {
                match child[result].as_ref() {
                    Some(r) => {
                        record.push(format!("{:?}", r.et));
                        record.push(r.name.to_string());
                        record.push(format!("{:?}", r.result));
                    }
                    None => {
                        record.push("-".to_string());
                        record.push("-".to_string());
                        record.push("-".to_string());
                    }
                }
            }
            // println!("{record:?}");
            wtr.write_record(&record)?;
        }

        wtr.flush()?;
        Ok(String::from_utf8(wtr.into_inner()?)?)
    }
}

#[cfg(test)]
mod test_multi_result_list {
    use super::*;
    use crate::element::{ElementFlat, ElementType, ResultType};

    #[test]
    fn create_empty() -> anyhow::Result<()> {
        let mrl = MultiResultList::new(0); // TODO unclear if 0 make sense
        let el: Vec<Option<ElementFlat>> = vec![];
        mrl.push(el)?;
        let result = mrl.list.borrow();
        println!("{:?}", mrl);
        assert_eq!(result[0], vec![]);
        Ok(())
    }
    #[test]
    fn create_none() -> anyhow::Result<()> {
        let mrl = MultiResultList::new(1);
        let el: Vec<Option<ElementFlat>> = vec![None];
        mrl.push(el)?;
        let result = mrl.list.borrow();
        assert_eq!(result[0], vec![None]);
        Ok(())
    }
    #[test]
    fn create_element() -> anyhow::Result<()> {
        let mrl = MultiResultList::new(1);
        println!("{:?}", mrl);

        mrl.push(vec![Some(ElementFlat {
            et: ElementType::Suite,
            result: ResultType::Pass,
            name: "a suite".to_string(),
        })])?;
        println!("{:?}", mrl);
        let result = mrl.list.borrow();

        // TODO switch to assert_matches when stable
        if let Some(_) = result[0][0] {
        } else {
            panic!("Pattern does not match")
        }
        Ok(())
    }
    #[test]
    fn create_elements() -> anyhow::Result<()> {
        let mrl = MultiResultList::new(2);
        println!("{:?}", mrl);

        mrl.push(vec![
            Some(ElementFlat {
                et: ElementType::Suite,
                result: ResultType::Pass,
                name: "a suite".to_string(),
            }),
            None,
        ])?;
        mrl.push(vec![
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
        ])?;
        println!("{:?}", mrl);
        let result = mrl.list.borrow();

        // TODO switch to assert_matches when stable
        if let Some(_) = result[0][0] {
        } else {
            panic!("Pattern does not match")
        }
        if let None = result[0][1] {
        } else {
            panic!("Pattern does not match")
        }
        if let [Some(_), Some(_)] = result[1][..] {
        } else {
            panic!("Pattern 2 does not match")
        }

        let mlrs = mrl.dump_to_csv_str();
        println!("{}", mlrs.unwrap());
        Ok(())
    }
}
