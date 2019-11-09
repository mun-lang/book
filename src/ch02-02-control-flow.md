# Control flow

Executing or repeating a block of code only under specific conditions are common
constructs that allow developers to control the flow of execution. Mun provides
 `if...else` expressions and loops.

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

All `if` expressions start with the keyword `if`, followed by a condition.
As opposed to many C-like languages, Mun omits parentheses around the
condition. Only when the condition is true - in the example, whether the `number`
variable is less than 5 - the consecutive code block (or *arm*) is executed.

Optionally, an `else` expression can be added that will be executed when the
condition evaluates to false. You can also have multiple conditions by combining
`if` and `else` in an `else if` expression. For example:

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

The `if` expression can be used on the right side of a `let` statement
just like a block:

```mun
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
```

Depending on the condition, the `number` variable will be bound to the value
of the `if` block or the `else` block. This means that both the `if` and `else`
arms need to evaluate to the same type.
If the types are mismatched the compiler will report an error.
