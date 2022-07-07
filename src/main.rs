fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let not_zero = 3;

    if not_zero != 0 {
        println!("number was something other than zero");
    }

    check_divisibility(6);

    let_assignment();
}

fn check_divisibility(number: i32) {
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn let_assignment() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of let_assignment is: {number}");
}
