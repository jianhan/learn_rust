fn main() {
    let str = String::from("test");
    println!("Hello, world! {}", count_str(str));
    // println!("str is {}", str); -- this will trigger error error: Could not compile `reference

    let str2 = String::from("test2");
    let size = count_str_ref(&str2);
    println!("{} size is {}", str2, size);
    
}

fn count_str(str: String) -> usize {
    return str.len();
}

fn count_str_ref(str: &String) -> usize {
    return str.len();
}
