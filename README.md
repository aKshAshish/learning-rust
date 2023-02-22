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

> Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

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

> Rust has two primitive compound types: *tuples* and *arrays*.

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
```rust
fn say_hello() {
    println!("Hello World");
}

fn square(n: u32) -> u32 {
    n*n
}

// Every block returns the value of the last expresssion
// in this case it returns square of n
```

## Statements and Expressions
Rust is an expression-based language, this is an important distinction. Other languages don’t have the same distinctions. Function bodies are made up of a series of statements optionally ending in an expression. So let’s look at what statements and expressions are and how their differences affect the bodies of functions. <br>

> **Statements** are instructions that perform some action and do not return a value.
```rust
fn main() {
    let y = 6; // This is a statement, while 6 in itself is a expression with value as 6
}
```
- Statements do not return values so one cannot assign a statement. *Function definition* is also a statement.<br>

> **Expressions** evaluate to a resultant value.
```rust
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
```
- Consider x + 1, which is an expression that evaluates to the value 4.
- Expressions can be part of statements, the 3 in the statement `let x = 3;` is an expression that evaluates to the value 3.
- Calling a function is an expression.
- Calling a macro is an expression.
- A new scope block created with curly brackets is an expression. This expression: 
    ```rust
    {
        let x = 3;
        x + 1
    }
    ```
    is a block that, in this case, evaluates to 4. That value gets bound to y as part of the let statement. Note that the x + 1 line doesn’t have a semicolon at the end. **Expressions do not include ending semicolons.** If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.

## Control And Flow

**if-else**

- `if` expression starts with `if` followed by a condition. The condition should evaluate to a `bool` value.

```rust
fn main() {
    let x = 3;
    if x % 2 == 0 {
        println!("x is even");
    } else {
        println!("x is odd");
    }

    // if x { // this will result in error as x is not boolean
    //     println!("Hi")
    // }

    // Using if as an expression
    let condition = true;
    let number = if conditiion { 5 } else { 6 }; // Return type of if and else should be same as rust compiler will need to know the type of value of number. If they are different type can only be determined at runtime which rust compiler does not allow for type safety.
}
```
### **Loops**

Rust has 3 types of loops: `loop`, `while` and `for`.<br>

**loop**
> To repeat the code until exited explicitly using `break` expression.
```rust
fn main() {
    loop {
        println!("again!");
    } // A never ending loop
}

fn main() {
    let mut count = 1;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2; // if a value is added after a break expression,
            // it will be returned as the value of expression and the loop will stop
        }
    };
    println!("Result of the loop {result}");
}

fn main() {
    let mut count = 1;
    let num = 2;
    'labelled_loop: loop {
        let value = num * count;
        println!("{value}")
        if count == 10 {
            break 'labelled_loop;
        }
        count += 1;
    }
}
```
- Loop label start with `'`(single quote).

**while**
> The conditional loops.