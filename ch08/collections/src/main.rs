fn main() {
    // Because we’ve given initial i32 values, Rust can infer that the type of v is Vec<i32>, and the type annotation isn’t necessary
    let v = vec![1, 2, 3];
    println!("v {:#?}", v);

    // mut vector

    let mut mv = vec!["test1", "test2", "test2"];

    mv.push("test5");

    println!("mv {:#?}", mv);
}
