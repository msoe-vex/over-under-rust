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

A strange but useful method of variable initialization is with the below structure:
```
let y = {
  let x = 3;
  x + 1
};
```
Using `{}` anywhere creates a closure that can act as an expression externally. Notice how the final statement in the closure does not have a semi-colon. In closures or functions, if the final statement does not have a semi-colon, Rust treats the closure or function as an expression externally.

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

# Functions
Below is a valid Rust function:
```
fn sale_price(price: i32) -> i32{
  if is_even(price) {
    price - 10
  } else {
    price - 3
  }
}
```
All parameters in Rust functions must include a type annotation. Functions do not *always* need return type annotations. However some scenarios do require them. Either way, it is good practice to include them wherever possible. Given previous knowledge, you should be able to notice that without semi-colons, Rust will treat the entire if-else block, and by extension, the function, as an expression.
