# Purpose
The purpose of this document is to serve as a aid for moderately experience programmers in their beginning journey with Rust. This should be contributed to by everyone. If you learn something new that is not on here, write it down. In fact, you should write down every new thing that you learn. Science says that you retain information that you write down 95% better!

## Authors 
Note: Anyone can contribute to this document, this is meant to aid everyone. If you contribute, put your name here.

Dylan Powers

# General
`println!()` and `print()` are the print functions in Rust. The `!` means that it is a macro, more on those later. 

# Variables
`let` declares an immutable variable. All variables are immutable by default in Rust and Rust often implies the type, meaning that type specifiers are not always required when the type is obvious.

`let mut` allows the variable to be mutable.

`const` declares a constant. Constants should be formatted AS_SUCH and must be constant expressions. In other words, able to be determined at compile time AND have a type specifier.

`str` is the string type in Rust. When working with strings, you should lookup `.parse()` and all of it's variations. String literals are also considered of type `&str` which is the memory address of a string. 

# Control Flow
If statements are considered expressions so the following code is legal.
```
let answer = if (10 == 10) {
  10
} else {
  0
}
```
But all parts of the if statement must have the same return type. Notice also how there are no semicolons in the above code chunk? That is because it is considered a single statement. Rust requires semicolons after every statement.
