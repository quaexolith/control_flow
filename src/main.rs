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

    loop_break_with_counter();

    loop_with_label();

    loop_while();

    loop_through_array_with_for();

    loop_through_range_with_for();
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

fn loop_break_with_counter() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn loop_with_label() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn loop_while() {
    println!("Loop while");

    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!!")
}

fn loop_through_array_with_for() {
    println!("Loop through array with for");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("{element}");
    }
}

fn loop_through_range_with_for() {
    println!("Loop through range with for");

    for number in (1..4).rev() {
        println!("{number}");
    }

    println!("LIFTOFF!!!");
}
