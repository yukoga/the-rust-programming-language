fn main() {
    
    /// ## 3.1. Variables and mutability  
    /// This is sample program written in Rust, which contains following topics.  
    /// - default mutability (i.e. Rust variable is immutable as default, and you can change it with "mut".  
    /// - Differences between variables and constants.  
    /// - Shadowing  
    

    /// ### Mutability
    let mut x = 5;
    println!("The value of x is : {}", x);
    x = 6;
    println!("The value of x is : {}", x);

    /// ### Variables and Constants  
    /// - Not allowed to use "mut" with constants.  
    /// - Constants is always immutable.  
    /// - Declare constants using the "const" keyword instead of "let" as for variables.  
    /// - Type of value must be annotated.  
    /// - Constants will be declared in any scope.  
    /// - Constants may only be set to a constant expression, not the result of function call and/or any computations.  


    // Rust constant naming convention is to use all upper case with underscores between words. 
    const MAX_POINTS: u32 = 100_000;


    /// ### Shadowing  
    /// We can declare a new variable with the same name as a previous variable.  
    /// And new variable "shadows" the previous variable.  
    /// The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value, but reuse the same name.  

    let y = 5;
    println!("The value of y is : {}", y);
    let y = y + 1;
    println!("The value of y is : {}", y);
    let y = y * 2;
    println!("The value of y is : {}", y);
}
