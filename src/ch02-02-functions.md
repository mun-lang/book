## Functions

Together with `struct`, functions are the core building blocks of hot-loading in Mun. Throughout this documentation you've already seen a lot of examples of functions. You have also seen the `fn` keyword, which allows you to define a function.

Mun uses *snake case* as the conventional style for function and variable names. In snake case all letters are lowercase and words are separated by underscores. 

```mun
fn main() {
    another_function();
}

fn another_function() {

}
```

Function definitions in Mun start with an optional access modifier (`pub`) followed by the `fn` keyword, a name, an argument list enclosed by parentheses, an optional return type specifier and finally a body. 

### Arguments

Functions can have an argument list. Arguments are special variables that are part of the function signature. Unlike regular variables you have to explicitly specify the type of the arguments. This is a deliberate decision in Mun, requiring type annotations in function definitions means that you almost never need to use them elsewhere in your code. It also ensure that you as a developer define a "contract" of what your function can accept as its input.

The following is a rewritten version of `another_function` that shows what an argument looks like:

```mun
fn main() {
    another_function(3);
}

fn another_function(x: int) {
}
```

The declaration of `another_function` specifies an argument `x` of the `int`. When you want a function to have multiple arguments separate them by comma's:

```mun
fn main() {
    another_function(3, 4);
}

fn another_function(x: int, y: int) {
}
```

### Bodies

Function bodies are made up of a sequence of statements and expressions. *Statements* are instructions that perform some action and do not return any value. *Expressions* evaluate to a resulting value. 

Creating a variable and assigning a value to it with the `let` keyword is a statement. In the below example `let y = 6;` is a statement.

```mun
fn main() {
    let y = 6;
}
```

Statements do not return values and can therefor not be assigned to another variable. 

Expressions evaluate to something and make up most of the rest of the code you write in Mun. Consider a simple math operation `5 + 6`, which is an expression that evaluates to `11`. Expressions can be part of a statement as can be seen in the example above. The expression `6` is assigned to the variable `y`. Calling a function is also an expression.

### Return values

Functions can also return a value that is returned to the code that calls them. We don't name return values, but we do declare their type after an arrow (`->`). In Mun, the return value of the function is synonymous with the value of the final expression in the block of the body of a function. You can return early from a function by using the `return` keyword and specifying a value, but most functions return the last expression implicitly. 

```mun
fn five() -> int {
    5
}

fn main() {
    let x = five();
}
```

There are no function calls or statements in the body of the `five` function, just the expression `5`. This is perfectly valid Mun. Note that the return type is specified too, as `-> int`. 

### Implicit & explicit returns

The body of a function is just a block. In Mun, not just bodies, but all blocks evaluate to the last expression in them. Blocks can therefore also be used on the right hand side of `let` statement.

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


### Access modifiers

When you want to be able to use a function outside of the module it is defined in, like in your host language, you define it as public with the `pub` keyword. This exposes the function to the outside world allowing it to be called from outside of the code it was defined in.

```mun
// This function is not accessible outside of this code
fn foo() {
    // ...
}

// This function is accessible from anywhere.
pub fn bar() {
    // Because `bar` and `foo` are in the same file, this call is valid.
    foo()
}
```

When you want to interface from your host language (C++, Rust, etc.) with Mun you can only access `pub` functions. These functions are also hot-reloaded in the runtime when they have been modified, **or** when code they call implicitly or explicitly has been modified.