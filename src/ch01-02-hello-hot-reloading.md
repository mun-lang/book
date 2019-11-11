# Hello, Hot Reloading!

Mun distinguishes itself from other languages by its inherent hot reloading capabilities. The
following example illustrates how you can create a hot reloadable application by extending the
[Hello, fibonacci?](ch01-01-hello-fibonacci.md) example.

Filename: hello_fibonacci.mun

```mun
fn fibonacci5():int {
    fibonacci(5)
}

fn fibonacci(n:int):int {
    if n <= 1 {
        n
    } else {
        fibonacci(n-1) + fibonacci(n-2)
    }
}
```

<span class="caption">Listing 1-2: A function that calculates a fibonacci number</span>

Apart from running Mun libraries from the command-line interface, a common use case is
embedding them in other programming languages.

## Mun embedded in C++

Mun [exposes](https://github.com/mun-lang/runtime-ffi) a C API and complementary C++ bindings for
the Mun Runtime. Listing 1-3 shows a C++ application that constructs a Mun Runtime for the
`hello_fibonacci` library and continuously invokes the `fibonacci5` function and outputs its result.

Filename: main.cc

```cpp
#include <iostream>

#include "mun/runtime.h"

int main() {
    if (argc < 2) {
        return 1;
    }

    auto lib_path = argv[1];
    if (auto runtime = mun::make_runtime(lib_path)) {
        while (true) {
            auto result = mun::invoke_fn<int64_t>(*runtime, "fibonacci5").wait();
            std::cout << "fibonacci5() = " << result << std::endl;

            runtime->update();
        }
    }

    return 2;
}
```

<span class="caption">Listing 1-3: Hello, Fibonacci? embedded in a C++ application</span>

## Mun embedded in Rust

As the Mun Runtime is written in Rust, it can be easily embedded in Rust applications by adding the
`mun_runtime` crate as a dependency. Listing 1-4 illustrates a simple Rust application that builds
a Mun Runtime and continuously invokes the `fibonacci5` function and prints its output.

Filename: main.rs

```rust
use mun_runtime::{invoke_fn, RetryResultExt, RuntimeBuilder};
use std::env;

fn main() {
    let lib_path = env::args().nth(1).expect("Expected path to a Mun library.");

    let mut runtime = RuntimeBuilder::new(lib_path)
        .spawn()
        .expect("Failed to spawn Runtime");

    loop {
        let result: i64 = invoke_fn!(runtime, "fibonacci5").wait();
        println!("fibonacci5() = {}", result);
        runtime.update();
    }
}
```

<span class="caption">Listing 1-4: Hello, Fibonacci? embedded in a Rust application</span>

## Hot Reloading

The prior examples both update the runtime every loop cycle. In the background, this detects
recompiled code and reloads the resulting shared libraries.

To ensure that the Mun compiler recompiles our code every time the *hello_fibonacci.mun* source
file from Listing 1-2 changes, the `--watch` argument must be added:

```
mun build hello_fibonacci.mun --watch
```

When saved, changes in the source file will automatically take affect in the running example
application. E.g. change the `n` value passed to the `fibonacci` function and the application will
log the corresponding Fibonacci number.

Some changes, such as a type mismatch between the compiled application and the hot reloadable library, can lead to runtime errors. When these occur, the runtime will log the error and halt until an update to the source code arrives.

That's it! Now you are ready to start developing hot reloadable Mun libraries.
