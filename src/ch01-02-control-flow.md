# Control flow

Deciding whether or not to run some code depending on if a condition is true and
deciding to run some code repeatedly while a condition is true are basic
building blocks in most programming languages. The most common constructs that
let you control the flow of execution of Mun code are if expressions and loops.

## `if` expressions

An `if` expression allows you to branch your code depending on conditions.

```mun
fn main() {
    let number = 3;

    if number < 5 {
        number = 4;
    } else {
        number = 6;
    }
}
```

All `if` expressions start with the keyword `if`, which is followed by a
condition. Different from many C-like languages parentheses are not required
around the condition. In this case the condition checks whether or not the variable
`number` has a value less than 5. The code block we want to execute if the
condition is true is placed immediately after the condition inside curly
brackets. Blocks of code associated with the conditions in `if` expressions are
sometimes called *arms*.

Optionally, we can also include an `else` expression, to give the program an
alternative block of code to execute should the condition evalulate to false. If
you don't provide an `else` expression and the condition is false, the program
will just skip the `if` block and move on to the next bit of code.

You can also have multiple conditions by combining `if` and `else` in an `else
if` expression. For example:

```mun
fn main() {
    let number = 6;
    if number > 10 {
        // The number if larger than 10
    } else if number > 8 {
        // The number is larger than 8 but smaller or equal to 10
    } else if number > 2 {
        // The number is larger than 2 but smaller or equal to 8
    } else {
        // The number is smaller than- or equal to 2.
    }
}
```


### Using `if` in a `let` statement

`if` is also an expression, we can use it on the right side of a `let`
statement just like a block:

```mun
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
```

The `number` variable will be bound to a value based on the outcome of the `if`
expression. The reason for this is that blocks of code evaluate to the last
expression in them, and numbers by themselves are also expressions. This means
the values that have the potential to be results from each arm of the `if` must
be the same type. If the types are mismatched the compiler will report an error.
