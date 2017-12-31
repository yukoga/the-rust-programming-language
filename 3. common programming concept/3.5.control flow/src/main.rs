// use std::string::String;

fn main() {
    my_first_if(3);
    my_first_if(6);

    multi_else_if(8);
    multi_else_if(9);

    println!("Returned value is : {}", using_if_in_a_let(3));
    println!("Returned value is : {}", using_if_in_a_let(6));

    loop_with_loop("Hello, hikachan".to_string());
    loop_with_while(5);

    loop_array_with_while();
    loop_array_with_for();

    loop_number_reverse(8);
}


fn my_first_if(number: i32) {
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}


fn multi_else_if(number: i32) {
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

fn using_if_in_a_let(number: i32) -> i32 {
    let msg = if number <= 5 {
        5
    } else {
        number
    };
    msg
}

fn loop_with_loop(msg: String) {
    let mut k = 0;
    loop {
        println!("{}", msg);
        if k == 10 {
            break;
        }
        k += 1;
    }
}

fn loop_with_while(number: i32) {
    let mut number = number;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");
}

fn loop_array_with_while() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }
}

fn loop_array_with_for() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn loop_number_reverse(v: i32) {
    for number in (1..v).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

