/// # How Functions Work - see official document [here](/Users/yukoga/.rustup/toolchains/nightly-x86_64-apple-darwin/share/doc/rust/html/book/second-edition/ch03-03-how-functions-work.html)

fn main() {
    println!("Hello, world!");

    another_function();

    func_add(5, 6);

    my_greeting();

    my_adding();

    println!("The result of function my_five() is : {}", my_five());
    println!("The result of function my_plus_one(3) is : {}", my_plus_one(3));
}

fn another_function() {
    println!("This is another function.");
}


// Function Parameters
fn func_add(x: i32, y: i32) {
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
    println!("x + y = {}", x+y);
}


/// ## Function bodies.  
/// Point :  
/// - What statements and expressions are  
///   - Statements do not return values.  
/// - And how their differences affect the function bodies.  
///   - Function definitions are also statements.  
fn my_greeting() {
    let msg = "Hello, good morning!";
    println!("{}", msg);
}

fn my_adding() {
    let x = 5;

    let y = {
        let x = 3;
        println!("Inside : The value of x is: {}", x);
        x + 1 // expressions: do not include ending semicolons.
    };
    // expression block { ... }, which evaluates to 4.
    // also block { ... } creates new scopes in which variables are able to have same name as it is outside of scope.

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// declare return value type after an arrow.
fn my_five() -> i32 {
    5
}

fn my_plus_one(x: i32) -> i32 {
    x + 1 // return value. make sure not to add semicolon ; at the end of line as it makes expression changes statements.
}
