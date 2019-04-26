# Mun

# Introduction

## Case Study: Lua @ Abbey Games

Changes in Lua code can have large implications throughout the entire codebase and since we cannot oversee the entire codebase at all times runtime errors are bound to occur. Runtime errors are nasty beasts because they can pop up after a long period of time and after work on the offending piece of code has already finished. They are also often detected by someone different from the person who worked on the code. This causes great frustration and delay, let alone when the runtime error is detected by a user of the software.

Lua amplifies this issue due to its dynamic and flexible nature. *It would be great if we could turn some of these runtime errors into compile time errors.* That way programmers are notified of errors way before someone else runs into them. The risk of causing implicit runtime errors causes programmers to distrust their refactoring tools. This in turn reduces the likelihood of programmers refactoring their code.

Even though Lua offers immense flexibility, we noticed that certain opinionated patterns recur a lot and as such have become standard practice. Introducing these practices assists us in daily development a lot, but requires more code and complexity than desirable. [*Having syntactic sugar would greatly help reduce complexity in our code base*]

Rapid iteration is key to prototyping game concepts and features. *Proper IDE-integration of a scripting language gives a huge boost to productivity.*


# Basic Concepts

This chapter describes the basic concepts of the language.

## Values and Types

Mun is a *statically typed language*, which means that the types of all variables must be known at compile time. The compiler can usually infer a variable's type based on its assigned value and its usage. In cases where multiple types are possible, a type annotation must be added.

[Rust](https://doc.rust-lang.org/book/ch03-02-data-types.html) handles it as follows:

```rust

let guess: u32 = "42".parse().expect("Not a number!");
```

When type annotation is not provided, Rust will display the following error, indicating that the compiler needs more information to determine the variable's type:

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

### Scalar Types

A *scalar type* represents a single value. Mun has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

#### Integer Types

An integer is a number without a fractional component. Table 3-1 shows the built-in integer types in Mun. Signed integer types start with `i`, unsigned integer types with `u`, followed by the number of bits that the integer value takes up.

| Length  | Signed  | Unsigned |
|:-------:|:-------:|:--------:|
| 8-bit   | `i8`    | `u8`     |
| 16-bit  | `i16`   | `u16`    |
| 32-bit  | `i32`   | `u32`    |
| 64-bit  | `i64`   | `u64`    |
| 128-bit | `i128`  | `u128`   |
| arch    | `isize` | `usize`  |

**Table 2-1: Integer types in Mun**
