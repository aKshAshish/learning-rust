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
- Rust allows **variable shadowing**.
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
## Data Types
Every value in rust is of a certain type. Rust is a *statically typed* language which means that it must know the types of all variables at compile time.<br>
Rust has two types of data **Scalar** and **Compound**.
### Scalar (or single value data types)
- Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.
```rust
// Integers
let a: u32 = 64; // Unsigned integer 32bit
let a: i64 = 124_000; // Signed integer 64 bit, can use _ for readability
let a = 34u8; // by adding u8 in integer literal we let the compiler know we want to use 8 bit integer

//Floats
let f = 32.4; // float 64 bit
let f: f32 = 43.5; // float 32 bit

// Characters
let b = 'c'; // Unicode character
let b = '😀'; // Valid character

//Booleans
let flag: bool = true;
let not_flag: bool = false;
```
|Number Literals|Examples|
|---------------|--------|
|Decimal        | 123_345|
|Hex            | 0xff   |
|Octal          | 0o77   |
|Binary         | 0b1010 |
|Byte (u8 only) | b'A'   |

### Compound (or multiple value data types)
- Rust has two primitive compound types: *tuples* and *arrays*.

**Tuples**
- A tuple is a general way of grouping together a number of values with a *variety of types* into one compound type.
- Tuples have a fixed length: once declared, they cannot grow or shrink in size.
- The tuple without any values has a special name, unit. This value and its corresponding type are both written `()` and represent an empty value or an empty return type. *Expressions* implicitly return the unit value if they don’t return any other value.
```rust
let tup: (u32, f64, u8) = (12, 34.5, 1);
let (x, y, z) = tup; // Destructuring
println!("Value of y is {y}");
let twelve = tup.0; // indexing
let thirty_point_five = tup.1;
let one = tup.2;
```
**Arrays**
- Every element of an array must have the same type.
- Arrays are of fixed length.
- Rust throws runtime error if an element is being indexed out of bound of an array.
```rust
let a = [1, 2, 3, 4, 5];
let a: [i32; 5] = [1, 2, 3, 4, 5]; // i32 is type and 5 is the length of array
let a = [3; 5] // is same as writing as let a = [3, 3, 3, 3, 3];
let first = a[0];
let second = a[1];
```
## Functions