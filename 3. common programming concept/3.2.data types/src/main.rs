#![crate_name = "data_types"]

fn main() {
    /// # Built-in Data Types
    /// Rust has two data type categories - Scalar types and Compound types.  
    /// ## Scalar types.  
    /// - Integer types - i8/u8, i16/u16, i32/u32, i64/u64, isize/usize  
    ///   iN means signed and uN means unsigned interger.  
    /// - Floating-point types - f32 and f64.  
    /// - Boolean type - true and false  
    /// - Character type - char type specified with single quotes, whereas doulbe quotes is used for string.  

    let guess: u32 = "42".parse().expect("Not a number.");
    println!("The value of guess is : {}", guess);

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("The value of c : {}", c);
    println!("The value of z : {}", z);
    println!("The value of heart_eyed_cat : {}", heart_eyed_cat);


    /// ## Compound types  
    /// Compound types can group multiple values of other types into one type. Rust has two primitive compound types: tuples and arrays.  

    /// ### Grouping Values into Tuples  
    /// A tuple is a general way of grouping together some number of other values with a variety of types into one compound type.

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x : {}", x);
    println!("The value of y : {}", y);
    println!("The value of z : {}", z);

    println!("The 1st value of tup : {}", tup.0);
    println!("The 2nd value of tup : {}", tup.1);
    println!("The 3rd value of tup : {}", tup.2);

    /// ### Arrays  
    /// Another way to have a collection of multiple values is with an array. Unlike a tuple, every element of an array must have the same type.

    let arr = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    println!("The first element of arr is {}", arr[0]);
    println!("The 2nd element of months is {}", months[1]);
}
