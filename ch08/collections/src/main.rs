fn main() {
    // Because we’ve given initial i32 values, Rust can infer that the type of v is Vec<i32>, and the type annotation isn’t necessary
    let v = vec![1, 2, 3];
    println!("v {:#?}", v);

    // mut vector

    let mut mv = vec!["test1", "test2", "test2"];

    mv.push("test5");

    println!("mv {:#?}", mv);

    // get element out
    let e = mv[0];
    println!("index {:#?}", e);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // To change the value that the mutable reference refers to, we have to use the dereference
        // operator (*) to get to the value in i before we can use the += operator
        *i += 50;
    }

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
}
