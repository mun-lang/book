# Mun

# Introduction

## Case Study: Lua @ Abbey Games

Changes in Lua code can have large implications throughout the entire codebase 
and since we cannot oversee the entire codebase at all times runtime errors are 
bound to occur. Runtime errors are nasty beasts because they can pop up after a 
long period of time and after work on the offending piece of code has already 
finished. They are also often detected by someone different from the person who 
worked on the code. This causes great frustration and delay, let alone when the 
runtime error is detected by a user of the software.

Lua amplifies this issue due to its dynamic and flexible nature. *It would be 
great if we could turn some of these runtime errors into compile time errors.* 
That way programmers are notified of errors way before someone else runs into 
them. The risk of causing implicit runtime errors causes programmers to 
distrust their refactoring tools. This in turn reduces the likelihood of 
programmers refactoring their code.

Even though Lua offers immense flexibility, we noticed that certain opinionated 
patterns recur a lot and as such have become standard practice. Introducing 
these practices assists us in daily development a lot, but requires more code 
and complexity than desirable. [*Having syntactic sugar would greatly help 
reduce complexity in our code base*]

Rapid iteration is key to prototyping game concepts and features. *Proper 
IDE-integration of a scripting language gives a huge boost to productivity.*


# Basic Concepts

This chapter describes the basic concepts of the language.

## Values and Types

Mun is a *statically typed language*, which means that the types of all 
variables must be known at compile time. The compiler can usually infer a 
variable's type based on its assigned value and its usage. In cases where 
multiple types are possible, a type annotation must be added.

