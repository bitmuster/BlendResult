// Use Bencer once stable
// https://doc.rust-lang.org/unstable-book/library-features/test.html
// use test::Bencher;

use std::thread::sleep;
use std::time::{Duration, Instant};

#[cfg(feature = "odson")]
use icu_locid::locale;

#[cfg(feature = "odson")]
use spreadsheet_ods::color::Rgb;
#[cfg(feature = "odson")]
use spreadsheet_ods::defaultstyles::DefaultFormat;
#[cfg(feature = "odson")]
use spreadsheet_ods::style::CellStyle;
#[cfg(feature = "odson")]
use spreadsheet_ods::{Sheet, WorkBook};

pub fn ods_test(loops: u32) {
    let mut wb = WorkBook::new(locale!("en_US"));

    let mut sheet = Sheet::new("one");

    for i in 1..loops {
        for j in 1..100 {
            sheet.set_value(i, j, 42);
        }
    }
    wb.push_sheet(sheet);

    spreadsheet_ods::write_ods(&mut wb, "bench.ods").expect("bench.ods");
}

// cargo test ods_export_time -- --show-output
// Measures around 960µs per element
#[cfg(feature = "odson")]
#[test]
fn ods_export_time() {
    for loops in 1..=5 {
        let instant = Instant::now();
        ods_test(loops * 10);
        let elapsed = instant.elapsed();
        println!(
            "Elements {} : {:?} : ms per element {:?}",
            loops * 10 * 100,
            elapsed,
            elapsed / (loops * 1000)
        );
    }
}
