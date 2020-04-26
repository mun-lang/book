## Values and types

Mun is a statically typed language, which helps to detect type-related errors at
compile-time. A type error is an invalid operation on a given type, such as an
integer divided by a string, trying to access a field that doesn't exist, or
calling a function with the wrong number of arguments.

Some languages require a programmer to explicitly annotate syntactic constructs
with type information:

```c++
int foo = 3 + 4;
```

However, often variable types can be inferred by their usage. Mun uses type
inferencing to determine variable types at compile time. However, you are still
forced to explicitly annotate variables in a few locations to ensure a contract
between interdependent code.

```mun
fn bar(a: int) -> int {
    let foo = 3 + a;
    foo
}
```

Here, the parameter `a` and the return type must be annotated because it solidifies
the signature of the function. The type of `foo` can be inferred through its
usage.

> NOTE: Although the above works, as of version 0.2, Mun is not yet very good at
>type inferencing. This will be improved in the future.

### Basic types

Mun understands the following built-in basic value types:

| Type                      | Types
|---------------------------|---------------------------------------------------|
| Signed integer types      | `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `int` |
| Unsigned integer types    | `u8`, `u16`, `u32`, `u64`, `u128`, `usize`, `uint`|
| IEEE floating point types | `f32`, `f64`, `float`                             |
| Boolean type              | `bool` |

Most types are defined by their bitness, so `i8` is an unsigned integer, 8 bits
in size. There are some exceptions:

* `usize` and `isize` are the size as of the pointer architecture, so 64 bits on
  64 bit platforms and 32 bits on 32 bit plaforms.
* `int`, `uint` and `float` are convenience values that are always 64 bits in
  size. These types are *not* aliases for their sized counterparts so `i64 !=
  int` and therefor implicit casting is not possible.
  

### Literals

There are three types of literals in Mun: integer-, floating-point and boolean
literals. 

A boolean literal is either `true` or `false`.

An integer literal is a number value without a decimal separator (`.`). It can be
written as an decimal, hexadecimal, octal or binary value. These are all
examples of valid literals:

```mun
let a: int = 367;
let b: int = 0xbeaf;
let c: int = 0o76532;
let d: int = 0b0101011;
```

A floating point literal comes in two forms:

* A decimal number followed by a decimal which is optionally followed by another
  decimal literal and an *optional* exponent.
* A decimal number followed by an exponent.

Examples of valid floating point literals are:

```mun
let a: float = 3.1415;
let b: float = 3.;
let c: float = 314.1592654e-2;
```

#### Separators

Both integer and floating-point literals can contain underscores (`_`) to
visually separate numbers from one another. They do not have any semantic
significance but can be useful to the eye.

```mun
let a: int = 1_000_000;
let b: float = 1_000.12;
```

#### Type suffix

Integer and floating-point literals may be followed by a type suffix to
explicitly specify the type of the literal.

For an integer literal valid suffixes are:

| Literal type | Suffixes |
|--------------|----------|
|Integer |`u8`, `i8`, `u16`, `i16`, `u32`, `i32`, `u64`, `i64`, `i128`, `u128`, `int`, `uint`, `usize`, `isize`, `f32`, `f64`, `float` |
| Floating-point | `f32`, `f64`, `float` |

Note that integer literals can have floating point suffixes. This is not the
case the other way around.

> Note: number literal types in Mun v0.2 are always either `int` or `float`
> unless a suffix is specified.

```mun
let a: u8 = 128_u8;
let b: i128 = 99999999999999999_i128;
let c: f32 = 10_f32; // integer literal with float suffix 
```

When providing a literal, the compiler will always check if a literal value will
fit the type. If the literal won't fit in the type an error will be emitted:

```mun
let a: u8 = 1123123124124_u8; // literal out of range for `u8`
```

### Numeric operations 

Mun supports the basic mathematical operations you'd expect for all number types: addition, subtraction, division, multiplication, and remainder. 

```mun
fn main() {
    // addition 
    let a = 10 + 5;

    // subtraction
    let b = 10 - 4;

    // multiplication
    let c = 5 * 10;

    // division
    let d = 25 / 5;

    // remainder
    let e = 21 % 5;
}
```

Each expression in these statements uses a mathematical operator and evaluates to a single value. This is valid as long as both sides of the operator are of the same type.

Unary operators are also supported:

```mun
fn main() {
    let a = 4;
    let b = -a;
}
```


### Implicit & explicit returns

A block is a grouping of statements and expressions surrounded by curly braces.
Function bodies are an example of blocks. In Mun, blocks evaluate to the last
expression in them. Blocks can therefore also be used on the right hand side of a
`let` statement.

```mun
fn foo() -> int {
    let bar = {
        let b = 3;
        b + 3
    };

    // `bar` has a value 6
    bar + 3
}
```

Besides implicit returns, explicit returns can also be used with `return`
expressions. However, explicit `return` statements always return from the
function, not from the block:

```mun
fn foo() -> int {
    let bar = {
        let b = 3;
        return b + 3;
    };

    // This code will never be executed
    return bar + 3;
}
```

### Shadowing

Redeclaring a variable by the same name with a `let` statement is valid and will
shadow any previous declaration in the same block. This is often useful if you
want to change the type of a variable.

```mun
let a: int = 3;
let a: float = 5.0; 
```

### Use before initialization

All variables in Mun must be initialized before use. Uninitialized variables can
be declared but they must be assigned a value before they can be read.

```mun
let a: int;
if some_conditional {
    a = 4;
}
let b = a; // invalid: a is potentially uninitialized
```

Note that declaring a variable without a value is often a code smell since the
above could have better been written by *returning* a value from the `if`/`else`
block instead of assigning to `a`. This wont require the use of an uninitialized
value.

```mun
let a: int = if some_conditional {
    4
} else {
    5
}
let b = a;
```
