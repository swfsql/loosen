# Loosen

Declares a derive-like function attr macro which creates another wrap-like function with single tuple-loosened argument, retrieved from the former function.

Upon call, the loosened function propagates the call with the flattened input tuple as arguments to the original function.

## Example

```rust
# extern crate loosen;
# use loosen::loose;
#
# pub struct A;
# pub struct B;
# 
#[loose]
fn fa(a: A, b: B) {}

// normal call
fa(A, B);

// loose call
let args = (A, B);
fa_loose(args);
// ie. instead of two arguments,
// there is only a single tuple argument

// another usage exaple
(0..10)
    .map(|_| (A, B))
    .map(fa_loose)
    .collect::<Vec<_>>();
```

## Note

This is a draft and is my first try on proc-macros.
I suggested this as an rfc before realising a proc macro would suffice: https://github.com/rust-lang/rfcs/issues/2667
