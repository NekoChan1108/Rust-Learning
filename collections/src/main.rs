#![allow(unused)]
fn main() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for cell in row {
        match cell {
            SpreadsheetCell::Int(i) => println!("{}", i),
            SpreadsheetCell::Float(f) => println!("{}", f),
            SpreadsheetCell::Text(s) => println!("{}", s),
        }
    }
}