[Rust](https://doc.rust-lang.org/book/ch03-02-data-types.html) handles it as 
follows:

```rust

let guess: u32 = "42".parse().expect("Not a number!");
```

When type annotation is not provided, Rust will display the following error, 
indicating that the compiler needs more information to determine the variable's 
type:

```rust
error[E0282]: type annotations needed
 --> src/main.rs:2:9
  |
2 |     let guess = "42".parse().expect("Not a number!");
  |         ^^^^^
  |         |
  |         cannot infer type for `_`
  |         consider giving `guess` a type
```

All values in Mun are *first-class citizens*; allowing them to be stored in 
variables, passed as arguments to other functions, and returned as results.

Mun has seven basic types: Booleans, numbers, options, strings, functions, 
userdata, and tables. 

The `boolean` type has two values, **true** and **false**, that are used to 
evaluate conditions.

The `number` type uses two internal representations - or subtypes - called 
*integer* and *float*. The former is used to represent a number without a 
fractional component, whereas the latter represents real (floating-point) 
numbers. Mun has explicit rules about when each representation is used, but 
provides automatic conversions between them at compile time, as long as the 
conversion does not cause undefined behaviour. By default Mun uses 32-bit 
signed integers and single-precision (32-bit) floats, but the user can instruct 
the compiler to use explicit types. If you are unsure which type to use, Mun's 
defaults are generally a good choice, as these types are generally the fastest 
\- even on 64-bit systems.

Table 3-1 shows the built-in integer types in Mun. Each variant can be either 
signed or unsigned and has an explicit size. *Signed* and *unsigned* refer to 
whether it is necessary to have a sign that indicates the possibility for the 
number to be negative or positive.

| Length  | Signed  | Unsigned |
|:-------:|:-------:|:--------:|
| 8-bit   | `i8`    | `u8`     |
| 16-bit  | `i16`   | `u16`    |
| 32-bit  | `i32`   | `u32`    |
| 64-bit  | `i64`   | `u64`    |
| 128-bit | `i128`  | `u128`   |

**Table 2-1: Integer types in Mun**

Signed integer types start with `i`, unsigned integer types with `u`, followed 
by the number of bits that the integer value takes up. Each signed variant can 
store numbers from -(2<sup>n - 1</sup>) to 2<sup>n - 1</sup> - 1 inclusive, 
where *n* is the number of bits that variant uses. Unsigned variants can store 
numbers from 0 to 2<sup>n - 1</sup>.

You can write integer literals in any of the forms shown in Table 2-2. Note 
that all number literals allow a type suffix, such as `57u8`, and `_` as a 
visual separator, such as `1_000`.

| Number literals  | Example       |
|------------------|---------------|
| Decimal          | `98_222`      |
| Hex              | `0xff`        |
| Octal            | `0o77`        |
| Binary           | `0b1111_0000` |

**Table 2-2: Integer literals in Mun**

Mun also has two primitive types for floating-point numbers, represented 
according to the IEEE-754 standard. The `f32` type is a single-precision float, 
and `f64` has double precision.

Mun can call (and manipulate) functions written in Mun and functions written in 
Rust. Both are represented by the `function` type.

The `userdata` type is provided to allow arbitrary Rust data to be stored in 
Mun variables. A userdata value represents a block of raw memory. There are two 
kinds of userdata: *full userdata*, which is an object with a block of memory 
managed by Mun, and *light* userdata, which is simply an mutable Rust pointer 
reference. Userdata has no predefined operations in Mun, except assignment and 
identity test. By using *metatables* the programmer can define operations for 
full userdata values. Userdata values cannot be created or modified in Mun, 
only through the Rust API. This guarantees the integrity of data owned by the 
host program.

The `table` type implements associative arrays, meaning arrays can have any Mun 
value, except **none** and NaN, as indices. Tables can be *heterogeneous*; i.e. 
they can contain values of all types, except *none*. Any key with value *none* 
is not considered part of the table. Conversely, any key that is not part of a 
table has an associated value of *none*.

> *Not a Number* is a special value used to represent undefined or unrepresentable numerical results, such as 0/0.

[discussion] Rust’s defaults are generally good choices, and integer types 
default to `i32`: this type is generally the fastest, even on 64-bit systems. 
For floating-point numbers, the default type is `f64` because on modern CPUs 
it’s roughly the same speed as `f32` but is capable of more precision.
[question] Do we want to follow the same standard, or rather stick with types 
of the same size (i.e. `i32` and `f32` or `i64` and `f64`)?

> ##### Integer Overflow
>
> Let’s say you have a variable of type `u8` that can hold values between 0 and 
> 255. If you try to change the variable to a value outside of that range, such 
> as 256, *integer overflow* will occur. Mun has some interesting rules 
> involving this behavior. When you are compiling in debug mode, Mun includes 
> checks for integer overflow that cause your program to *panic* at runtime if 
> this behavior occurs. Mun uses the term panicking when a program exits with 
> an error.
>
> When you are compiling in release mode with the `--release` flag, Mun does
> *not* include checks for integer overflow that cause panics. Instead, if
> overflow occurs, Mun performs *two’s complement wrapping*. In short, values
> greater than the maximum value the type can hold “wrap around” to the minimum
> of the values the type can hold. In the case of a `u8`, 256 becomes 0, 257
> becomes 1, and so on. The program will not panic, but the variable will have a
> value that probably is not what you were expecting it to have. Relying on
> integer overflow’s wrapping behavior is considered an error.

## Error Handling

## Garbage Collection / Lifetimes

## Coroutines

# The Language

## Lexical Conventions

Mun is a *free-form* language. It ignores spaces (including new lines), except 
as delimiters between names and keywords. *Comments* are also ignored by the 
compiler.

Single-line comments start with two forward slashes (`//`), whereas multi-line 
comments start with a forward slash and asterisk (`/*`), and close with the 
same characters in opposite ordering (`*/`).

```rust
// An example of a single-line comment

/* An example of a multi-line comment
   More information can be added here.
 */
```

## Variables

Variable names are case sensitive. Valid variable names to start with a letter 
or underscore, followed by any number of letters, numbers or underscores. 
Variables must be assigned an initial value.

```rust
    let x = 5;
```

The compiler can usually infer a variable's type based on its assigned value 
and its usage. If the user wants more control over the variable type or in 
cases where multiple types are possible, a type annotation must be added.

```rust
  let x = 5.0;
  let y: number = 5.0;
  let z: f32 = 5.0;
```

By default variables are *immutable*, which means that you cannot change the 
value once it has been bound to a name. To make variables *mutable* add the 
`mut` keyword

```rust
    let mut x = 5;
    x = 6;
```

## Statements

### Blocks

Blocks are a group of several expressions. The syntax of a block is:

```rust
{
    let x = 5;
    let mut y = 3;
    // ...
    y = 4;
}
```

### Chunks

### Assignment

### Control Structures

### For Statement

### Function Calls as Statements

## Expressions

### Arithmetic Operators

### Bitwise Operators

### Coercions and Conversions

### Relational Operators

### Logical Operators

### Concatenation

### Precedence

### Function Calls

### Function Definitions

# Comparison to Lua

 - The type `nil` was removed in favor of *option*. As such the value **nil** 
   was replaced by **none**.
 - Mun interfaces with a Rust API instead of a C API.


#### Integer Types

An integer is a number without a fractional component. 

What values are first-class citizens?

Go | Lua | Python | Rust
:-:|:---:|:------:|:---:
? | All | ? | ?

What basic types are supported?

|Lua  |
|:---:|
| nil |
| boolean |
| number¹ (float ∨ integer) |
| string (encoding-agnostic) |
| function |
| userdata |
| thread |
| table |

¹) Can be either 32-bit or 64-bit depending on a macro setting

²) Size depends on the target architecture
