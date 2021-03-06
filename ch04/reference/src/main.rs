fn main() {
    let str_test = String::from("test");
    println!("Hello, world! {}", count_str(str_test));
    // println!("str is {}", str); -- this will trigger error error: Could not compile `reference

    let str2 = String::from("test2");
    let size = count_str_ref(&str2);
    println!("{} size is {}", str2, size);

    let str3 = String::from("test3");
    let str3_result = double_str(str3);
    println!("doulbe str is {}", str3_result);

    let str4 = dangle();
}

fn count_str(str: String) -> usize {
    return str.len();
}

fn count_str_ref(str: &String) -> usize {
    return str.len();
}

fn double_str(mut str_test: String) -> String {
    str_test.push_str(&str_test.to_string());
    str_test
}


fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!