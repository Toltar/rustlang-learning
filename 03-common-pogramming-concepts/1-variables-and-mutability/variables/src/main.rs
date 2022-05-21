fn main() {
    // --- Variables and Mutability ---
    //let x: i32 = 5 would cause compilation errors since Rust variables are immutable by default
    //which is why we need the `mut` keyword after `let`
    let mut x = 5;
    println!("x is {}", x);
    x = 6;
    println!("x is {}", x);


    // --- Constants ---
    // Constants are not allowed to use the `mut` keyword.

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is {}", THREE_HOURS_IN_SECONDS);


    // --- Shadowing ---
    // Allows you to use the same variable name for multiple variable declarations
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x in the outer scope is: {}", x);

    // We are allowed to mutate a variable into a different type
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Space is {}", spaces);

    // However if we add a mut to spaces it will cause an error because we are mutating the variables type
    // let mut spaces = "   ";
    // spaces = spaces.len();
}
