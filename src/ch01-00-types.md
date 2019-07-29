# Types

Mun is a statically typed languages which helps to detect type-related errors in
a program at compile-time. A type error is an invalid operation on a given type
such as a division by a string, trying to access a field that doesn't exist or
calling a function with the wrong number of arguments.

Some languages require a programmer to explicitly annotate syntactic constructs
with type information:

```c++
int foo = 3+4;
```

Often the types of variables can be inferred by their usage just as in the
example above. Mun uses type inferencing to determine the type of a variable at
compile time. However, it does force you to explicitly annotate variables in a
few locations to ensure a contract between interdependent code.

```mun
function bar(a:int) {
    let foo = 3+a
}
```

Here, the parameter `a` must be annotated because it solidifies the signature of
the function. However, the type of `foo` can be inferred through it's usage.