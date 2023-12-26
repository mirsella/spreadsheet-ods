//!
//! Read-only access.
//!
use std::{fs::File, io::BufReader};

use spreadsheet_ods::{OdsOptions, OdsResult};

///
pub fn main() -> OdsResult<()> {
    read_only()?;

    Ok(())
}

fn read_only() -> OdsResult<()> {
    let f = BufReader::new(File::open("examples/data.ods").expect("sample"));

    let wb = OdsOptions::default().
        // don't read styles
        content_only()
        // don't create empty cells
        .ignore_empty_cells()
        .read_ods(f)?;

    let sh = wb.sheet(0);

    for r in 0..27 {
        for c in 0..4 {
            // read value
            match sh.value(r, c) {
                spreadsheet_ods::Value::Empty => {}
                spreadsheet_ods::Value::Boolean(v) => println!("({},{}) = bool {}", r, c, v),
                spreadsheet_ods::Value::Number(v) => println!("({},{}) = number {}", r, c, v),
                spreadsheet_ods::Value::Percentage(v) => println!("({},{}) = percent {}", r, c, v),
                spreadsheet_ods::Value::Currency(v, cur) => {
                    println!("({},{}) = currency {} {}", r, c, v, cur)
                }
                spreadsheet_ods::Value::Text(v) => println!("({},{}) = text {}", r, c, v),
                spreadsheet_ods::Value::TextXml(v) => println!("({},{}) = xml {:?}", r, c, v),
                spreadsheet_ods::Value::DateTime(v) => println!("({},{}) = date {}", r, c, v),
                spreadsheet_ods::Value::TimeDuration(v) => {
                    println!("({},{}) = duration {}", r, c, v)
                }
            }
        }
    }

    // get converted value
    let a1 = sh.value(0, 0).as_str_or("");
    println!("A1 {}", a1);

    let a2 = sh.value(1, 0).as_f64_opt();
    println!("A2 {:?}", a2);

    Ok(())
}
