# Overview
## Variables
- In rust variables are immutable by default. Variables are decalared using `let` keyword. 
- To make variables mutable we need to explicitly let the compiler know using `mut`.
```rust
let a = 5;
let b = "Hello World";
let mut c = 24;
c = 24 + 5;
```
- Rust allows variable shadowing.
```rust
let a = 9;
let a = 15;
{
    let a = "abc";
    println!(a) // prints abc
}
println!(a) // prints 15
```
- Shadowing is different from mutable variables as you can change the type of variable when shadowing, which is not possible for mutable varaiables.

## Constants
- Constants are declared using `const` keyword. Use capital letters with `_` to name the constants.
- Constants are immutable.
```rust
const PI = 3.14;
PI = 3.142; // will throw error
```
