// This is the main function.
fn main() {
    // Statements here are executed when the compiled binary is called.

    // Print text to the console.
    // println! is a macro that prints text to console
    println!("Hello World!");
    println!("I'm a Rustacean!");

    // a binary can be generated using the Rust compiler: "rustc"

    // this is a line comments which go to the end of the line
    /*
        block comments which go to the closing delimiter.
    */

    // This is an example of a line comment.
    // There are two slashes at the beginning of the line.
    // And nothing written after these will be read by the compiler.

    // println!("Hello, world!");

    // Run it. See? Now try deleting the two slashes, and run it again.

    /*
        * This is another type of comment, a block comment. In general,
        * line comments are the recommended comment style. But block comments
        * are extremely useful for temporarily disabling chunks of code.
        * /* Block comments can be /* nested, */ */ so it takes only a few
        * keystrokes to comment out everything in this main() function.
        * /*/*/* Try it yourself! */*/*/
        */

    /*
    Note: The previous column of `*` was entirely for style. There's
    no actual need for it.
    */

    // You can manipulate expressions more easily with block comments
    // than with line comments. Try deleting the comment delimiters
    // to change the result:
    let x = 5 + /* 90 + */ 5;
    println!("Is `x` 10 or 100? x = {}", x);


    struct MyStruct {
        name: String,
        surname: String,
        age: u32,
    }

    let my_struct;
    my_struct =  MyStruct {
        name: String::from("Emre"),
        surname: String::from("Ergün"),
        age: 34
    };


    println!("I am {} {}, and {} years old.",
        my_struct.name,
        my_struct.surname,
        my_struct.age);


    //formatted print
    // format! : write formatted text to String
    // print! : same as format! but the text is printed to the console io::stdout
    // println! : same as print! but a new line is appended.
    // eprint! : same as print! but the text is printed out to the standard error io::stderr
    // eprintln! : same as eprint! but the new line is appended.

    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Positional arguments can be used. Specifying an integer inside `{}`
    // determines which additional argument will be replaced. Arguments start
    // at 0 immediately after the format string.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Different formatting can be invoked by specifying the format character
    // after a `:`.
    println!("Base 10:               {}",   69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
    println!("Base 16 (hexadecimal): {:X}", 69420); // 10F2C


    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number:>5}", number=1);

    // You can pad numbers with extra zeroes,
    // and left-adjust by flipping the sign. This will output "10000".
    println!("{number:0<5}", number=1);

    // You can use named arguments in the format specifier by appending a `$`.
    println!("{number:0>width$}", number=1, width=5);


    // Rust even checks to make sure the correct number of arguments are used.
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types do not implement fmt::Display by default.

    // this is an attribute for compiler
    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);

    // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    // "    1", 4 white spaces and a "1".
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    let pi = 3.141592;
    println!("Pi is roughly {:>.width$}", pi, width = 3);



}