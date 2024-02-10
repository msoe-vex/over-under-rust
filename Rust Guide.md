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

# Enums & Structs
In Rust, enums & structs work very similarly to other languages at the base level. In addition, Rust allows you to use the `impl` (read as implement) keyword to define methods on structs or enums much like you would define methods in a class. Additionally, enums can have data associated with each value. For example in `pneumatics.rs`:

```
pub enum Pneumatic {
    Connected(AdiDigitalOutput),
    Disconnected(Option<AdiPort>),
}
```

Enums and structs seems pretty similar in Rust, the major difference being that a struct has one or more fields that can all exist and contain data at one time. Enums have a range of possible values that can contain data but can only be one of the values at a time. 

In the above example, a `Pneumatic` can only be `Connected` or `Disconnected`; and it contains different data depending on that. It also does not have any "member data" like a typical class would. Instead it only exists to create a `AdiDigitalOut` object from a port. 

# [Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
Rust does not have a garbage collector but enforces memory safety through a system called Ownership. In Rust, every dynamically allocated value (heap values) is owned by one and only one variable. 

```
let s1 = String::from("hello");
let s2 = s1;
```

In the above example, `s1` initially owns the `String` object. But when `let s2 = s1;` is called, ownership is transferred to `s2`. In fact, `s1` is no long "valid" and Rust will throw a compile error is you try to access it. When a variable goes out of scope, Rust frees all of it's memory that it was pointing to. By having each heap value have one and only one owner, Rust ensures it frees all dynamically allocated memory once and only once. The same thing happens with passing data into methods. Passing a complex object into a method will transfer the ownership of that object into that method, same for returning it. These rules only apply to complex objects, most primitive types just copy themselves everywhere since they are so small.

## References

Rust also has references, which are very similar to pointers, and work very similarly to pointers and references. You can get around transfering ownership by simply passing in a reference to a function, this is called Reference Borrowing. By default, you are not allowed to change something you have a reference to. References are identified by using a `&` before the variable or parameter name.

### Mutable References

If you want to be able to change something that you have a reference to, you must get a mutable reference to it. This is done by putting `&mut` before the variable or parameter name. This is also sometimes called a mutable borrow. In order to ensure memory safety, Rust only allows you to have a mutable reference to something if there are no other references of any kind. Only one thing is allowed access to an object if it can change it. 
