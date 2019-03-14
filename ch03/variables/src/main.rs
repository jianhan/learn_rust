fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);

    let tup = (500, 6.4, 1);

    let (_, y, _) = tup;

    println!("The value of y is: {}", y);

    let x1: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x1.0;

    let six_point_four = x1.1;

    let one = x1.2;

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}

fn five() -> i32 {
    5
}