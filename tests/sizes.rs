use std::collections::HashMap;
use std::mem::size_of;

use chrono::{Duration, NaiveDateTime};

use spreadsheet_ods::metadata::Metadata;
use spreadsheet_ods::style::TableStyle;
use spreadsheet_ods::text::TextTag;
use spreadsheet_ods::{Sheet, Value, WorkBook};

#[test]
pub fn sizes() {
    println!("WorkBook {}", size_of::<WorkBook>());
    println!("Sheet {}", size_of::<Sheet>());
    println!("Metadata {}", size_of::<Metadata>());

    println!("(ucell,ucell) {}", size_of::<(u32, u32)>());

    println!("Value {}", size_of::<Value>());

    println!("bool {}", size_of::<bool>());
    println!("f64 {}", size_of::<f64>());
    println!("f64, Box<str> {}", size_of::<(f64, Box<str>)>());
    println!("String {}", size_of::<String>());
    println!("Vec<TextTag> {}", size_of::<Vec<TextTag>>());
    println!(
        "HashMap<String, TableStyle> {}",
        size_of::<HashMap<String, TableStyle>>()
    );
    println!("NaiveDateTime {}", size_of::<NaiveDateTime>());
    println!("Duration {}", size_of::<Duration>());
}
