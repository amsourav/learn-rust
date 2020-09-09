/// This is a Doc comment outside the function
/// Generate docs for the following item.
/// This shows my code outside a module or a function
fn main() {
    
    //! This a doc comment that is inside the function   
    //*! This comment shows my code inside a module or a function  
    println!("Hello World!");
    println!("Hello {}", "World");
    //*! Generate docs for the enclosing item
    // Multiple Placeholder
    println!("Hello {}, {}", "World", "Sourav");
    // Positional Arguments
    println!("Hello {0}, {1}", "World", "Sourav");
    println!("Hello {0}, {0}", "World");
    
    println!("{1} Hello {0}, {0}", "World", "I can go anywhere,");
    // Named Arguments
    println!("{first}, Hello World", first="lolz");
    // Placeholder **Traits**
    println!("Number : 10 \n\tBinary:{:b}\n\tHexadecimal:{:x}\n\tOctal:{:o}", 10, 10, 10);
    // do math
    println!("{} + {} = {}",10, 10, 10 + 10);
    // Debug traits
    println!("{:?}", ("hello", 10));


    print!("Hello;"); // prints to console
    println!("Hello World"); // print to console and appends new line
    eprint!("Bye world"); // prints as error
    eprintln!("Bye world"); // prints as error and appends new line

    let first_name = "Sourav";
    let mut last_name = "WOrld";
    println!("Hey, {} {}", first_name, last_name);
    last_name = "S";
    println!("Hey, {} {}", first_name, last_name);
    
    let (course,category) =("Rust","beginner"); // assign multiple values
    println!("This is a {} course in {}.", category, course); // print the value
}