## `extern` functions

Extern functions are functions defined in Mun without an implementation. They behave exactly the same as regular functions but they have to be provided to the runtime when loading a Mun library. Failure to do so will result in a runtime link error, and loading the library will not work.

Take this code for example:

```mun
{{#include ../listings/ch02-basic-concepts/random.mun}}
```

The `random` function is marked as an `extern` function which means it must be provide to the runtime when loading this library.

First building the above code as `main.munlib` and then trying to load the library in Rust using:

```rust
{{#include ../listings/ch02-basic-concepts/random_missing.rs}}
```

will result in an error:

```
Failed to link: function `random` is missing.
```

This indicates that we have to provide the runtime with the `random` method, which we can do through the use of the `insert_fn` method. Lets add a method that uses the current time as the base of our `random` method:

```rust
{{#include ../listings/ch02-basic-concepts/random.rs}}
```

Note that we have to explicitly cast the function `random` to `extern "C" fn() -> i64`. This is because each function in Rust has its own unique type.

Now if we run this, the error is gone and you should have a function that returns a random boolean written in Mun.