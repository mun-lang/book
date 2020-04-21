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

### Basic types

Mun knows two basic numeric types: `float` and `int`. A `float` is a
double-precision IEEE 64-bit floating point number and an `int` represents a
64-bit integral number.

In Mun an `int` can be explicitly cast to `float`, but the reverse is not true.
Assigning a `float` to an `int` might cause loss of precision and can therefore
not be cast implicitly.

A `bool` has two possible values: `true` and `false`.

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

> WIP
