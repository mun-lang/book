# Introduction

> Note: Mun & this book are currently under active development, any and all
> content of this book not final and can still change.

*Mun* is an embedable scripting language designed for developer productivity. 

* **Ahead of time compiled**
  Mun is ahead of time (AOT) compiled instead of interpreted or JITed. This
  enables catching a lot of errors at compile time instead of at runtime. AOT
  compilation allows developers to stay in their IDE instead of having to debug
  the target application to check for runtime errors. 

* **Statically typed**
  Mun uses types that are resolved at compilation time instead of at runtime.
  This enables powerful refactoring tools and better error detection at the time
  of writing the code vs running the code.

* **First class hot-reloading**
  Mun is written from the ground up with hot-reloading in mind. Hot-reloading
  enables developers to alter the code while its executing. There no longer is a
  need to start/stop the application, changing functions, values or classes take
  effect instantly without restarting the target application. 

* **Performance** 
  Mun compiles the code to optimized machine code using LLVM. Together with
  static typing this enables native performance. Hot-reloading is enabled by
  default which adds a little bit of overhead at runtime, however, for
  production builds hot-reloading can also be turned off which removes all
  overhead.

* **Cross compilation**
  The Mun compiler is able to compile to all supported target platforms from any
  supported compiler platform. 

* **Powerful IDE integration**
  The Mun language and compiler framework are designed from the ground up to
  support queries on the source code which enable powerful IDE integration like
  code completion and refactoring tools.