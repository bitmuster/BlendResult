use csv::Writer;
use std::cell::RefCell;
use std::fs;
use std::rc::Rc;

use anyhow::anyhow;

use crate::element::{ElementFlat, ResultType};

/// Multiple results merged together as matrix of flat elements.
/// When the keyword is not executed it is None.
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
    #[allow(dead_code)]
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

#[cfg(feature = "odson")]
use icu_locid::locale;

#[cfg(feature = "odson")]
use spreadsheet_ods::color::Rgb;
#[cfg(feature = "odson")]
use spreadsheet_ods::defaultstyles::DefaultFormat;
#[cfg(feature = "odson")]
use spreadsheet_ods::style::CellStyle;
#[cfg(feature = "odson")]
use spreadsheet_ods::{pt, Sheet, WorkBook};

impl MultiResultList {
    /// Experimental ods export
    /// there are many todos hidden here
    #[cfg(feature = "odson")]
    pub fn export_to_ods(&self) {
        fs::create_dir_all("test_out").expect("create_dir");

        // let path = std::path::Path::new("test_out/lib_example.ods");
        // let mut wb = if path.exists() {
        // spreadsheet_ods::read_ods(path).unwrap()
        // } else {
        let mut wb = WorkBook::new(locale!("en_US"));
        // };

        //
        let mut no_style = CellStyle::new("no", &DefaultFormat::default());
        no_style.set_font_size(pt!(8));
        // 90ee900 // lightgreen
        let mut pass_style = CellStyle::new("pass", &DefaultFormat::default());
        pass_style.set_background_color(Rgb::new(0x90, 0xee, 0x90));
        pass_style.set_font_size(pt!(8));
        // ffbcc8 // lightpink
        let mut fail_style = CellStyle::new("fail", &DefaultFormat::default());
        fail_style.set_background_color(Rgb::new(0xff, 0xbc, 0xb8));
        fail_style.set_font_size(pt!(8));
        // add8e6 // lightblue
        let mut skip_style = CellStyle::new("skip", &DefaultFormat::default());
        skip_style.set_background_color(Rgb::new(0xad, 0xd8, 0xe6));
        skip_style.set_font_size(pt!(8));
        // d3d3d3 // lightgray
        let mut notrun_style = CellStyle::new("notrun", &DefaultFormat::default());
        notrun_style.set_background_color(Rgb::new(0xd3, 0xd3, 0xd3));
        notrun_style.set_font_size(pt!(8));

        // if wb.num_sheets() == 0 {
        let ref_no = wb.add_cellstyle(no_style);
        let ref_pass = wb.add_cellstyle(pass_style);
        let ref_fail = wb.add_cellstyle(fail_style);
        let ref_skip = wb.add_cellstyle(skip_style);
        let ref_notrun = wb.add_cellstyle(notrun_style);
        let mut sheet = Sheet::new("Results");
        let width = 4; // Amount of entries for each test analyzed testfile
        for result in 0..self.width {
            sheet.set_value(0, result as u32 * width + 0, format!("Type {result}"));
            sheet.set_value(0, result as u32 * width + 1, format!("Name {result}"));
            sheet.set_value(0, result as u32 * width + 2, format!("Result {result}"));
            sheet.set_value(0, result as u32 * width + 3, format!("Depth {width}"));
        }
        let mut child_num = 0;
        for child in self.list.borrow().iter() {
            for result in 0..self.width {
                match child[result].as_ref() {
                    Some(r) => {
                        sheet.set_value(
                            child_num,
                            result as u32 * width + 0,
                            format!("{:?}", r.et),
                        );
                        sheet.set_value(child_num, result as u32 * width + 1, r.name.to_string());
                        sheet.set_value(
                            child_num,
                            result as u32 * width + 2,
                            format!("{:?}", r.result),
                        );
                        sheet.set_value(child_num, result as u32 * width + 3, r.depth.to_string());
                        let style = match r.result {
                            ResultType::Pass => &ref_pass,
                            ResultType::Fail => &ref_fail,
                            ResultType::NotRun => &ref_notrun,
                            ResultType::Skip => &ref_skip,
                            _ => &ref_no,
                        };
                        sheet.set_cellstyle(child_num, result as u32 * width + 0, style);
                        sheet.set_cellstyle(child_num, result as u32 * width + 1, style);
                        sheet.set_cellstyle(child_num, result as u32 * width + 2, style);
                        sheet.set_cellstyle(child_num, result as u32 * width + 3, style);
                    }
                    None => {
                        sheet.set_value(child_num, result as u32 * width + 0, "-");
                        sheet.set_value(child_num, result as u32 * width + 1, "-");
                        sheet.set_value(child_num, result as u32 * width + 2, "-");
                        sheet.set_value(child_num, result as u32 * width + 3, "-");
                    }
                }
            }
            child_num += 1;
        }
        wb.push_sheet(sheet);

        spreadsheet_ods::write_ods(&mut wb, "export.ods").expect("write_ods");
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
            depth: 42,
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
                depth: 10,
            }),
            None,
        ])?;
        mrl.push(vec![
            Some(ElementFlat {
                et: ElementType::Suite,
                result: ResultType::Pass,
                name: "a suite".to_string(),
                depth: 10,
            }),
            Some(ElementFlat {
                et: ElementType::Keyword,
                result: ResultType::Fail,
                name: "another suite".to_string(),
                depth: 10,
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
