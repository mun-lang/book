# Types

Mun is a statically typed language, which helps to detect type-related errors at
compile-time. A type error is an invalid operation on a given type, such as an
integer divided by a string, trying to access a field that doesn't exist, or
calling a function with the wrong number of arguments.

Some languages require a programmer to explicitly annotate syntactic constructs
with type information:

```c++
int foo = 3 + 4;
```

However, often variable types can be inferred by their usage, as illustrated
above. Mun uses type inferencing to determine variable types at compile time.
You are still forced to explicitly annotate variables in a few locations to ensure
a contract between interdependent code.

```mun
fn bar(a:int) {
    let foo = 3 + a
}
```

Here, the parameter `a` must be annotated because it solidifies the signature of
the function. The type of `foo` can be inferred through its usage.

## Numeric types

Mun knows two basic numeric types: `float` and `int`. A `float` is a
double-precision IEEE 64-bit floating point number and an `int` represents a
64-bit integral number.

In Mun an `int` can be explicitly cast to `float`, but the reverse is not true.
Assigning a `float` to an `int` might cause loss of precision and can therefore
not be cast implicitly.
