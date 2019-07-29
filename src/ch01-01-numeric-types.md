# Numeric types

Mun knows two basic numeric types: `float` and `int`. A `float` is a
double-precision IEEE 64-bit floating point number and an `int` represents a
64-bit integral number.

In Mun an `int` can be explicitly casted to `float` but the reverse is not true:
assigning a `float` to an `int` might cause loss of precision and is therefor
not allowed implicitly.